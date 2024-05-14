# import the necessary packages
from imutils import paths
import numpy as np
import argparse
import imutils
import cv2
import time

# construct the argument parser and parse the arguments
s_name_file_default = str(time.time())+'_python_image_stitching_result.png'

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
images = []
# loop over the image paths, load each one, and add them to our
# images to stich list
for imagePath in imagePaths:
	image = cv2.imread(imagePath)
	images.append(image)
# initialize OpenCV's image sticher object and then perform the image
# stitching
print("[INFO] stitching images...")
stitcher = cv2.createStitcher(
	cv2.Stitcher_SCANS
) if imutils.is_cv3() else cv2.Stitcher_create(
	cv2.Stitcher_SCANS
)
# Reduce cropping by setting the compositing resolution to 'full resolution'
# stitcher.setCompositingResol(-1)  # This sets stitching at full available resolution

(status, stitched) = stitcher.stitch(images)


if status == cv2.Stitcher_OK:
    print("[INFO] stitching successful")
    cv2.imwrite(args["output"], stitched)
    
else:
    print("[INFO] stitching failed ({})".format(status))
    
# write the output stitched image to disk

# display the output stitched image to our screen
# cv2.imshow("Stitched", stitched)
# cv2.waitKey(0)
# usage: 
# python3 image_stitch.py -i public/media/tmp_image_stitching_images/ -o python_image_stitch.jpg


