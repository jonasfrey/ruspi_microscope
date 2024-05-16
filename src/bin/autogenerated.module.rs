
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
    