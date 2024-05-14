import cv2
import numpy as np
import argparse
import imutils
from imutils import paths
import inspect

ap = argparse.ArgumentParser()
ap.add_argument("-i", "--images", type=str, required=True,
                help="path to input directory of images to stitch")
ap.add_argument("-o", "--output", type=str, required=False, default="stitching_out.png",
                help="path to the output image")
args = vars(ap.parse_args())

# Grab the paths to the input images and initialize our images list
print("[INFO] loading images...")
imagePaths = sorted(list(paths.list_images(args["images"])))

images = []
for imagePath in imagePaths:
	image = cv2.imread(imagePath)
	images.append(image)

# Create stitcher object
stitcher = cv2.Stitcher_create(cv2.Stitcher_SCANS)

# Perform stitching
# status, stitched_image = stitcher.stitch(images)
print(stitcher)
a = [name for name,thing in inspect.getmembers(stitcher)]
print(a)
v = stitcher.composePanorama(images)
print(v)
exit()
# Check if stitching was successful
if status == cv2.Stitcher_OK:

    # Save the stitched image
    cv2.imwrite(args["output"], stitched_image)
    
    print(stitcher)


    # # Access internal information about the translation of each image
    # warpers = stitcher.warper().warps()
    # translations = []
    # for i, warp in enumerate(warpers):
    #     tx = warp[0, 2]
    #     ty = warp[1, 2]
    #     translations.append((tx, ty))    
    # Get information about the translation of each image
    # translations = []
    # for i in range(len(images) - 1):
    #     src = images[i]
    #     dst = images[i + 1]
        
    #     # Find keypoints and descriptors with SIFT
    #     sift = cv2.SIFT_create()
    #     kp1, des1 = sift.detectAndCompute(src, None)
    #     kp2, des2 = sift.detectAndCompute(dst, None)
        
    #     # Use FLANN based matcher to find matches
    #     index_params = dict(algorithm=1, trees=5)
    #     search_params = dict(checks=50)
    #     flann = cv2.FlannBasedMatcher(index_params, search_params)
    #     matches = flann.knnMatch(des1, des2, k=2)
        
    #     # Apply Lowe's ratio test
    #     good_matches = []
    #     for m, n in matches:
    #         if m.distance < 0.7 * n.distance:
    #             good_matches.append(m)
        
    #     # Extract location of good matches
    #     src_pts = np.float32([kp1[m.queryIdx].pt for m in good_matches]).reshape(-1, 1, 2)
    #     dst_pts = np.float32([kp2[m.trainIdx].pt for m in good_matches]).reshape(-1, 1, 2)
        
    #     # Find homography
    #     M, mask = cv2.findHomography(src_pts, dst_pts, cv2.RANSAC, 5.0)
        
    #     # Extract translation information
    #     if M is not None:
    #         tx = M[0, 2]
    #         ty = M[1, 2]
    #         translations.append((tx, ty))
    #     else:
    #         translations.append((None, None))

    print("Translations between consecutive images:", translations)
else:
    print("Stitching failed with status code", status)
