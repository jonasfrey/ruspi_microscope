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
    O_input_action
  }