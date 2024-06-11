class O_keyboard_key{
    constructor(
      s_name,
      n_ms_wpn_down, 
      n_ms_wpn_up, 
      b_down, 
      b_down__last
    ){
      this.s_name = s_name,
      this.n_ms_wpn_down = n_ms_wpn_down, 
      this.n_ms_wpn_up = n_ms_wpn_up, 
      this.b_down = b_down
      this.b_down__last = b_down__last 
    }
  }

  class O_video_capability{
    constructor(
      s_name, 
      v_n_min, 
      v_n_max, 
      v_n_step, 
      v_a_v_option,
      v_n_current_value, 
      v_n_idx_current_value
    ){
      this.s_name = s_name
      this.v_n_min = v_n_min
      this.v_n_max = v_n_max
      this.v_n_step = v_n_step
      this.v_a_v_option = v_a_v_option
      this.v_n_current_value = v_n_current_value
      this.v_n_idx_current_value = v_n_idx_current_value
    }
  }
  class O_resolution{
    constructor(
      n_scl_x_px,
      n_scl_y_px, 
      s_name
    ){
      this.n_scl_x_px = n_scl_x_px
      this.n_scl_y_px = n_scl_y_px
      this.s_name = s_name
    }
  }
  class O_camera{
    constructor(
      s_deviceId, // eg "55f0f7a4eefe176b2bada32b43c0426c001583656fc094aedbead085e4d8cbd4"
      s_groupId, // eg "637242ea10febbf55cf50000e39f3b026fec7ecc120420485e659f2b98534760"
      s_kind,// eg. "videoinput"
      s_label, // eg "HY-500B (0ac8:3420)"
      a_o_video_capability, 
      a_o_resolution, 
      n_idx_a_o_resolution
    ){
      this.s_deviceId = s_deviceId
      this.s_groupId = s_groupId
      this.s_kind = s_kind
      this.s_label = s_label
      this.a_o_video_capability = a_o_video_capability
      this.a_o_resolution = a_o_resolution
      this.n_idx_a_o_resolution = n_idx_a_o_resolution
    }
  }
  
  
  class O_input_font_icon{
    constructor(
      s_name_input, 
      s_char, 
      s_name_font
    ){
      this.s_name_input = s_name_input, 
      this.s_char = s_char, 
      this.s_name_font = s_name_font
    }
  }
  class O_input_action_mapping{
    constructor(
      o_input_action, 
      s_name_input__controller, 
      s_name_char_keyboard, 
      v_b_invert_axis
    ){
      this.o_input_action = o_input_action, 
      this.s_name_input__controller = s_name_input__controller, 
      this.s_name_char_keyboard = s_name_char_keyboard, 
      this.v_b_invert_axis = v_b_invert_axis
      this.v_o_keyboard_key = null
      this.v_o_input_sensor = null
      this.v_o_input_sensor_last = null
    }
  }
  class O_input_action{
    constructor(
      s_name, 
      s_nicename,
      s_description
    ){
        this.s_name = s_name
        this.s_nicename = s_nicename
        this.s_description = s_description
    }
  }


  
  export {
    O_keyboard_key,
    O_input_font_icon,
    O_input_action_mapping,
    O_input_action, 
    O_camera,
    O_video_capability, 
    O_resolution
  }