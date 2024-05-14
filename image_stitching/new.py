import os
import argparse
import cv2
import numpy as np
from imutils import paths
import time

def compute_homography(image1, image2, scale_percent):
    # Resize images
    image1_small = resize_image(image1, scale_percent)
    image2_small = resize_image(image2, scale_percent)

    # Convert to grayscale
    gray1 = cv2.cvtColor(image1_small, cv2.COLOR_BGR2GRAY)
    gray2 = cv2.cvtColor(image2_small, cv2.COLOR_BGR2GRAY)

    # Detect ORB keypoints and descriptors
    orb = cv2.ORB_create(5000)
    kp1, des1 = orb.detectAndCompute(gray1, None)
    kp2, des2 = orb.detectAndCompute(gray2, None)

    # Match descriptors
    matcher = cv2.BFMatcher(cv2.NORM_HAMMING, crossCheck=True)
    matches = matcher.match(des1, des2)
    matches = sorted(matches, key=lambda x: x.distance)

    # Extract matched keypoints
    pts1 = np.float32([kp1[m.queryIdx].pt for m in matches])
    pts2 = np.float32([kp2[m.trainIdx].pt for m in matches])

    # Compute homography
    homography, mask = cv2.findHomography(pts2, pts1, cv2.RANSAC)
    return homography

def resize_image(image, scale_percent):
    width = int(image.shape[1] * scale_percent / 100)
    height = int(image.shape[0] * scale_percent / 100)
    return cv2.resize(image, (width, height), interpolation=cv2.INTER_AREA)

def main():
    s_name_file_default = str(time.time()) + '_python_image_stitching_result.png'

    ap = argparse.ArgumentParser()
    ap.add_argument("-i", "--images", type=str, required=True,
                    help="path to input directory of images to stitch")
    ap.add_argument("-o", "--output", type=str, required=False, default=s_name_file_default,
                    help="path to the output image")
    ap.add_argument("-c", "--crop", type=int, default=0,
                    help="whether to crop out largest rectangular region")
    ap.add_argument("-s", "--scale", type=int, default=30,
                    help="scale percent for downscaling images")
    args = vars(ap.parse_args())

    # Grab the paths to the input images and initialize our images list
    print("[INFO] loading images...")
    imagePaths = sorted(list(paths.list_images(args["images"])))

    if len(imagePaths) < 2:
        print("[ERROR] Need at least two images to perform stitching.")
        return

    # Read the first image
    base_image = cv2.imread(imagePaths[0])
    h, w = base_image.shape[:2]

    # Initialize the list to store homographies
    homographies = [np.eye(3)]

    # Compute homographies using downscaled images
    for imagePath in imagePaths[1:]:
        next_image = cv2.imread(imagePath)
        homography = compute_homography(base_image, next_image, args["scale"])
        homographies.append(homography)
        base_image = next_image

    # Apply homographies to high-res images
    base_image = cv2.imread(imagePaths[0])
    h, w = base_image.shape[:2]

    # Create a canvas for the final stitched high-res image
    final_canvas = np.zeros((h * 3, w * 3, 3), dtype=base_image.dtype)
    final_canvas[h:h*2, w:w*2] = base_image

    current_homography = np.eye(3)
    translation = np.array([[1, 0, w], [0, 1, h], [0, 0, 1]])

    for imagePath, homography in zip(imagePaths[1:], homographies[1:]):
        next_image = cv2.imread(imagePath)
        current_homography = translation @ current_homography @ homography
        stitched_high_res = cv2.warpPerspective(next_image, current_homography, (final_canvas.shape[1], final_canvas.shape[0]))
        non_black_pixels = stitched_high_res.sum(axis=-1) > 0
        final_canvas[non_black_pixels] = stitched_high_res[non_black_pixels]

    # Save the final high-res stitched image
    cv2.imwrite(args["output"], final_canvas)
    print(f"[INFO] Final stitched image saved as {args['output']}")

if __name__ == "__main__":
    main()
