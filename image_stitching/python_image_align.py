import os
import argparse
import cv2
import numpy as np

def is_translation_only(homography, threshold=0.01):
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

o_ap = argparse.ArgumentParser()
o_ap.add_argument("-img_base", type=str, required=True, help="image that acts as a reference image basis")
o_ap.add_argument("-img_to_be_aligned", type=str, required=True, help="image that will be aligned")
o_ap.add_argument("-img_out", type=str, default='python_image_align_result.png', help="path of the output image")
o_arguments = vars(o_ap.parse_args())

# Open the image files.
img1_color = cv2.imread(o_arguments['img_to_be_aligned'])
img2_color = cv2.imread(o_arguments['img_base'])

if img1_color is None or img2_color is None:
    raise ValueError("One or both of the input images could not be read.")

# Create masks for valid regions (non-black areas)
mask1 = create_valid_mask(img1_color)
mask2 = create_valid_mask(img2_color)

# Create ORB detector with 5000 features.
orb_detector = cv2.ORB_create(5000)

# Find keypoints and descriptors in the valid regions only.
kp1, d1 = orb_detector.detectAndCompute(cv2.cvtColor(img1_color, cv2.COLOR_BGR2GRAY), mask1)
kp2, d2 = orb_detector.detectAndCompute(cv2.cvtColor(img2_color, cv2.COLOR_BGR2GRAY), mask2)

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
# if not is_translation_only(homography):
#     raise ValueError("Alignment requires significant transformation. Alignment failed.")

# Get the size of the input images
h1, w1 = img1_color.shape[:2]
h2, w2 = img2_color.shape[:2]

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
warped_img1 = cv2.warpPerspective(img1_color, homography_translation.dot(homography), (x_max - x_min, y_max - y_min))

# Create an empty canvas for the blended image
blended_img = np.zeros_like(warped_img1)

# Copy the reference image to the correct location in the blended image
blended_img[translation_dist[1]:h2 + translation_dist[1], translation_dist[0]:w2 + translation_dist[0]] = img2_color

# Find the overlapping region
overlap_region = (blended_img != 0) & (warped_img1 != 0)
non_overlap_region1 = (blended_img == 0) & (warped_img1 != 0)
non_overlap_region2 = (blended_img != 0) & (warped_img1 == 0)

# Average the pixel values in the overlapping region
blended_img[overlap_region] = (blended_img[overlap_region] / 2 + warped_img1[overlap_region] / 2).astype(np.uint8)

# Copy the non-overlapping regions
blended_img[non_overlap_region1] = warped_img1[non_overlap_region1]
blended_img[non_overlap_region2] = blended_img[non_overlap_region2]

# Save the output.
cv2.imwrite(o_arguments['img_out'], blended_img)
