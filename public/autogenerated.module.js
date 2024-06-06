

        class O_microscope_brand{
            constructor(
                n_u32_id,
s_name,
n_u64_ts_ms_ut__created,
n_u64_ts_ms_ut__updated
            ){
                this.n_u32_id = n_u32_id,
this.s_name = s_name,
this.n_u64_ts_ms_ut__created = n_u64_ts_ms_ut__created,
this.n_u64_ts_ms_ut__updated = n_u64_ts_ms_ut__updated
            }
        }
    

        class O_microscope_objective{
            constructor(
                n_u32_id,
s_name,
n_f64_magnification,
n_f64_numerical_aperture__dividend,
n_f64_numerical_aperture__divisor,
n_f64_focal_length,
n_f64_field_of_view,
b_plan_apochromatic,
s_tube_length,
n_u64_ts_ms_ut__created,
n_u64_ts_ms_ut__updated
            ){
                this.n_u32_id = n_u32_id,
this.s_name = s_name,
this.n_f64_magnification = n_f64_magnification,
this.n_f64_numerical_aperture__dividend = n_f64_numerical_aperture__dividend,
this.n_f64_numerical_aperture__divisor = n_f64_numerical_aperture__divisor,
this.n_f64_focal_length = n_f64_focal_length,
this.n_f64_field_of_view = n_f64_field_of_view,
this.b_plan_apochromatic = b_plan_apochromatic,
this.s_tube_length = s_tube_length,
this.n_u64_ts_ms_ut__created = n_u64_ts_ms_ut__created,
this.n_u64_ts_ms_ut__updated = n_u64_ts_ms_ut__updated
            }
        }
    

        class O_microscope{
            constructor(
                n_u32_id,
s_model,
n_u32_id__O_microscope_brand,
n_u64_ts_ms_ut__created
            ){
                this.n_u32_id = n_u32_id,
this.s_model = s_model,
this.n_u32_id__O_microscope_brand = n_u32_id__O_microscope_brand,
this.n_u64_ts_ms_ut__created = n_u64_ts_ms_ut__created
            }
        }
    

        class O_image{
            constructor(
                n_u32_id,
s_path_abs_file,
n_u32_id__O_microscope,
n_u32_id__O_microscope_objective,
n_u64_ts_ms_ut__created
            ){
                this.n_u32_id = n_u32_id,
this.s_path_abs_file = s_path_abs_file,
this.n_u32_id__O_microscope = n_u32_id__O_microscope,
this.n_u32_id__O_microscope_objective = n_u32_id__O_microscope_objective,
this.n_u64_ts_ms_ut__created = n_u64_ts_ms_ut__created
            }
        }
    

        class O_image_description{
            constructor(
                n_u32_id,
s_description,
n_f64_micrometer_x_axis,
b_ai_estimated,
n_u32_id__O_image,
n_u64_ts_ms_ut__created,
n_u64_ts_ms_ut__updated
            ){
                this.n_u32_id = n_u32_id,
this.s_description = s_description,
this.n_f64_micrometer_x_axis = n_f64_micrometer_x_axis,
this.b_ai_estimated = b_ai_estimated,
this.n_u32_id__O_image = n_u32_id__O_image,
this.n_u64_ts_ms_ut__created = n_u64_ts_ms_ut__created,
this.n_u64_ts_ms_ut__updated = n_u64_ts_ms_ut__updated
            }
        }
    

        class O_vec2{
            constructor(
                n_f64_x,
n_f64_y
            ){
                this.n_f64_x = n_f64_x,
this.n_f64_y = n_f64_y
            }
        }
    

        class O_spacial_information_nor{
            constructor(
                n_u32_id,
n_u32_id__O_vec2__trn,
n_u32_id__O_vec2__scl
            ){
                this.n_u32_id = n_u32_id,
this.n_u32_id__O_vec2__trn = n_u32_id__O_vec2__trn,
this.n_u32_id__O_vec2__scl = n_u32_id__O_vec2__scl
            }
        }
    

        class O_input_action{
            constructor(
                n_u32_id,
s_name,
s_nicename,
s_description
            ){
                this.n_u32_id = n_u32_id,
this.s_name = s_name,
this.s_nicename = s_nicename,
this.s_description = s_description
            }
        }
    

        class O_image_object{
            constructor(
                n_u32_id,
s_name,
s_description,
n_u32_id__O_spacial_information_nor,
b_ai_estimated
            ){
                this.n_u32_id = n_u32_id,
this.s_name = s_name,
this.s_description = s_description,
this.n_u32_id__O_spacial_information_nor = n_u32_id__O_spacial_information_nor,
this.b_ai_estimated = b_ai_estimated
            }
        }
    

        class O_image_description_o_image_object{
            constructor(
                n_u32_id,
n_u32_id__O_image_description,
n_u32_id__O_image_object,
n_u64_ts_ms_ut__created
            ){
                this.n_u32_id = n_u32_id,
this.n_u32_id__O_image_description = n_u32_id__O_image_description,
this.n_u32_id__O_image_object = n_u32_id__O_image_object,
this.n_u64_ts_ms_ut__created = n_u64_ts_ms_ut__created
            }
        }
    

export {
    O_microscope_brand,
O_microscope_objective,
O_microscope,
O_image,
O_image_description,
O_vec2,
O_spacial_information_nor,
O_input_action,
O_image_object,
O_image_description_o_image_object
}


    let s_name_o_input_sensor__face_button_bottom = 'face_button_bottom';
let s_name_o_input_sensor__face_button_right = 'face_button_right';
let s_name_o_input_sensor__face_button_top = 'face_button_top';
let s_name_o_input_sensor__face_button_left = 'face_button_left';
let s_name_o_input_sensor__right_index_finger_button_r1 = 'right_index_finger_button_r1';
let s_name_o_input_sensor__right_middle_finger_button_r2 = 'right_middle_finger_button_r2';
let s_name_o_input_sensor__left_index_finger_button_l1 = 'left_index_finger_button_l1';
let s_name_o_input_sensor__left_middle_finger_button_l2 = 'left_middle_finger_button_l2';
let s_name_o_input_sensor__left_stick_button_l3 = 'left_stick_button_l3';
let s_name_o_input_sensor__right_stick_button_r3 = 'right_stick_button_r3';
let s_name_o_input_sensor__left_stick_x_axis = 'left_stick_x_axis';
let s_name_o_input_sensor__left_stick_y_axis = 'left_stick_y_axis';
let s_name_o_input_sensor__right_stick_x_axis = 'right_stick_x_axis';
let s_name_o_input_sensor__right_stick_y_axis = 'right_stick_y_axis';
let s_name_o_input_sensor__left_meta1_button = 'left_meta1_button';
let s_name_o_input_sensor__right_meta1_button = 'right_meta1_button';
let s_name_o_input_sensor__center_meta1_button = 'center_meta1_button';
let s_name_o_input_sensor__direction_pad_up = 'direction_pad_up';
let s_name_o_input_sensor__direction_pad_down = 'direction_pad_down';
let s_name_o_input_sensor__direction_pad_right = 'direction_pad_right';
let s_name_o_input_sensor__direction_pad_left = 'direction_pad_left';
let s_name_o_input_sensor__direction_pad_values = 'direction_pad_values'
    let a_s_name_o_input_sensor = [
        s_name_o_input_sensor__face_button_bottom,
s_name_o_input_sensor__face_button_right,
s_name_o_input_sensor__face_button_top,
s_name_o_input_sensor__face_button_left,
s_name_o_input_sensor__right_index_finger_button_r1,
s_name_o_input_sensor__right_middle_finger_button_r2,
s_name_o_input_sensor__left_index_finger_button_l1,
s_name_o_input_sensor__left_middle_finger_button_l2,
s_name_o_input_sensor__left_stick_button_l3,
s_name_o_input_sensor__right_stick_button_r3,
s_name_o_input_sensor__left_stick_x_axis,
s_name_o_input_sensor__left_stick_y_axis,
s_name_o_input_sensor__right_stick_x_axis,
s_name_o_input_sensor__right_stick_y_axis,
s_name_o_input_sensor__left_meta1_button,
s_name_o_input_sensor__right_meta1_button,
s_name_o_input_sensor__center_meta1_button,
s_name_o_input_sensor__direction_pad_up,
s_name_o_input_sensor__direction_pad_down,
s_name_o_input_sensor__direction_pad_right,
s_name_o_input_sensor__direction_pad_left,
s_name_o_input_sensor__direction_pad_values
    ]
    let s_name_ws_action__hello = "hello";
let s_name_ws_action__f_save_screenshot = "f_save_screenshot";
let s_name_ws_action__f_add_image_to_focus_stack = "f_add_image_to_focus_stack";
let s_name_ws_action__f_create_focus_stack = "f_create_focus_stack";
let s_name_ws_action__f_add_iamge_to_image_stitch = "f_add_iamge_to_image_stitch";
let s_name_ws_action__f_update_image_stitching_result = "f_update_image_stitching_result";
let s_name_ws_action__f_s_read_text_file = "f_s_read_text_file";
let s_name_ws_action__f_b_write_s_text_file = "f_b_write_s_text_file";
let s_name_ws_action__f_o_command = "f_o_command";
let s_name_ws_action__f_control_stepper_motor = "f_control_stepper_motor";
let s_name_ws_action__f_switch_usb_device = "f_switch_usb_device";
    let a_s_name_ws_action = [
        s_name_ws_action__hello,s_name_ws_action__f_save_screenshot,s_name_ws_action__f_add_image_to_focus_stack,s_name_ws_action__f_create_focus_stack,s_name_ws_action__f_add_iamge_to_image_stitch,s_name_ws_action__f_update_image_stitching_result,s_name_ws_action__f_s_read_text_file,s_name_ws_action__f_b_write_s_text_file,s_name_ws_action__f_o_command,s_name_ws_action__f_control_stepper_motor,s_name_ws_action__f_switch_usb_device
    ];
    export { 
        a_s_name_ws_action,
        s_name_ws_action__hello,s_name_ws_action__f_save_screenshot,s_name_ws_action__f_add_image_to_focus_stack,s_name_ws_action__f_create_focus_stack,s_name_ws_action__f_add_iamge_to_image_stitch,s_name_ws_action__f_update_image_stitching_result,s_name_ws_action__f_s_read_text_file,s_name_ws_action__f_b_write_s_text_file,s_name_ws_action__f_o_command,s_name_ws_action__f_control_stepper_motor,s_name_ws_action__f_switch_usb_device,
        a_s_name_o_input_sensor,
        s_name_o_input_sensor__face_button_bottom,
s_name_o_input_sensor__face_button_right,
s_name_o_input_sensor__face_button_top,
s_name_o_input_sensor__face_button_left,
s_name_o_input_sensor__right_index_finger_button_r1,
s_name_o_input_sensor__right_middle_finger_button_r2,
s_name_o_input_sensor__left_index_finger_button_l1,
s_name_o_input_sensor__left_middle_finger_button_l2,
s_name_o_input_sensor__left_stick_button_l3,
s_name_o_input_sensor__right_stick_button_r3,
s_name_o_input_sensor__left_stick_x_axis,
s_name_o_input_sensor__left_stick_y_axis,
s_name_o_input_sensor__right_stick_x_axis,
s_name_o_input_sensor__right_stick_y_axis,
s_name_o_input_sensor__left_meta1_button,
s_name_o_input_sensor__right_meta1_button,
s_name_o_input_sensor__center_meta1_button,
s_name_o_input_sensor__direction_pad_up,
s_name_o_input_sensor__direction_pad_down,
s_name_o_input_sensor__direction_pad_right,
s_name_o_input_sensor__direction_pad_left,
s_name_o_input_sensor__direction_pad_values
    }
    


let o_microscope_brand__GenericBrand = new O_microscope_brand(1,
"GenericBrand",
1717665985881,
1717665985881);let o_microscope_brand__Olympus = new O_microscope_brand(2,
"Olympus",
1717665985881,
1717665985881);let o_microscope_brand__Bresser = new O_microscope_brand(3,
"Bresser",
1717665985881,
1717665985881);let o_microscope_brand__Nikon = new O_microscope_brand(4,
"Nikon",
1717665985881,
1717665985881);let o_microscope_brand__Zeiss = new O_microscope_brand(5,
"Zeiss",
1717665985881,
1717665985881);let o_microscope_brand__Leica = new O_microscope_brand(6,
"Leica",
1717665985881,
1717665985881);let o_microscope_objective__4x_Objective = new O_microscope_objective(1,
"4x Objective",
4,
0.1,
1,
17,
4.5,
false,
"160mm",
1717665985881,
1717665985881);let o_microscope_objective__10x_Objective = new O_microscope_objective(2,
"10x Objective",
10,
0.25,
1,
16,
2,
false,
"160mm",
1717665985881,
1717665985881);let o_microscope_objective__20x_Objective = new O_microscope_objective(3,
"20x Objective",
20,
0.4,
1,
15,
1,
true,
"160mm",
1717665985881,
1717665985881);let o_microscope_objective__40x_Objective = new O_microscope_objective(4,
"40x Objective",
40,
0.65,
1,
14,
0.5,
true,
"160mm",
1717665985881,
1717665985881);let o_microscope__ModelX = new O_microscope(1,
"ModelX",
1,
1717665985881);let o_input_action__move_slide_x_plus = new O_input_action(1,
"move_slide_x_plus",
"Slide right",
"Physically move the slide (only if the mic is motorized)");let o_input_action__move_slide_x_minus = new O_input_action(2,
"move_slide_x_minus",
"Slide left",
"");let o_input_action__move_slide_y_plus = new O_input_action(3,
"move_slide_y_plus",
"Slide up",
"");let o_input_action__move_slide_y_minus = new O_input_action(4,
"move_slide_y_minus",
"Slide down",
"");let o_input_action__move_focus_plus = new O_input_action(5,
"move_focus_plus",
"Focus in",
"");let o_input_action__move_focus_minus = new O_input_action(6,
"move_focus_minus",
"Focus out",
"Physically turn the focus know (only if the mic is motorized)");let o_input_action__move_slide_single_step_x_plus = new O_input_action(7,
"move_slide_single_step_x_plus",
"Slide right (one-step)",
"Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter");let o_input_action__move_slide_single_step_x_minus = new O_input_action(8,
"move_slide_single_step_x_minus",
"Slide left (one-step)",
"Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter");let o_input_action__move_slide_single_step_y_plus = new O_input_action(9,
"move_slide_single_step_y_plus",
"Slide up (one-step)",
"Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter");let o_input_action__move_slide_single_step_y_minus = new O_input_action(10,
"move_slide_single_step_y_minus",
"Slide down (one-step)",
"Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter");let o_input_action__keep_hold_down_toggle_layer2 = new O_input_action(11,
"keep_hold_down_toggle_layer2",
"Shift to layer 2",
"hold to to temporarily switch to a new layer");let o_input_action__keep_hold_down_toggle_layer3 = new O_input_action(12,
"keep_hold_down_toggle_layer3",
"Shift to layer 3",
"hold to to temporarily switch to a new layer");let o_input_action__toggle_settings = new O_input_action(13,
"toggle_settings",
"Settings",
"Press once to show the settings, press again to hide");let o_input_action__next_image_mode = new O_input_action(14,
"next_image_mode",
"Image mode: next",
"Cycle throught the image modes");let o_input_action__previous_image_mode = new O_input_action(15,
"previous_image_mode",
"Image mode: prev",
"");let o_input_action__reset_image_manipulation = new O_input_action(16,
"reset_image_manipulation",
"Image reset",
"");let o_input_action__move_digital_x_plus = new O_input_action(17,
"move_digital_x_plus",
"Digital right",
"");let o_input_action__move_digital_x_minus = new O_input_action(18,
"move_digital_x_minus",
"Digital left",
"");let o_input_action__move_digital_y_plus = new O_input_action(19,
"move_digital_y_plus",
"Digital up",
"");let o_input_action__move_digital_y_minus = new O_input_action(20,
"move_digital_y_minus",
"Digital down",
"Perform a digital movement, may be handy for showing and pointing out interesting image regions");let o_input_action__zoom_digital_plus = new O_input_action(21,
"zoom_digital_plus",
"Digital zoom in",
"");let o_input_action__zoom_digital_minus = new O_input_action(22,
"zoom_digital_minus",
"Digital zoom out",
"Perform a digital zoom, may be handy for showing and pointing out interesting image regions");let o_input_action__next_action_layer = new O_input_action(23,
"next_action_layer",
"Layer: next",
"Switch to the next action layer");let o_input_action__prev_action_layer = new O_input_action(24,
"prev_action_layer",
"Layer: prev",
"Switch to the previous action layer");let o_input_action__take_screenshot = new O_input_action(25,
"take_screenshot",
"Screenshot",
"Take a simple screnshot");let o_input_action__ask_ai = new O_input_action(26,
"ask_ai",
"AI-Help",
"Let AI detect common image features and estimation of size dont yet fully trust it, its GPT Vision AI its not that good yet (but its fun for sure!!!)");let o_input_action__add_image_to_focus_stack = new O_input_action(27,
"add_image_to_focus_stack",
"Add image to focus stack",
"");let o_input_action__add_image_to_image_stitch = new O_input_action(28,
"add_image_to_image_stitch",
"Add image to image stitch",
"Add an image to the image stitching result which autmotically updates in the background after each addition of an image");let o_input_action__clear_image_stitch = new O_input_action(29,
"clear_image_stitch",
"Clear image stitch",
"clears the current image stitching result, no worries it will be saved!");let o_input_action__finish_focus_stack = new O_input_action(30,
"finish_focus_stack",
"Finish focus stack",
"");let o_input_action__toggle_record_video = new O_input_action(31,
"toggle_record_video",
"Record/Stop video",
"");let o_input_action__image_brightness_plus = new O_input_action(32,
"image_brightness_plus",
"Image brightness plus",
"");let o_input_action__image_brightness_minus = new O_input_action(33,
"image_brightness_minus",
"Image brightness minus",
"");let o_input_action__image_contrast_plus = new O_input_action(34,
"image_contrast_plus",
"Image brightness plus",
"");let o_input_action__image_contrast_minus = new O_input_action(35,
"image_contrast_minus",
"Image brightness minus",
"");let o_input_action__image_gamma_plus = new O_input_action(36,
"image_gamma_plus",
"Image brightness plus",
"");let o_input_action__image_gamma_minus = new O_input_action(37,
"image_gamma_minus",
"Image brightness minus",
"");let o_input_action__usb_camera_exposureMode_next = new O_input_action(38,
"usb_camera_exposureMode_next",
"auto, manual",
"");let o_input_action__usb_camera_exposureMode_previous = new O_input_action(39,
"usb_camera_exposureMode_previous",
"auto, manual",
"");let o_input_action__usb_camera_exposureTime_plus = new O_input_action(40,
"usb_camera_exposureTime_plus",
"Time in seconds for exposure",
"");let o_input_action__usb_camera_exposureTime_minus = new O_input_action(41,
"usb_camera_exposureTime_minus",
"Time in seconds for exposure",
"");let o_input_action__usb_camera_whiteBalanceMode_next = new O_input_action(42,
"usb_camera_whiteBalanceMode_next",
"auto, manual",
"");let o_input_action__usb_camera_whiteBalanceMode_previous = new O_input_action(43,
"usb_camera_whiteBalanceMode_previous",
"auto, manual",
"");let o_input_action__usb_camera_whiteBalance_plus = new O_input_action(44,
"usb_camera_whiteBalance_plus",
"Color temperature in Kelvin",
"");let o_input_action__usb_camera_whiteBalance_minus = new O_input_action(44,
"usb_camera_whiteBalance_minus",
"Color temperature in Kelvin",
"");let o_input_action__usb_camera_focusMode_next = new O_input_action(45,
"usb_camera_focusMode_next",
"auto, manual",
"");let o_input_action__usb_camera_focusMode_previous = new O_input_action(46,
"usb_camera_focusMode_previous",
"auto, manual",
"");let o_input_action__usb_camera_focusDistance_plus = new O_input_action(47,
"usb_camera_focusDistance_plus",
"Distance for focus",
"");let o_input_action__usb_camera_focusDistance_minus = new O_input_action(48,
"usb_camera_focusDistance_minus",
"Distance for focus",
"");let o_input_action__usb_camera_brightness_plus = new O_input_action(49,
"usb_camera_brightness_plus",
"Brightness level",
"");let o_input_action__usb_camera_brightness_minus = new O_input_action(50,
"usb_camera_brightness_minus",
"Brightness level",
"");let o_input_action__usb_camera_contrast_minus = new O_input_action(51,
"usb_camera_contrast_minus",
"Contrast level",
"");let o_input_action__usb_camera_contrast_plus = new O_input_action(52,
"usb_camera_contrast_plus",
"Contrast level",
"");let o_input_action__usb_camera_saturation_minus = new O_input_action(53,
"usb_camera_saturation_minus",
"Saturation level",
"");let o_input_action__usb_camera_saturation_plus = new O_input_action(54,
"usb_camera_saturation_plus",
"Saturation level",
"");let o_input_action__usb_camera_sharpness_minus = new O_input_action(55,
"usb_camera_sharpness_minus",
"Sharpness level",
"");let o_input_action__usb_camera_sharpness_plus = new O_input_action(56,
"usb_camera_sharpness_plus",
"Sharpness level",
"");let o_input_action__usb_camera_zoom_minus = new O_input_action(57,
"usb_camera_zoom_minus",
"Zoom level",
"");let o_input_action__usb_camera_zoom_plus = new O_input_action(58,
"usb_camera_zoom_plus",
"Zoom level",
"");let o_input_action__usb_camera_torch_next = new O_input_action(58,
"usb_camera_torch_next",
"Boolean for turning on/off the torch (if supported ) ",
"");let o_input_action__usb_camera_torch_previous = new O_input_action(59,
"usb_camera_torch_previous",
"Boolean for turning on/off the torch (if supported ) ",
"")
let a_o_microscope_brand = [
                o_microscope_brand__GenericBrand,
o_microscope_brand__Olympus,
o_microscope_brand__Bresser,
o_microscope_brand__Nikon,
o_microscope_brand__Zeiss,
o_microscope_brand__Leica
            ];let a_o_microscope_objective = [
                o_microscope_objective__4x_Objective,
o_microscope_objective__10x_Objective,
o_microscope_objective__20x_Objective,
o_microscope_objective__40x_Objective
            ];let a_o_microscope = [
                o_microscope__ModelX
            ];let a_o_image = [
                
            ];let a_o_image_description = [
                
            ];let a_o_vec2 = [
                
            ];let a_o_spacial_information_nor = [
                
            ];let a_o_input_action = [
                o_input_action__move_slide_x_plus,
o_input_action__move_slide_x_minus,
o_input_action__move_slide_y_plus,
o_input_action__move_slide_y_minus,
o_input_action__move_focus_plus,
o_input_action__move_focus_minus,
o_input_action__move_slide_single_step_x_plus,
o_input_action__move_slide_single_step_x_minus,
o_input_action__move_slide_single_step_y_plus,
o_input_action__move_slide_single_step_y_minus,
o_input_action__keep_hold_down_toggle_layer2,
o_input_action__keep_hold_down_toggle_layer3,
o_input_action__toggle_settings,
o_input_action__next_image_mode,
o_input_action__previous_image_mode,
o_input_action__reset_image_manipulation,
o_input_action__move_digital_x_plus,
o_input_action__move_digital_x_minus,
o_input_action__move_digital_y_plus,
o_input_action__move_digital_y_minus,
o_input_action__zoom_digital_plus,
o_input_action__zoom_digital_minus,
o_input_action__next_action_layer,
o_input_action__prev_action_layer,
o_input_action__take_screenshot,
o_input_action__ask_ai,
o_input_action__add_image_to_focus_stack,
o_input_action__add_image_to_image_stitch,
o_input_action__clear_image_stitch,
o_input_action__finish_focus_stack,
o_input_action__toggle_record_video,
o_input_action__image_brightness_plus,
o_input_action__image_brightness_minus,
o_input_action__image_contrast_plus,
o_input_action__image_contrast_minus,
o_input_action__image_gamma_plus,
o_input_action__image_gamma_minus,
o_input_action__usb_camera_exposureMode_next,
o_input_action__usb_camera_exposureMode_previous,
o_input_action__usb_camera_exposureTime_plus,
o_input_action__usb_camera_exposureTime_minus,
o_input_action__usb_camera_whiteBalanceMode_next,
o_input_action__usb_camera_whiteBalanceMode_previous,
o_input_action__usb_camera_whiteBalance_plus,
o_input_action__usb_camera_whiteBalance_minus,
o_input_action__usb_camera_focusMode_next,
o_input_action__usb_camera_focusMode_previous,
o_input_action__usb_camera_focusDistance_plus,
o_input_action__usb_camera_focusDistance_minus,
o_input_action__usb_camera_brightness_plus,
o_input_action__usb_camera_brightness_minus,
o_input_action__usb_camera_contrast_minus,
o_input_action__usb_camera_contrast_plus,
o_input_action__usb_camera_saturation_minus,
o_input_action__usb_camera_saturation_plus,
o_input_action__usb_camera_sharpness_minus,
o_input_action__usb_camera_sharpness_plus,
o_input_action__usb_camera_zoom_minus,
o_input_action__usb_camera_zoom_plus,
o_input_action__usb_camera_torch_next,
o_input_action__usb_camera_torch_previous
            ];let a_o_image_object = [
                
            ];let a_o_image_description_o_image_object = [
                
            ]
export {
    o_microscope_brand__GenericBrand,
o_microscope_brand__Olympus,
o_microscope_brand__Bresser,
o_microscope_brand__Nikon,
o_microscope_brand__Zeiss,
o_microscope_brand__Leica,
o_microscope_objective__4x_Objective,
o_microscope_objective__10x_Objective,
o_microscope_objective__20x_Objective,
o_microscope_objective__40x_Objective,
o_microscope__ModelX,
o_input_action__move_slide_x_plus,
o_input_action__move_slide_x_minus,
o_input_action__move_slide_y_plus,
o_input_action__move_slide_y_minus,
o_input_action__move_focus_plus,
o_input_action__move_focus_minus,
o_input_action__move_slide_single_step_x_plus,
o_input_action__move_slide_single_step_x_minus,
o_input_action__move_slide_single_step_y_plus,
o_input_action__move_slide_single_step_y_minus,
o_input_action__keep_hold_down_toggle_layer2,
o_input_action__keep_hold_down_toggle_layer3,
o_input_action__toggle_settings,
o_input_action__next_image_mode,
o_input_action__previous_image_mode,
o_input_action__reset_image_manipulation,
o_input_action__move_digital_x_plus,
o_input_action__move_digital_x_minus,
o_input_action__move_digital_y_plus,
o_input_action__move_digital_y_minus,
o_input_action__zoom_digital_plus,
o_input_action__zoom_digital_minus,
o_input_action__next_action_layer,
o_input_action__prev_action_layer,
o_input_action__take_screenshot,
o_input_action__ask_ai,
o_input_action__add_image_to_focus_stack,
o_input_action__add_image_to_image_stitch,
o_input_action__clear_image_stitch,
o_input_action__finish_focus_stack,
o_input_action__toggle_record_video,
o_input_action__image_brightness_plus,
o_input_action__image_brightness_minus,
o_input_action__image_contrast_plus,
o_input_action__image_contrast_minus,
o_input_action__image_gamma_plus,
o_input_action__image_gamma_minus,
o_input_action__usb_camera_exposureMode_next,
o_input_action__usb_camera_exposureMode_previous,
o_input_action__usb_camera_exposureTime_plus,
o_input_action__usb_camera_exposureTime_minus,
o_input_action__usb_camera_whiteBalanceMode_next,
o_input_action__usb_camera_whiteBalanceMode_previous,
o_input_action__usb_camera_whiteBalance_plus,
o_input_action__usb_camera_whiteBalance_minus,
o_input_action__usb_camera_focusMode_next,
o_input_action__usb_camera_focusMode_previous,
o_input_action__usb_camera_focusDistance_plus,
o_input_action__usb_camera_focusDistance_minus,
o_input_action__usb_camera_brightness_plus,
o_input_action__usb_camera_brightness_minus,
o_input_action__usb_camera_contrast_minus,
o_input_action__usb_camera_contrast_plus,
o_input_action__usb_camera_saturation_minus,
o_input_action__usb_camera_saturation_plus,
o_input_action__usb_camera_sharpness_minus,
o_input_action__usb_camera_sharpness_plus,
o_input_action__usb_camera_zoom_minus,
o_input_action__usb_camera_zoom_plus,
o_input_action__usb_camera_torch_next,
o_input_action__usb_camera_torch_previous,
    a_o_microscope_brand,
a_o_microscope_objective,
a_o_microscope,
a_o_image,
a_o_image_description,
a_o_vec2,
a_o_spacial_information_nor,
a_o_input_action,
a_o_image_object,
a_o_image_description_o_image_object
}

