# plan
## motorized stage 
1. user connects motors and LED to raspberry
2. user connects controller to raspberry
    - multiple controller should be usable for the moment esperenza (+-20chf) usb wireless controller 0x045e:0x028e
        - most common controllers could/should be supportable 
        - xbox playstation (bluetooth?)
    - controls:
        move stage: 
         - r2: if pressed slow movement (idea)
         - x : right analog x axis (speed depending on how far the analog axis potentiometer is pulled (the potentiometer is not really linear and would have to be scaled better so that more precise speed adjustments are easily possible ))
         - y : right analog y axis
         - z : left analog y axis
         - dpad: up right down lef
            - do a stepper microstep ( just one step per click)
                - hold down dpad, (move slowly multiple steps)
        zoom (not implemented):
         - needs hardware implementation to move the camera sensor along the 'tube' 
         - r1 zoom in
         - l1 zoom out
        
## advanced camera options
1. user connects the microscope camera
    - camera image can be accessed by network eg. 192.168.1.2:8080(raspberry has to be )

    - general image
        on the top left:
            the basic layout of the controller foreach button a small string that descibes the function
            
        on the top right: 
            - for the image properties (brightness, contrast, gamma, saturation)
                - a string containing the prop
                - a bar that is 0.0 to 1.0 (default 0.5) 

        - AI base prompt (uses network connection and openai api key)
            `i want information about this image, the image is taken with a camera that is connected to a microscope give the response as a json object with the following structure, the prefixes indicate the types (n_... number) (s_... string) (o_... object) (a_[prefix]... array , eg. a_o_... arrray of objects, a_s_... array of strings )
            'n_micrometer_x_axis_approx':
                 the approximate size of the image on the full x axis try to guess based on what you see on this
            'a_o_object' (array of objects)
                an array holding objects , each object represents an object that is seen on the image (if there are multiple same for example bloodcells, only the best visible should be listed here otherwise hundrets of object would need to be listed in the example of bloodcells...), where the json structure of a "o_object" looks like this
                's_name' (string)
                    a short string describing the object
                'n_x_nor' 
                    a normalized number describing the position on the x axis where 1.0 is fully on the right and 0.0 is fully on the left of the image
                'n_y_nor'
                    same like n_x_nor for the y axis, 0.0 is fully on the bottom, 1.0 is fully on the top
                'n_micrometer_diameter_approx'
                    approximation of the diameter of the circle bounding the object
                
            's_description' 
                a general description of what can be seen on the image
            ` 

    - right thumb buttons ( ps:xbox -> square:x, x:a, triangle:y circle:b)
        - square:x
            when held down, camera control mode is engaded, now all buttons on the controller can be used for controlling the camera ? 
            this would remove the possibility to move the stage at the same time as capturing images or videos or controlling the image (brighness/gamma/etc...)
            - right y axis: change brightness +-
            - right x axis: change contrast +-
            - left y axis: change gamma +-
            - r3: reset all, l3 reset all (for simplicity reset all, a user cannot remember what to press for resetting a certain property (brightness or contrast or gamma etc.))
            - left x axis: change saturation +-

        - triangle:y
            if clicked the AI base prompt gets sent to the api (requires internet connection) and the result will contain information about the image, 
            the scale of the image will be shown with a bar on the top right
            the ai will identify subjects in the image, for example bloodcells or microorganism , for each subject there is a position and there will be a circle that outcircumfences the subject,a name of the object an a size of the object
        - x:a
            capture a frame and download it from the browser
        - circle:b
            start capturing a video, 'rec' icon will be shown and pulsing on the top right
            click again to stop the video, the video will be downloaded from browser and stored on computer, ai will be used to name the file (or put this as a setting since it uses network connection and AI api key)
    - r3
        click once:
            start following the subject: 
        click again: 
            stop following
        how to follow?:
            if clicked a subject will be picked in the image and will be followed by sending a prompt "where is the subject in the new image, in the old image it was on xy 0.23:0.4 (old image), (new image)" the delta of old and new position can be used to move the stage to the new positoin. if this is executed in a loop the subject will be followed
        what to follow?:
            a way to select the subject to follow has to be implemented
        
    
## advanced

3. user can controll microscope stage with controller 


# requirements 

## list webcams 
opencv is so dumb and cannot list the webcams, it also cannot get string (name of the webcam) of any webcam, it can only open it and read it by the index for example 0
therefore to list the webcams and check if a cam is available we have to 
sudo apt-get install v4l-utils

## read webcam 
opencv 
https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md
apt install libopencv-dev clang libclang-dev
configure clang
check version of 
gcc --version
export CPLUS_INCLUDE_PATH="/usr/include/c++/[gccversionhere]:/usr/include/x86_64-linux-gnu/c++/[gccversionhere]
## stream webcam
```rust

```