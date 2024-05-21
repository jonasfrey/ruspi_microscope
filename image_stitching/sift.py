import cv2
import numpy as np

# Load the two images
image1 = cv2.imread('1.png')
image2 = cv2.imread('2.png')

# Convert images to grayscale
gray1 = cv2.cvtColor(image1, cv2.COLOR_BGR2GRAY)
gray2 = cv2.cvtColor(image2, cv2.COLOR_BGR2GRAY)

# Initialize the ORB detector
orb = cv2.ORB_create()

# Detect keypoints and descriptors
keypoints1, descriptors1 = orb.detectAndCompute(gray1, None)
keypoints2, descriptors2 = orb.detectAndCompute(gray2, None)

# Use the BFMatcher to find matches
bf = cv2.BFMatcher(cv2.NORM_HAMMING, crossCheck=True)
matches = bf.match(descriptors1, descriptors2)

# Sort the matches based on distance
matches = sorted(matches, key=lambda x: x.distance)

# Extract the matched keypoints
points1 = np.zeros((len(matches), 2), dtype=np.float32)
points2 = np.zeros((len(matches), 2), dtype=np.float32)

for i, match in enumerate(matches):
    points1[i, :] = keypoints1[match.queryIdx].pt
    points2[i, :] = keypoints2[match.trainIdx].pt

# Compute the homography matrix
homography, mask = cv2.findHomography(points2, points1, cv2.RANSAC)

# Warp the second image to align with the first
height1, width1 = image1.shape[:2]
height2, width2 = image2.shape[:2]

warped_image2 = cv2.warpPerspective(image2, homography, (width1 + width2, height1))

# Create a new canvas to hold both images
stitched_image = np.zeros((height1, width1 + width2, 3), dtype=np.uint8)
stitched_image[0:height1, 0:width1] = image1
stitched_image[0:height1, width1:width1+warped_image2.shape[1]] = warped_image2

# Save the stitched image
cv2.imwrite('stitched_image.jpg', stitched_image)

# Display the result
cv2.imshow('Stitched Image', stitched_image)
cv2.waitKey(0)
cv2.destroyAllWindows()
