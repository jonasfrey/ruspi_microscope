let o_state = {}

Object.assign(
    o_state, 
    {
      o_js__a_o_input_mapping: {
        f_o_jsh: ()=>{
          let f_o_jsh = function(o, s_name){
            console.log(o)
            let v_o_input_font = o_state.a_o_input_font_icon.find(
              o2=>{
                return o2.s_name_input == o.s_name_input__controller
              }
            )
            if(!s_name){
              s_name = o.s_name
            }
            return {
              class: "hoverable",
              style: "display:flex; flex-direction:row;justify-content:space-between;align-items:center",
              a_o: [
                {
                  innerText: s_name
                }, 
                {
                  onclick: ()=>{
                    o_state.o_config.o_input_action = o;
                    o_state.o_js__a_o_input_mapping?._f_render?.();
                  },
                  s_tag: "button", 
                style: "display:flex; flex-direction:row;justify-content:space-between;align-items:center",
                  a_o: [
                    {
                      class: 'keyboard_char',
                      innerText: (o_state.o_config.o_input_action == o ) ? '?': o.s_name_char_keyboard.toUpperCase()
                    },
                    {
                      innerHTML: "&nbsp;&nbsp;&nbsp;"
                    },
                    {
                      class: f_s_class__from_s_name_font(v_o_input_font?.s_name_font),
                      innerText: (o_state.o_config.o_input_action == o ) ? '?':  v_o_input_font?.s_char
                    }
                  ]
                }
                // f_o_jsh__keyicons(o.s_name_input)
              ]
            }
          }
          
          return {
            class: 'a_o_input_mapping', 
            a_o: [
              {
                innerText: "Slide movement", 
              },
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_slide_x_plus'
                ),
                'Right'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_slide_x_minus'
                ),
                'Left'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_slide_y_plus'
                ),
                'Up'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_slide_y_minus'
                ),
                'Down'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_slide_single_step_x_plus'
                ),
                'Right (single step)'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_slide_single_step_x_minus'
                ),
                'Left  (single step)'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_slide_single_step_y_plus'
                ),
                'Up  (single step)'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_slide_single_step_y_minus'
                ),
                'Down  (single step)'
              ),
              {
                innerText: "Focus movement", 
              },
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_focus_plus'
                ),
                'In'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_focus_minus'
                ),
                'Out'
              ), 
              {
                innerText: "Camera control Layer", 
              },

              {
                innerText: "Image control Layer", 
              },
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='next_image_mode'
                ),
                'Next image mode'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='previous_image_mode'
                ),
                'Previous image mode'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_digital_x_plus'
                ),
                'Right'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_digital_x_minus'
                ),
                'Left'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_digital_y_plus'
                ),
                'Up'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='move_digital_y_minus'
                ),
                'Down'
              ),
              {
                innerText: "Digital Zoom", 
              },
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='zoom_digital_plus'
                ),
                'In'
              ),
              f_o_jsh(
                o_state.o_config.a_o_input_action.find(o=>
                  o.s_name=='zoom_digital_minus'
                ),
                'Out'
              ), 
              {
                innerText: "Image mode", 
              },
              
              // ...o_state.o_config.a_o_input_action.map(o=>)
            ]
          }
        }
      }
    }
  ).o_js__a_o_input_mapping;