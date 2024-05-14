from imutils import paths
import os
import argparse
import cv2
import numpy as np
import imutils
import time

def f_o_img_stitched(o_img1, o_img2):

    def is_translation_only(homography, threshold=0.05):
        # Check if the homography matrix is close to an identity matrix with translation
        identity = np.array([[1, 0, 0], [0, 1, 0], [0, 0, 1]], dtype=float)
        translation_only = np.copy(homography)
        translation_only[0, 2] = 0  # Ignore x translation
        translation_only[1, 2] = 0  # Ignore y translation
        translation_only[2, :] = [0, 0, 1]  # Ignore the last row

        return np.allclose(translation_only, identity, atol=threshold)

    def create_valid_mask(image):
        # Create a mask where the valid (non-black) regions are 255 and the black regions are 0
        gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
        _, mask = cv2.threshold(gray, 1, 255, cv2.THRESH_BINARY)
        return mask


    if o_img1 is None or o_img2 is None:
        raise ValueError("One or both of the input images could not be read.")

    # Create masks for valid regions (non-black areas)
    mask1 = create_valid_mask(o_img1)
    mask2 = create_valid_mask(o_img2)

    # Create ORB detector with 5000 features.
    orb_detector = cv2.ORB_create(5000)

    # Find keypoints and descriptors in the valid regions only.
    kp1, d1 = orb_detector.detectAndCompute(cv2.cvtColor(o_img1, cv2.COLOR_BGR2GRAY), mask1)
    kp2, d2 = orb_detector.detectAndCompute(cv2.cvtColor(o_img2, cv2.COLOR_BGR2GRAY), mask2)

    # Match features between the two images.
    matcher = cv2.BFMatcher(cv2.NORM_HAMMING, crossCheck=True)

    # Match the two sets of descriptors.
    matches = matcher.match(d1, d2)

    # Sort matches on the basis of their Hamming distance.
    matches = sorted(matches, key=lambda x: x.distance)

    # Take the top 90 % matches forward.
    matches = matches[:int(len(matches) * 0.9)]
    no_of_matches = len(matches)

    # Define empty matrices of shape no_of_matches * 2.
    p1 = np.zeros((no_of_matches, 2))
    p2 = np.zeros((no_of_matches, 2))

    for i in range(len(matches)):
        p1[i, :] = kp1[matches[i].queryIdx].pt
        p2[i, :] = kp2[matches[i].trainIdx].pt

    # Find the homography matrix.
    homography, mask = cv2.findHomography(p1, p2, cv2.RANSAC)

    # Check if the homography matrix represents only translation with slight deviations
    if not is_translation_only(homography, 0.01):
        raise ValueError("Alignment requires significant transformation. Alignment failed.")
        
    # Get the size of the input images
    h1, w1 = o_img1.shape[:2]
    h2, w2 = o_img2.shape[:2]

    # Get the canvas size
    corners_img1 = np.array([[0, 0], [0, h1], [w1, h1], [w1, 0]], dtype='float32').reshape(-1, 1, 2)
    corners_img2 = np.array([[0, 0], [0, h2], [w2, h2], [w2, 0]], dtype='float32').reshape(-1, 1, 2)
    corners_img1_transformed = cv2.perspectiveTransform(corners_img1, homography)
    all_corners = np.concatenate((corners_img2, corners_img1_transformed), axis=0)

    [x_min, y_min] = np.int32(all_corners.min(axis=0).ravel() - 0.5)
    [x_max, y_max] = np.int32(all_corners.max(axis=0).ravel() + 0.5)

    translation_dist = [-x_min, -y_min]
    homography_translation = np.array([[1, 0, translation_dist[0]], [0, 1, translation_dist[1]], [0, 0, 1]])

    # Warp images with the homography matrix
    warped_img1 = cv2.warpPerspective(o_img1, homography_translation.dot(homography), (x_max - x_min, y_max - y_min))

    # Create an empty canvas for the blended image
    blended_img = np.zeros_like(warped_img1)

    # Copy the reference image to the correct location in the blended image
    blended_img[translation_dist[1]:h2 + translation_dist[1], translation_dist[0]:w2 + translation_dist[0]] = o_img2

    # Find the overlapping region
    overlap_region = (blended_img != 0) & (warped_img1 != 0)
    non_overlap_region1 = (blended_img == 0) & (warped_img1 != 0)
    non_overlap_region2 = (blended_img != 0) & (warped_img1 == 0)

    # Average the pixel values in the overlapping region
    blended_img[overlap_region] = (blended_img[overlap_region] / 2 + warped_img1[overlap_region] / 2).astype(np.uint8)

    # Copy the non-overlapping regions
    blended_img[non_overlap_region1] = warped_img1[non_overlap_region1]
    blended_img[non_overlap_region2] = blended_img[non_overlap_region2]

    return blended_img

def main():
    s_name_file_default = str(time.time()) + '_python_image_stitching_result.png'

    ap = argparse.ArgumentParser()
    ap.add_argument("-i", "--images", type=str, required=True,
                    help="path to input directory of images to stitch")
    ap.add_argument("-o", "--output", type=str, required=False, default=s_name_file_default,
                    help="path to the output image")
    ap.add_argument("-c", "--crop", type=int, default=0,
                    help="whether to crop out largest rectangular region")
    args = vars(ap.parse_args())

    # grab the paths to the input images and initialize our images list
    print("[INFO] loading images...")
    imagePaths = sorted(list(paths.list_images(args["images"])))

    if len(imagePaths) < 2:
        print("[ERROR] Need at least two images to perform stitching.")
        return

    # Read the first image
    base_image = cv2.imread(imagePaths[0])
    fails = []


    for imagePath in imagePaths[1:]:
        next_image = cv2.imread(imagePath)

        print(f"[INFO] stitching {imagePath}...")
        try:
            o_img_stitched = f_o_img_stitched(base_image, next_image)
            print("[INFO] stitching successful")
            base_image = o_img_stitched
        except: 
            print(f"[INFO] stitching failed , adding to retry list")
            fails.append(imagePath)

    # Retry failed images
    for imagePath in fails[:]:
        next_image = cv2.imread(imagePath)
        print(f"[INFO] retrying stitching for {imagePath}...")
        try:
            o_img_stitched = f_o_img_stitched(base_image, next_image)
            print("[INFO] retry stitching successful")
            base_image = o_img_stitched
            fails.remove(imagePath)
        except: 
            print(f"[INFO] retry stitching failed ")




	# Save the final stitched image
    cv2.imwrite(args["output"], base_image)
    print(f"[INFO] Final stitched image saved as {args['output']}")

if __name__ == "__main__":
    main()
