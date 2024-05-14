from imutils import paths
import os
import argparse
import cv2
import numpy as np
import imutils
import time

def stitch_images(image1, image2):
    print("[INFO] stitching images...")
    stitcher = cv2.createStitcher(
		cv2.Stitcher_SCANS
	) if imutils.is_cv3() else cv2.Stitcher_create(
		cv2.Stitcher_SCANS
	)

    (status, stitched) = stitcher.stitch([image1, image2])
    return status, stitched

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
        status, stitched = stitch_images(base_image, next_image)

        if status == cv2.Stitcher_OK:
            print("[INFO] stitching successful")
            base_image = stitched
        else:
            print(f"[INFO] stitching failed ({status}), adding to retry list")
            fails.append(imagePath)

    # # Retry failed images
    # for imagePath in fails[:]:
    #     next_image = cv2.imread(imagePath)
    #     print(f"[INFO] retrying stitching for {imagePath}...")
    #     status, stitched = stitch_images(base_image, next_image)

    #     if status == cv2.Stitcher_OK:
    #         print("[INFO] retry stitching successful")
    #         base_image = stitched
    #         fails.remove(imagePath)
    #     else:
    #         print(f"[INFO] retry stitching failed ({status})")


	# Save the final stitched image
    cv2.imwrite(args["output"], base_image)
    print(f"[INFO] Final stitched image saved as {args['output']}")

if __name__ == "__main__":
    main()
