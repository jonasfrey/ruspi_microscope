
use lazy_static::lazy_static;

    #[derive(Clone)]
    pub struct O_microscope_brand{
        pub n_u32_id: u32,
pub s_name: String,
pub n_u64_ts_ms_ut__created: u64,
pub n_u64_ts_ms_ut__updated: u64
    }

    #[derive(Clone)]
    pub struct O_microscope_objective{
        pub n_u32_id: u32,
pub s_name: String,
pub n_f64_magnification: f64,
pub n_f64_numerical_aperture__dividend: f64,
pub n_f64_numerical_aperture__divisor: f64,
pub n_f64_focal_length: f64,
pub n_f64_field_of_view: f64,
pub b_plan_apochromatic: bool,
pub s_tube_length: String,
pub n_u64_ts_ms_ut__created: u64,
pub n_u64_ts_ms_ut__updated: u64
    }

    #[derive(Clone)]
    pub struct O_microscope{
        pub n_u32_id: u32,
pub s_model: String,
pub n_u32_id__O_microscope_brand: u32,
pub n_u64_ts_ms_ut__created: u64
    }

    #[derive(Clone)]
    pub struct O_image{
        pub n_u32_id: u32,
pub s_path_abs_file: String,
pub n_u32_id__O_microscope: u32,
pub n_u32_id__O_microscope_objective: u32,
pub n_u64_ts_ms_ut__created: u64
    }

    #[derive(Clone)]
    pub struct O_image_description{
        pub n_u32_id: u32,
pub s_description: String,
pub n_f64_micrometer_x_axis: f64,
pub b_ai_estimated: bool,
pub n_u32_id__O_image: u32,
pub n_u64_ts_ms_ut__created: u64,
pub n_u64_ts_ms_ut__updated: u64
    }

    #[derive(Clone)]
    pub struct O_vec2{
        pub n_f64_x: f64,
pub n_f64_y: f64
    }

    #[derive(Clone)]
    pub struct O_spacial_information_nor{
        pub n_u32_id: u32,
pub n_u32_id__O_vec2__trn: u32,
pub n_u32_id__O_vec2__scl: u32
    }

    #[derive(Clone)]
    pub struct O_input_action{
        pub n_u32_id: u32,
pub s_name: String,
pub s_nicename: String,
pub s_description: String
    }

    #[derive(Clone)]
    pub struct O_image_object{
        pub n_u32_id: u32,
pub s_name: String,
pub s_description: String,
pub n_u32_id__O_spacial_information_nor: u32,
pub b_ai_estimated: bool
    }

    #[derive(Clone)]
    pub struct O_image_description_o_image_object{
        pub n_u32_id: u32,
pub n_u32_id__O_image_description: u32,
pub n_u32_id__O_image_object: u32,
pub n_u64_ts_ms_ut__created: u64
    }
lazy_static! {
    pub static ref s_name_ws_action__hello: String = String::from("hello");
pub static ref s_name_ws_action__f_save_screenshot: String = String::from("f_save_screenshot");
pub static ref s_name_ws_action__f_add_image_to_focus_stack: String = String::from("f_add_image_to_focus_stack");
pub static ref s_name_ws_action__f_create_focus_stack: String = String::from("f_create_focus_stack");
pub static ref s_name_ws_action__f_add_iamge_to_image_stitch: String = String::from("f_add_iamge_to_image_stitch");
pub static ref s_name_ws_action__f_update_image_stitching_result: String = String::from("f_update_image_stitching_result");
pub static ref s_name_ws_action__f_s_read_text_file: String = String::from("f_s_read_text_file");
pub static ref s_name_ws_action__f_b_write_s_text_file: String = String::from("f_b_write_s_text_file");
pub static ref s_name_ws_action__f_o_command: String = String::from("f_o_command");
pub static ref s_name_ws_action__f_control_stepper_motor: String = String::from("f_control_stepper_motor");
pub static ref s_name_ws_action__f_switch_usb_device: String = String::from("f_switch_usb_device");
    


                pub static ref o_microscope_brand__GenericBrand: O_microscope_brand = O_microscope_brand{
                    n_u32_id: 1 as u32,
s_name: String::from("GenericBrand"),
n_u64_ts_ms_ut__created: 1717665985881 as u64,
n_u64_ts_ms_ut__updated: 1717665985881 as u64
                }
                ;
                pub static ref o_microscope_brand__Olympus: O_microscope_brand = O_microscope_brand{
                    n_u32_id: 2 as u32,
s_name: String::from("Olympus"),
n_u64_ts_ms_ut__created: 1717665985881 as u64,
n_u64_ts_ms_ut__updated: 1717665985881 as u64
                }
                ;
                pub static ref o_microscope_brand__Bresser: O_microscope_brand = O_microscope_brand{
                    n_u32_id: 3 as u32,
s_name: String::from("Bresser"),
n_u64_ts_ms_ut__created: 1717665985881 as u64,
n_u64_ts_ms_ut__updated: 1717665985881 as u64
                }
                ;
                pub static ref o_microscope_brand__Nikon: O_microscope_brand = O_microscope_brand{
                    n_u32_id: 4 as u32,
s_name: String::from("Nikon"),
n_u64_ts_ms_ut__created: 1717665985881 as u64,
n_u64_ts_ms_ut__updated: 1717665985881 as u64
                }
                ;
                pub static ref o_microscope_brand__Zeiss: O_microscope_brand = O_microscope_brand{
                    n_u32_id: 5 as u32,
s_name: String::from("Zeiss"),
n_u64_ts_ms_ut__created: 1717665985881 as u64,
n_u64_ts_ms_ut__updated: 1717665985881 as u64
                }
                ;
                pub static ref o_microscope_brand__Leica: O_microscope_brand = O_microscope_brand{
                    n_u32_id: 6 as u32,
s_name: String::from("Leica"),
n_u64_ts_ms_ut__created: 1717665985881 as u64,
n_u64_ts_ms_ut__updated: 1717665985881 as u64
                }
                ;
                pub static ref o_microscope_objective__4x_Objective: O_microscope_objective = O_microscope_objective{
                    n_u32_id: 1 as u32,
s_name: String::from("4x Objective"),
n_f64_magnification: 4 as f64,
n_f64_numerical_aperture__dividend: 0.1 as f64,
n_f64_numerical_aperture__divisor: 1 as f64,
n_f64_focal_length: 17 as f64,
n_f64_field_of_view: 4.5 as f64,
b_plan_apochromatic: false as bool,
s_tube_length: String::from("160mm"),
n_u64_ts_ms_ut__created: 1717665985881 as u64,
n_u64_ts_ms_ut__updated: 1717665985881 as u64
                }
                ;
                pub static ref o_microscope_objective__10x_Objective: O_microscope_objective = O_microscope_objective{
                    n_u32_id: 2 as u32,
s_name: String::from("10x Objective"),
n_f64_magnification: 10 as f64,
n_f64_numerical_aperture__dividend: 0.25 as f64,
n_f64_numerical_aperture__divisor: 1 as f64,
n_f64_focal_length: 16 as f64,
n_f64_field_of_view: 2 as f64,
b_plan_apochromatic: false as bool,
s_tube_length: String::from("160mm"),
n_u64_ts_ms_ut__created: 1717665985881 as u64,
n_u64_ts_ms_ut__updated: 1717665985881 as u64
                }
                ;
                pub static ref o_microscope_objective__20x_Objective: O_microscope_objective = O_microscope_objective{
                    n_u32_id: 3 as u32,
s_name: String::from("20x Objective"),
n_f64_magnification: 20 as f64,
n_f64_numerical_aperture__dividend: 0.4 as f64,
n_f64_numerical_aperture__divisor: 1 as f64,
n_f64_focal_length: 15 as f64,
n_f64_field_of_view: 1 as f64,
b_plan_apochromatic: true as bool,
s_tube_length: String::from("160mm"),
n_u64_ts_ms_ut__created: 1717665985881 as u64,
n_u64_ts_ms_ut__updated: 1717665985881 as u64
                }
                ;
                pub static ref o_microscope_objective__40x_Objective: O_microscope_objective = O_microscope_objective{
                    n_u32_id: 4 as u32,
s_name: String::from("40x Objective"),
n_f64_magnification: 40 as f64,
n_f64_numerical_aperture__dividend: 0.65 as f64,
n_f64_numerical_aperture__divisor: 1 as f64,
n_f64_focal_length: 14 as f64,
n_f64_field_of_view: 0.5 as f64,
b_plan_apochromatic: true as bool,
s_tube_length: String::from("160mm"),
n_u64_ts_ms_ut__created: 1717665985881 as u64,
n_u64_ts_ms_ut__updated: 1717665985881 as u64
                }
                ;
                pub static ref o_microscope__ModelX: O_microscope = O_microscope{
                    n_u32_id: 1 as u32,
s_model: String::from("ModelX"),
n_u32_id__O_microscope_brand: 1 as u32,
n_u64_ts_ms_ut__created: 1717665985881 as u64
                }
                ;
                pub static ref o_input_action__move_slide_x_plus: O_input_action = O_input_action{
                    n_u32_id: 1 as u32,
s_name: String::from("move_slide_x_plus"),
s_nicename: String::from("Slide right"),
s_description: String::from("Physically move the slide (only if the mic is motorized)")
                }
                ;
                pub static ref o_input_action__move_slide_x_minus: O_input_action = O_input_action{
                    n_u32_id: 2 as u32,
s_name: String::from("move_slide_x_minus"),
s_nicename: String::from("Slide left"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__move_slide_y_plus: O_input_action = O_input_action{
                    n_u32_id: 3 as u32,
s_name: String::from("move_slide_y_plus"),
s_nicename: String::from("Slide up"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__move_slide_y_minus: O_input_action = O_input_action{
                    n_u32_id: 4 as u32,
s_name: String::from("move_slide_y_minus"),
s_nicename: String::from("Slide down"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__move_focus_plus: O_input_action = O_input_action{
                    n_u32_id: 5 as u32,
s_name: String::from("move_focus_plus"),
s_nicename: String::from("Focus in"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__move_focus_minus: O_input_action = O_input_action{
                    n_u32_id: 6 as u32,
s_name: String::from("move_focus_minus"),
s_nicename: String::from("Focus out"),
s_description: String::from("Physically turn the focus know (only if the mic is motorized)")
                }
                ;
                pub static ref o_input_action__move_slide_single_step_x_plus: O_input_action = O_input_action{
                    n_u32_id: 7 as u32,
s_name: String::from("move_slide_single_step_x_plus"),
s_nicename: String::from("Slide right (one-step)"),
s_description: String::from("Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter")
                }
                ;
                pub static ref o_input_action__move_slide_single_step_x_minus: O_input_action = O_input_action{
                    n_u32_id: 8 as u32,
s_name: String::from("move_slide_single_step_x_minus"),
s_nicename: String::from("Slide left (one-step)"),
s_description: String::from("Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter")
                }
                ;
                pub static ref o_input_action__move_slide_single_step_y_plus: O_input_action = O_input_action{
                    n_u32_id: 9 as u32,
s_name: String::from("move_slide_single_step_y_plus"),
s_nicename: String::from("Slide up (one-step)"),
s_description: String::from("Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter")
                }
                ;
                pub static ref o_input_action__move_slide_single_step_y_minus: O_input_action = O_input_action{
                    n_u32_id: 10 as u32,
s_name: String::from("move_slide_single_step_y_minus"),
s_nicename: String::from("Slide down (one-step)"),
s_description: String::from("Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter")
                }
                ;
                pub static ref o_input_action__keep_hold_down_toggle_layer2: O_input_action = O_input_action{
                    n_u32_id: 11 as u32,
s_name: String::from("keep_hold_down_toggle_layer2"),
s_nicename: String::from("Shift to layer 2"),
s_description: String::from("hold to to temporarily switch to a new layer")
                }
                ;
                pub static ref o_input_action__keep_hold_down_toggle_layer3: O_input_action = O_input_action{
                    n_u32_id: 12 as u32,
s_name: String::from("keep_hold_down_toggle_layer3"),
s_nicename: String::from("Shift to layer 3"),
s_description: String::from("hold to to temporarily switch to a new layer")
                }
                ;
                pub static ref o_input_action__toggle_settings: O_input_action = O_input_action{
                    n_u32_id: 13 as u32,
s_name: String::from("toggle_settings"),
s_nicename: String::from("Settings"),
s_description: String::from("Press once to show the settings, press again to hide")
                }
                ;
                pub static ref o_input_action__next_image_mode: O_input_action = O_input_action{
                    n_u32_id: 14 as u32,
s_name: String::from("next_image_mode"),
s_nicename: String::from("Image mode: next"),
s_description: String::from("Cycle throught the image modes")
                }
                ;
                pub static ref o_input_action__previous_image_mode: O_input_action = O_input_action{
                    n_u32_id: 15 as u32,
s_name: String::from("previous_image_mode"),
s_nicename: String::from("Image mode: prev"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__reset_image_manipulation: O_input_action = O_input_action{
                    n_u32_id: 16 as u32,
s_name: String::from("reset_image_manipulation"),
s_nicename: String::from("Image reset"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__move_digital_x_plus: O_input_action = O_input_action{
                    n_u32_id: 17 as u32,
s_name: String::from("move_digital_x_plus"),
s_nicename: String::from("Digital right"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__move_digital_x_minus: O_input_action = O_input_action{
                    n_u32_id: 18 as u32,
s_name: String::from("move_digital_x_minus"),
s_nicename: String::from("Digital left"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__move_digital_y_plus: O_input_action = O_input_action{
                    n_u32_id: 19 as u32,
s_name: String::from("move_digital_y_plus"),
s_nicename: String::from("Digital up"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__move_digital_y_minus: O_input_action = O_input_action{
                    n_u32_id: 20 as u32,
s_name: String::from("move_digital_y_minus"),
s_nicename: String::from("Digital down"),
s_description: String::from("Perform a digital movement, may be handy for showing and pointing out interesting image regions")
                }
                ;
                pub static ref o_input_action__zoom_digital_plus: O_input_action = O_input_action{
                    n_u32_id: 21 as u32,
s_name: String::from("zoom_digital_plus"),
s_nicename: String::from("Digital zoom in"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__zoom_digital_minus: O_input_action = O_input_action{
                    n_u32_id: 22 as u32,
s_name: String::from("zoom_digital_minus"),
s_nicename: String::from("Digital zoom out"),
s_description: String::from("Perform a digital zoom, may be handy for showing and pointing out interesting image regions")
                }
                ;
                pub static ref o_input_action__next_action_layer: O_input_action = O_input_action{
                    n_u32_id: 23 as u32,
s_name: String::from("next_action_layer"),
s_nicename: String::from("Layer: next"),
s_description: String::from("Switch to the next action layer")
                }
                ;
                pub static ref o_input_action__prev_action_layer: O_input_action = O_input_action{
                    n_u32_id: 24 as u32,
s_name: String::from("prev_action_layer"),
s_nicename: String::from("Layer: prev"),
s_description: String::from("Switch to the previous action layer")
                }
                ;
                pub static ref o_input_action__take_screenshot: O_input_action = O_input_action{
                    n_u32_id: 25 as u32,
s_name: String::from("take_screenshot"),
s_nicename: String::from("Screenshot"),
s_description: String::from("Take a simple screnshot")
                }
                ;
                pub static ref o_input_action__ask_ai: O_input_action = O_input_action{
                    n_u32_id: 26 as u32,
s_name: String::from("ask_ai"),
s_nicename: String::from("AI-Help"),
s_description: String::from("Let AI detect common image features and estimation of size dont yet fully trust it, its GPT Vision AI its not that good yet (but its fun for sure!!!)")
                }
                ;
                pub static ref o_input_action__add_image_to_focus_stack: O_input_action = O_input_action{
                    n_u32_id: 27 as u32,
s_name: String::from("add_image_to_focus_stack"),
s_nicename: String::from("Add image to focus stack"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__add_image_to_image_stitch: O_input_action = O_input_action{
                    n_u32_id: 28 as u32,
s_name: String::from("add_image_to_image_stitch"),
s_nicename: String::from("Add image to image stitch"),
s_description: String::from("Add an image to the image stitching result which autmotically updates in the background after each addition of an image")
                }
                ;
                pub static ref o_input_action__clear_image_stitch: O_input_action = O_input_action{
                    n_u32_id: 29 as u32,
s_name: String::from("clear_image_stitch"),
s_nicename: String::from("Clear image stitch"),
s_description: String::from("clears the current image stitching result, no worries it will be saved!")
                }
                ;
                pub static ref o_input_action__finish_focus_stack: O_input_action = O_input_action{
                    n_u32_id: 30 as u32,
s_name: String::from("finish_focus_stack"),
s_nicename: String::from("Finish focus stack"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__toggle_record_video: O_input_action = O_input_action{
                    n_u32_id: 31 as u32,
s_name: String::from("toggle_record_video"),
s_nicename: String::from("Record/Stop video"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__image_brightness_plus: O_input_action = O_input_action{
                    n_u32_id: 32 as u32,
s_name: String::from("image_brightness_plus"),
s_nicename: String::from("Image brightness plus"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__image_brightness_minus: O_input_action = O_input_action{
                    n_u32_id: 33 as u32,
s_name: String::from("image_brightness_minus"),
s_nicename: String::from("Image brightness minus"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__image_contrast_plus: O_input_action = O_input_action{
                    n_u32_id: 34 as u32,
s_name: String::from("image_contrast_plus"),
s_nicename: String::from("Image brightness plus"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__image_contrast_minus: O_input_action = O_input_action{
                    n_u32_id: 35 as u32,
s_name: String::from("image_contrast_minus"),
s_nicename: String::from("Image brightness minus"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__image_gamma_plus: O_input_action = O_input_action{
                    n_u32_id: 36 as u32,
s_name: String::from("image_gamma_plus"),
s_nicename: String::from("Image brightness plus"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__image_gamma_minus: O_input_action = O_input_action{
                    n_u32_id: 37 as u32,
s_name: String::from("image_gamma_minus"),
s_nicename: String::from("Image brightness minus"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_exposureMode_next: O_input_action = O_input_action{
                    n_u32_id: 38 as u32,
s_name: String::from("usb_camera_exposureMode_next"),
s_nicename: String::from("auto, manual"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_exposureMode_previous: O_input_action = O_input_action{
                    n_u32_id: 39 as u32,
s_name: String::from("usb_camera_exposureMode_previous"),
s_nicename: String::from("auto, manual"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_exposureTime_plus: O_input_action = O_input_action{
                    n_u32_id: 40 as u32,
s_name: String::from("usb_camera_exposureTime_plus"),
s_nicename: String::from("Time in seconds for exposure"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_exposureTime_minus: O_input_action = O_input_action{
                    n_u32_id: 41 as u32,
s_name: String::from("usb_camera_exposureTime_minus"),
s_nicename: String::from("Time in seconds for exposure"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_whiteBalanceMode_next: O_input_action = O_input_action{
                    n_u32_id: 42 as u32,
s_name: String::from("usb_camera_whiteBalanceMode_next"),
s_nicename: String::from("auto, manual"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_whiteBalanceMode_previous: O_input_action = O_input_action{
                    n_u32_id: 43 as u32,
s_name: String::from("usb_camera_whiteBalanceMode_previous"),
s_nicename: String::from("auto, manual"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_whiteBalance_plus: O_input_action = O_input_action{
                    n_u32_id: 44 as u32,
s_name: String::from("usb_camera_whiteBalance_plus"),
s_nicename: String::from("Color temperature in Kelvin"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_whiteBalance_minus: O_input_action = O_input_action{
                    n_u32_id: 44 as u32,
s_name: String::from("usb_camera_whiteBalance_minus"),
s_nicename: String::from("Color temperature in Kelvin"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_focusMode_next: O_input_action = O_input_action{
                    n_u32_id: 45 as u32,
s_name: String::from("usb_camera_focusMode_next"),
s_nicename: String::from("auto, manual"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_focusMode_previous: O_input_action = O_input_action{
                    n_u32_id: 46 as u32,
s_name: String::from("usb_camera_focusMode_previous"),
s_nicename: String::from("auto, manual"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_focusDistance_plus: O_input_action = O_input_action{
                    n_u32_id: 47 as u32,
s_name: String::from("usb_camera_focusDistance_plus"),
s_nicename: String::from("Distance for focus"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_focusDistance_minus: O_input_action = O_input_action{
                    n_u32_id: 48 as u32,
s_name: String::from("usb_camera_focusDistance_minus"),
s_nicename: String::from("Distance for focus"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_brightness_plus: O_input_action = O_input_action{
                    n_u32_id: 49 as u32,
s_name: String::from("usb_camera_brightness_plus"),
s_nicename: String::from("Brightness level"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_brightness_minus: O_input_action = O_input_action{
                    n_u32_id: 50 as u32,
s_name: String::from("usb_camera_brightness_minus"),
s_nicename: String::from("Brightness level"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_contrast_minus: O_input_action = O_input_action{
                    n_u32_id: 51 as u32,
s_name: String::from("usb_camera_contrast_minus"),
s_nicename: String::from("Contrast level"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_contrast_plus: O_input_action = O_input_action{
                    n_u32_id: 52 as u32,
s_name: String::from("usb_camera_contrast_plus"),
s_nicename: String::from("Contrast level"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_saturation_minus: O_input_action = O_input_action{
                    n_u32_id: 53 as u32,
s_name: String::from("usb_camera_saturation_minus"),
s_nicename: String::from("Saturation level"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_saturation_plus: O_input_action = O_input_action{
                    n_u32_id: 54 as u32,
s_name: String::from("usb_camera_saturation_plus"),
s_nicename: String::from("Saturation level"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_sharpness_minus: O_input_action = O_input_action{
                    n_u32_id: 55 as u32,
s_name: String::from("usb_camera_sharpness_minus"),
s_nicename: String::from("Sharpness level"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_sharpness_plus: O_input_action = O_input_action{
                    n_u32_id: 56 as u32,
s_name: String::from("usb_camera_sharpness_plus"),
s_nicename: String::from("Sharpness level"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_zoom_minus: O_input_action = O_input_action{
                    n_u32_id: 57 as u32,
s_name: String::from("usb_camera_zoom_minus"),
s_nicename: String::from("Zoom level"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_zoom_plus: O_input_action = O_input_action{
                    n_u32_id: 58 as u32,
s_name: String::from("usb_camera_zoom_plus"),
s_nicename: String::from("Zoom level"),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_torch_next: O_input_action = O_input_action{
                    n_u32_id: 58 as u32,
s_name: String::from("usb_camera_torch_next"),
s_nicename: String::from("Boolean for turning on/off the torch (if supported ) "),
s_description: String::from("")
                }
                ;
                pub static ref o_input_action__usb_camera_torch_previous: O_input_action = O_input_action{
                    n_u32_id: 59 as u32,
s_name: String::from("usb_camera_torch_previous"),
s_nicename: String::from("Boolean for turning on/off the torch (if supported ) "),
s_description: String::from("")
                }
                ;
pub static ref a_o_microscope_brand: Vec<&'static O_microscope_brand> = vec![
                &*o_microscope_brand__GenericBrand,
&*o_microscope_brand__Olympus,
&*o_microscope_brand__Bresser,
&*o_microscope_brand__Nikon,
&*o_microscope_brand__Zeiss,
&*o_microscope_brand__Leica
            ];pub static ref a_o_microscope_objective: Vec<&'static O_microscope_objective> = vec![
                &*o_microscope_objective__4x_Objective,
&*o_microscope_objective__10x_Objective,
&*o_microscope_objective__20x_Objective,
&*o_microscope_objective__40x_Objective
            ];pub static ref a_o_microscope: Vec<&'static O_microscope> = vec![
                &*o_microscope__ModelX
            ];pub static ref a_o_image: Vec<&'static O_image> = vec![
                
            ];pub static ref a_o_image_description: Vec<&'static O_image_description> = vec![
                
            ];pub static ref a_o_vec2: Vec<&'static O_vec2> = vec![
                
            ];pub static ref a_o_spacial_information_nor: Vec<&'static O_spacial_information_nor> = vec![
                
            ];pub static ref a_o_input_action: Vec<&'static O_input_action> = vec![
                &*o_input_action__move_slide_x_plus,
&*o_input_action__move_slide_x_minus,
&*o_input_action__move_slide_y_plus,
&*o_input_action__move_slide_y_minus,
&*o_input_action__move_focus_plus,
&*o_input_action__move_focus_minus,
&*o_input_action__move_slide_single_step_x_plus,
&*o_input_action__move_slide_single_step_x_minus,
&*o_input_action__move_slide_single_step_y_plus,
&*o_input_action__move_slide_single_step_y_minus,
&*o_input_action__keep_hold_down_toggle_layer2,
&*o_input_action__keep_hold_down_toggle_layer3,
&*o_input_action__toggle_settings,
&*o_input_action__next_image_mode,
&*o_input_action__previous_image_mode,
&*o_input_action__reset_image_manipulation,
&*o_input_action__move_digital_x_plus,
&*o_input_action__move_digital_x_minus,
&*o_input_action__move_digital_y_plus,
&*o_input_action__move_digital_y_minus,
&*o_input_action__zoom_digital_plus,
&*o_input_action__zoom_digital_minus,
&*o_input_action__next_action_layer,
&*o_input_action__prev_action_layer,
&*o_input_action__take_screenshot,
&*o_input_action__ask_ai,
&*o_input_action__add_image_to_focus_stack,
&*o_input_action__add_image_to_image_stitch,
&*o_input_action__clear_image_stitch,
&*o_input_action__finish_focus_stack,
&*o_input_action__toggle_record_video,
&*o_input_action__image_brightness_plus,
&*o_input_action__image_brightness_minus,
&*o_input_action__image_contrast_plus,
&*o_input_action__image_contrast_minus,
&*o_input_action__image_gamma_plus,
&*o_input_action__image_gamma_minus,
&*o_input_action__usb_camera_exposureMode_next,
&*o_input_action__usb_camera_exposureMode_previous,
&*o_input_action__usb_camera_exposureTime_plus,
&*o_input_action__usb_camera_exposureTime_minus,
&*o_input_action__usb_camera_whiteBalanceMode_next,
&*o_input_action__usb_camera_whiteBalanceMode_previous,
&*o_input_action__usb_camera_whiteBalance_plus,
&*o_input_action__usb_camera_whiteBalance_minus,
&*o_input_action__usb_camera_focusMode_next,
&*o_input_action__usb_camera_focusMode_previous,
&*o_input_action__usb_camera_focusDistance_plus,
&*o_input_action__usb_camera_focusDistance_minus,
&*o_input_action__usb_camera_brightness_plus,
&*o_input_action__usb_camera_brightness_minus,
&*o_input_action__usb_camera_contrast_minus,
&*o_input_action__usb_camera_contrast_plus,
&*o_input_action__usb_camera_saturation_minus,
&*o_input_action__usb_camera_saturation_plus,
&*o_input_action__usb_camera_sharpness_minus,
&*o_input_action__usb_camera_sharpness_plus,
&*o_input_action__usb_camera_zoom_minus,
&*o_input_action__usb_camera_zoom_plus,
&*o_input_action__usb_camera_torch_next,
&*o_input_action__usb_camera_torch_previous
            ];pub static ref a_o_image_object: Vec<&'static O_image_object> = vec![
                
            ];pub static ref a_o_image_description_o_image_object: Vec<&'static O_image_description_o_image_object> = vec![
                
            ];

}
