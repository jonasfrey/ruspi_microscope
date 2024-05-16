

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
    export { 
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
    

