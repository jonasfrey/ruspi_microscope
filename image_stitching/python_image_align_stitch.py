# import the necessary packages
from imutils import paths
import numpy as np
import argparse
import imutils
import cv2
import time

# construct the argument parser and parse the arguments
o_ap = argparse.ArgumentParser()
o_ap.add_argument("-img_base", type=str, required=True, help="image that acts as a reference image basis")
o_ap.add_argument("-img_to_be_aligned", type=str, required=True, help="image that will be aligned")
o_ap.add_argument("-img_out", type=str, default='python_image_align_result.png', help="path of the output image")
o_arguments = vars(o_ap.parse_args())
# grab the paths to the input images and initialize our images list

img1_color = cv2.imread(o_arguments['img_to_be_aligned'])
img2_color = cv2.imread(o_arguments['img_base'])

stitcher = cv2.createStitcher(
	cv2.Stitcher_SCANS
) if imutils.is_cv3() else cv2.Stitcher_create(
	cv2.Stitcher_SCANS
)
# Reduce cropping by setting the compositing resolution to 'full resolution'
# stitcher.setCompositingResol(-1)  # This sets stitching at full available resolution

(status, stitched) = stitcher.stitch([img1_color,img2_color])


if status == cv2.Stitcher_OK:
    print("[INFO] stitching successful")
    cv2.imwrite(o_arguments["img_out"], stitched)
    
else:
    print("[INFO] stitching failed ({})".format(status))
    
# write the output stitched image to disk

# display the output stitched image to our screen
# cv2.imshow("Stitched", stitched)
# cv2.waitKey(0)
# usage: 
# python3 image_stitch.py -i public/media/tmp_image_stitching_images/ -o python_image_stitch.jpg


