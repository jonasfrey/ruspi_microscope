// class O_class{
//     constructor(
//         s_name, 
//         s_rs_prefix_string,
//         s_rs_name_prefix_string,
//         a_o_property
//     ){
//         this.s_name = s_name
//         this.s_rs_prefix_string = s_rs_prefix_string
//         this.s_rs_name_prefix_string = s_rs_name_prefix_string
//         this.a_o_property = a_o_property
//     }
// }
// class O_property{
//     constructor(
//         s_name, 
//         s_rs_prefix_string, 
//         s_rs_type
//     ){
//         this.s_name = s_name
//         this.s_rs_prefix_string = s_rs_prefix_string
//         this.s_rs_type = s_rs_type
//     }
// }
// let o_property_id_default = new O_property(
//     'n_id', 
//     'pub ', 
//     'u64'
// );
// let o_property_n_ts_ms_ut__created = new O_property(
//     'n_ts_ms_ut__created', 
//     'pub ', 
//     'u64'
// );
// let o_property_n_ts_ms_ut__updated = new O_property(
//     'n_ts_ms_ut__updated', 
//     'pub ', 
//     'u64'
// );
// let o_property_ai_estimated = new O_property(
//     'b_ai_estimated', 
//     `pub `, 
//     `bool`
// )
// let a_o_class = [
//     new O_class(
//         'O_image', 
//         `#[derive(Clone)]`, 
//         `pub struct`, 
//         [
//             o_property_id_default, 
//             new O_property(
//                 's_path_abs_file', 
//                 `pub `, 
//                 `String`
//             ), 
//             o_property_n_ts_ms_ut__created
//         ]
//     ), 
//     new O_class(
//         'O_image_description', 
//         `#[derive(Clone)]`, 
//         `pub struct`, 
//         [
//             o_property_id_default, 
//             new O_property(
//                 's_description', 
//                 `pub `, 
//                 `String`
//             ), 
//             new O_property(
//                 'n_micrometer_x_axis', 
//                 `pub `, 
//                 `f64`
//             ), 
//             o_property_ai_estimated
//         ]
//     ), 
//     new O_class(
//         'O_image_object', 
//         `#[derive(Clone)]`, 
//         `pub struct`, 
//         [
//             o_property_id_default, 
//             new O_property(
//                 's_name', 
//                 `pub `, 
//                 `String`
//             ), 
//             new O_property(
//                 'n_micrometer_x_axis', 
//                 `pub `, 
//                 `f64`
//             ), 
//             o_property_ai_estimated
//         ]
//     ),
// ]

// #[derive(Clone)]
// pub struct O_image_description{
//     pub n_id: u32, 
//     pub s_description: String, 
//     pub n_micrometer_x_axis: f64,
//     pub b_ai_estimated: bool
// }

// #[derive(Clone)]
// pub struct O_image_object{
//     pub n_id: u32, 
//     pub s_name: String,
//     pub s_description: String, 
//     pub n_id_o_spacial_information_nor: u64, 
//     pub b_ai_estimated: bool
// }

// #[derive(Clone)]
// pub struct O_image_description_o_image_object{
//     pub n_id: u32, 
//     pub n_id_o_image_description: u64, 
//     pub n_id_o_image_object: u64, 
//     pub n_ts_ms_ut__created: u64, 
//     pub n_ts_ms_ut__updated: u64
// }

// #[derive(Clone)]
// pub struct O_spacial_information_nor{
//     pub n_id: u32, 
//     pub o_trn: O_vec2,
//     pub o_scl: O_vec2,
//     pub n_ts_ms_ut__created: u64, 
//     pub n_ts_ms_ut__updated: u64
// }

// #[derive(Clone)]
// pub struct O_vec2{
//     pub n_x: f64,
//     pub n_y: f64,
// }

import { dirname as f_s_path_folder, fromFileUrl } from "https://deno.land/std@0.110.0/path/mod.ts";


let s_path_abs_file_current = f_s_path_folder(fromFileUrl(import.meta.url));
console.log(s_path_abs_file_current)
// Deno.exit()

class O_model_info{
    constructor(
        a_s_name_prop,
        a_a_v__default_data = []
    ){
        this.a_s_name_prop = a_s_name_prop,
        this.a_a_v__default_data = a_a_v__default_data
    }
}
let o_info = JSON.parse(await Deno.readTextFile('./o_info.json'));
let o_o_model_info = {}

let o_s_prefix_s_rust_type__exception = {
    's': 'String', 
    'b': 'bool', 
}
let f_n_idx_first_string_property = function(o_model_info){
    
    let s = o_model_info.a_s_name_prop.find(s=>s.startsWith(
        `s_`
    ));
    if(!s){
        return 0
    }
    return o_model_info.a_s_name_prop.indexOf(s);
}
let f_s_rust_type = function(s){
    let a_s_part = s.split("_")
    if(s.indexOf('n_') == 0){
        return a_s_part[1];
    }
    return o_s_prefix_s_rust_type__exception[a_s_part[0]];
}
let f_s_name_model = function(s){
    return s//s.split('__').slice(1).join('__');
}
let s_name_prop_id_default = 'n_u32_id';
let s_name_prop_n_ts_ms_ut__created = 'n_u64_ts_ms_ut__created';
let s_name_prop_n_ts_ms_ut__updated = 'n_u64_ts_ms_ut__updated';
let s_name_prop_s_name = 's_name'
let s_name_prop_s_description = `s_description`
let s_name_prop_b_ai_estimated = `b_ai_estimated`
let f_s_name_prop_id_foreign = function(o){
    let n_idx = Object.values(o_o_model_info).indexOf(o);
    let s_name_model = f_s_name_model(Object.keys(o_o_model_info)[n_idx])
    // console.log(Object.keys(o))
    return `${s_name_prop_id_default}__${s_name_model}`;
}

let n_ts_ms_ut__created = Date.now();

o_o_model_info.O_microscope_brand = new O_model_info(
    [
        s_name_prop_id_default,
        `s_name`, 
        s_name_prop_n_ts_ms_ut__created,
        s_name_prop_n_ts_ms_ut__updated,
    ], 
    [
        // default data
        [1, 'GenericBrand', n_ts_ms_ut__created, n_ts_ms_ut__created], 
        [2, 'Olympus', n_ts_ms_ut__created, n_ts_ms_ut__created],
        [3, 'Bresser', n_ts_ms_ut__created, n_ts_ms_ut__created],
        [4, 'Nikon', n_ts_ms_ut__created, n_ts_ms_ut__created],
        [5, 'Zeiss', n_ts_ms_ut__created, n_ts_ms_ut__created],
        [6, 'Leica', n_ts_ms_ut__created, n_ts_ms_ut__created],
    ]
)

o_o_model_info.O_microscope_objective = new O_model_info(
    [
        s_name_prop_id_default, 
        `s_name`, 
        `n_f64_magnification`, 
        `n_f64_numerical_aperture__dividend`,
        `n_f64_numerical_aperture__divisor`,
        `n_f64_focal_length`, 
        `n_f64_field_of_view`,
        `b_plan_apochromatic`,
        `s_tube_length`,
        s_name_prop_n_ts_ms_ut__created,
        s_name_prop_n_ts_ms_ut__updated,
    ],

    [
        //default data, 
        [1, '4x Objective', 4, 0.10, 1, 17, 4.5, false, '160mm', n_ts_ms_ut__created, n_ts_ms_ut__created],
        [2, '10x Objective', 10, 0.25, 1, 16, 2.0, false, '160mm', n_ts_ms_ut__created, n_ts_ms_ut__created],
        [3, '20x Objective', 20, 0.40, 1, 15, 1.0, true, '160mm', n_ts_ms_ut__created, n_ts_ms_ut__created],
        [4, '40x Objective', 40, 0.65, 1, 14, 0.5, true, '160mm', n_ts_ms_ut__created, n_ts_ms_ut__created],
    ]
)


o_o_model_info.O_microscope = new O_model_info(
    [
        s_name_prop_id_default, 
        `s_model`, 
        `${f_s_name_prop_id_foreign(o_o_model_info.O_microscope_brand)}`,
        s_name_prop_n_ts_ms_ut__created, 
    ],
    [
        // default data
        [1, 'ModelX', 1, n_ts_ms_ut__created]
    ]

)

o_o_model_info.O_image = new O_model_info(
    [
        s_name_prop_id_default, 
        's_path_abs_file',
        `${f_s_name_prop_id_foreign(o_o_model_info.O_microscope)}`, 
        `${f_s_name_prop_id_foreign(o_o_model_info.O_microscope_objective)}`, 
        s_name_prop_n_ts_ms_ut__created
    ],
    []
)


o_o_model_info.O_image_description = new O_model_info(
    [
        s_name_prop_id_default, 
        s_name_prop_s_description,
        `n_f64_micrometer_x_axis`,
        s_name_prop_b_ai_estimated,
        `${f_s_name_prop_id_foreign(o_o_model_info.O_image)}`, 
        s_name_prop_n_ts_ms_ut__created, 
        s_name_prop_n_ts_ms_ut__updated
    ]
)


o_o_model_info.O_vec2 = new O_model_info(
    [
        'n_f64_x',
        `n_f64_y`
    ]
)

o_o_model_info.O_spacial_information_nor = new O_model_info(
    [
       s_name_prop_id_default, 
       `${f_s_name_prop_id_foreign(o_o_model_info.O_vec2)}__trn`,
       `${f_s_name_prop_id_foreign(o_o_model_info.O_vec2)}__scl`,
   ]
)


o_o_model_info.O_input_action =
	new O_model_info(
		[
			s_name_prop_id_default,
			`s_name`,
			`s_nicename`,
			`s_description`,
		],
		[
			[
                1,
				"move_slide_x_plus",
				`Slide right`,
				`Physically move the slide (only if the mic is motorized)`,
			],
			[
                2,
				"move_slide_x_minus",
				"Slide left",
				``,
			],
			[
                3,
				"move_slide_y_plus",
				"Slide up",
				``,
			],
			[
                4,
				"move_slide_y_minus",
				"Slide down",
				``,
			],
			[
                5,
				"move_focus_plus",
				"Focus in",
				``,
			],
			[
                6,
				"move_focus_minus",
				"Focus out",
				`Physically turn the focus know (only if the mic is motorized)`,
			],
			[
                7,
				"move_slide_single_step_x_plus",
				"Slide right (one-step)",
				`Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter`,
			],
            [
                8,
                "move_slide_single_step_x_minus",
                "Slide left (one-step)",
                "Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter"
            ],
            [
                9,
                "move_slide_single_step_y_plus",
                "Slide up (one-step)",
                "Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter"
            ],
            [
                10,
                "move_slide_single_step_y_minus",
                "Slide down (one-step)",
                "Moves the slide a single step of the stepper motor, which translates to approximately 2r*pi/(number_of_steps_per_rotation) = 0.007 millimeter"
            ],
            [
                11,
                "keep_hold_down_toggle_camera_control",
                "Shift to layer 2",
                "hold to to temporarily switch to a new layer"
            ],
            [
                12,
                "keep_hold_down_toggle_image_control",
                "Shift to layer 3",
                "hold to to temporarily switch to a new layer"
            ],
            [
                13,
                "toggle_settings",
                "Settings",
                "Press once to show the settings, press again to hide"
            ],
            [
                14,
                "next_image_mode",
                "Image mode: next",
                "Cycle throught the image modes"
            ],
            [
                15,
                "previous_image_mode",
                "Image mode: prev",
                ""
            ],
            [
                16,
                "reset_image_manipulation",
                "Image reset",
                ""
            ],
            [
                17,
                "move_digital_x_plus",
                "Digital right",
                ""
            ],
            [
                18,
                "move_digital_x_minus",
                "Digital left",
                ""
            ],
            [
                19,
                "move_digital_y_plus",
                "Digital up",
                ""
            ],
            [
                20,
                "move_digital_y_minus",
                "Digital down",
                "Perform a digital movement, may be handy for showing and pointing out interesting image regions"
            ],
            [
                21,
                "zoom_digital_plus",
                "Digital zoom in",
                ""
            ],
            [
                22,
                "zoom_digital_minus",
                "Digital zoom out",
                "Perform a digital zoom, may be handy for showing and pointing out interesting image regions"
            ],
            [
                23,
                "next_action_layer",
                "Layer: next",
                "Switch to the next action layer"
            ],
            [
                24,
                "prev_action_layer",
                "Layer: prev",
                "Switch to the previous action layer"
            ],
            [
                25,
                "take_screenshot",
                "Screenshot",
                "Take a simple screnshot"
            ],
            [
                26,
                "ask_ai",
                "AI-Help",
                "Let AI detect common image features and estimation of size dont yet fully trust it, its GPT Vision AI its not that good yet (but its fun for sure!!!)"
            ],
            [
                27,
                "add_image_to_focus_stack",
                "Add image to focus stack",
                ""
            ],
            [
                28,
                "add_image_to_image_stitch",
                "Add image to image stitch",
                "Add an image to the image stitching result which autmotically updates in the background after each addition of an image"
            ],
            [
                29,
                "clear_image_stitch",
                "Clear image stitch",
                "clears the current image stitching result, no worries it will be saved!"
            ],
            [
                30,
                "finish_focus_stack",
                "Finish focus stack",
                ""
            ],
            [
                31,
                "toggle_record_video",
                "Record/Stop video",
                ""
            ],
            [
                32,
                "image_brightness_plus",
                "Image brightness plus",
                ""
            ],
            [
                33,
                "image_brightness_minus",
                "Image brightness minus",
                ""
            ],
            [
                34,
                "image_contrast_plus",
                "Image brightness plus",
                ""
            ],
            [
                35,
                "image_contrast_minus",
                "Image brightness minus",
                ""
            ],
            [
                36,
                "image_gamma_plus",
                "Image brightness plus",
                ""
            ],
            [
                37,
                "image_gamma_minus",
                "Image brightness minus",
                ""
            ],
            [
                38,
                "usb_camera_exposureMode_next",
                `auto, manual`,
                ""    
            ],
            [
                39,
                "usb_camera_exposureMode_previous",
                `auto, manual`,
                ""    
            ],
            [
                40,
                "usb_camera_exposureTime_plus",
                `Time in seconds for exposure`,
                ""    
            ],
            [
                41,
                "usb_camera_exposureTime_minus",
                `Time in seconds for exposure`,
                ""    
            ],
            [
                42,
                "usb_camera_whiteBalanceMode_next",
                `auto, manual`,
                ""    
            ],
            [
                43,
                "usb_camera_whiteBalanceMode_previous",
                `auto, manual`,
                ""    
            ],
            [
                44,
                "usb_camera_whiteBalance_plus",
                `Color temperature in Kelvin`,
                ""    
            ],
            [
                44,
                "usb_camera_whiteBalance_minus",
                `Color temperature in Kelvin`,
                ""    
            ],
            [
                45,
                "usb_camera_focusMode_next",
                `auto, manual`,
                ""    
            ],
            [
                46,
                "usb_camera_focusMode_previous",
                `auto, manual`,
                ""    
            ],
            [
                47,
                "usb_camera_focusDistance_plus",
                `Distance for focus`,
                ""    
            ],
            [
                48,
                "usb_camera_focusDistance_minus",
                `Distance for focus`,
                ""    
            ],
            [
                49,
                "usb_camera_brightness_plus",
                `Brightness level`,
                ""    
            ],
            [
                50,
                "usb_camera_brightness_minus",
                `Brightness level`,
                ""    
            ],
            [
                51,
                "usb_camera_contrast_minus",
                `Contrast level`,
                ""    
            ],
            [
                52,
                "usb_camera_contrast_plus",
                `Contrast level`,
                ""    
            ],
            [
                53,
                "usb_camera_saturation_minus",
                `Saturation level`,
                ""    
            ],
            [
                54,
                "usb_camera_saturation_plus",
                `Saturation level`,
                ""    
            ],
            [
                55,
                "usb_camera_sharpness_minus",
                `Sharpness level`,
                ""    
            ],
            [
                56,
                "usb_camera_sharpness_plus",
                `Sharpness level`,
                ""    
            ],
            [
                57,
                "usb_camera_zoom_minus",
                `Zoom level`,
                ""    
            ],
            [
                58,
                "usb_camera_zoom_plus",
                `Zoom level`,
                ""    
            ],
            [
                58,
                "usb_camera_torch_next",
                `Boolean for turning on/off the torch (if supported ) `,
                ""    
            ],
            [
                59,
                "usb_camera_torch_previous",
                `Boolean for turning on/off the torch (if supported ) `,
                ""    
            ] 

		]
	);
    
o_o_model_info.O_image_object = new O_model_info(
    [
        s_name_prop_id_default, 
        s_name_prop_s_name, 
        s_name_prop_s_description, 
        `${f_s_name_prop_id_foreign(o_o_model_info.O_spacial_information_nor)}`,
        s_name_prop_b_ai_estimated
    ]   
)

o_o_model_info.O_image_description_o_image_object = new O_model_info(
    [
        s_name_prop_id_default, 
        f_s_name_prop_id_foreign(o_o_model_info.O_image_description), 
        f_s_name_prop_id_foreign(o_o_model_info.O_image_object),
        s_name_prop_n_ts_ms_ut__created
    ]
)

let a_s_name_ws_action = [
    "hello",
    "f_save_screenshot",
    "f_add_image_to_focus_stack",
    "f_create_focus_stack",
    "f_add_iamge_to_image_stitch",
    "f_update_image_stitching_result",
    "f_s_read_text_file",
    "f_b_write_s_text_file",
    "f_o_command",
    "f_control_stepper_motor",
    "f_switch_usb_device",
]

let s_js__classes = Object.keys(o_o_model_info).map(s=>{
    let s_name_model = f_s_name_model(s);
    let a_s = o_o_model_info[s].a_s_name_prop;

    return `
        class ${s_name_model}{
            constructor(
                ${a_s.map(s=>{
                    return `${s}`
                }).join(',\n')}
            ){
                ${a_s.map(s=>{
                    return `this.${s} = ${s}`
                }).join(',\n')}
            }
        }
    `
}).join('\n');
let s_js__classes_exports = `
export {
    ${Object.keys(o_o_model_info).map(s=>{
        return f_s_name_model(s);
    }).join(',\n')}
}
`
let a_s_var_s_name_ws_action = []
let s_js__other =  `
    ${o_info.a_o_name_synonym.map(o=>{
        return `let s_name_o_input_sensor__${o.s_name} = '${o.s_name}'`
    }).join(';\n')}
    let a_s_name_o_input_sensor = [
        ${
            o_info.a_o_name_synonym.map(o=>`s_name_o_input_sensor__${o.s_name}`).join(',\n')
        }
    ]
    ${a_s_name_ws_action.map(s=>{
        let s_name_var = `s_name_ws_action__${s}`
        a_s_var_s_name_ws_action.push(s_name_var)
        return `let ${s_name_var} = "${s}";`
    }).join('\n')}
    let a_s_name_ws_action = [
        ${a_s_var_s_name_ws_action.map(s=>s).join(',')}
    ];
    export { 
        a_s_name_ws_action,
        ${a_s_var_s_name_ws_action.map(s=>s).join(',')},
        a_s_name_o_input_sensor,
        ${
            o_info.a_o_name_synonym.map(o=>{
                return `s_name_o_input_sensor__${o.s_name}`
            }).join(',\n')
        }
    }
    
`
let a_s_name_var_all = []; 
let a_s_statement_instance_js = [];
let a_s_statement_instance_rs = [];
let a_s_statement_array_js = [];
let a_s_statement_array_rs = [];

for(let s of Object.keys(o_o_model_info)){
    let o = o_o_model_info[s];
    let a_s_name_var = [];
        o.a_a_v__default_data.map(a_v=>{
            let n_idx_first_string_property = f_n_idx_first_string_property(o);
            let s_prop = o.a_s_name_prop[n_idx_first_string_property];
            let v_value = a_v[n_idx_first_string_property];
            let s_name_var = `${s.toLowerCase()}__${v_value.replaceAll(' ', '_')}`
            a_s_name_var.push(s_name_var);
            a_s_name_var_all.push(s_name_var);
            a_s_statement_instance_js.push(`let ${s_name_var} = new ${s}(${
                a_v.map((v, n_idx)=>{
                    let s_name_prop = o.a_s_name_prop[n_idx];

                   return `${(s_name_prop.startsWith('s_')?
                    `"${v}"`:
                    v
                    )}`
                }).join(',\n')
            })`)
            a_s_statement_instance_rs.push(
                `
                pub static ref ${s_name_var}: ${s} = ${s}{
                    ${a_v.map((v, n_idx)=>{
                        let s_name_prop = o.a_s_name_prop[n_idx];
                        let s_rs_value = `${(s_name_prop.startsWith('s_')) 
                            ? `String::from("${v}")`
                            : `${v} as ${f_s_rust_type(s_name_prop)}`
                        }`;
                        return `${s_name_prop}: ${s_rs_value}`
                    }).join(',\n')}
                }
                `
            )
        });
        a_s_statement_array_js.push(
            `let a_${s.toLowerCase()} = [
                ${a_s_name_var.map(s=>{
                    return s
                }).join(',\n')}
            ]`
        )
        a_s_statement_array_rs.push(
            `pub static ref a_${s.toLowerCase()}: Vec<&'static ${s}> = vec![
                ${a_s_name_var.map(s=>{
                    return `&*${s}`
                }).join(',\n')}
            ]`
        )
}

let s_js__instances = `
${a_s_statement_instance_js.map(s=>s).join(';')}
${a_s_statement_array_js.map(s=>s).join(';')}
export {
    ${a_s_name_var_all.join(',\n')},
    ${Object.keys(o_o_model_info).map(s=>{
        return `a_${s.toLowerCase()}`
    }).join(',\n')}
}
`
let s_rs__instances = `

${a_s_statement_instance_rs.map(s=>s).join(';')};
${a_s_statement_array_rs.map(s=>s).join(';')};
`

let s_js =
`
${s_js__classes}
${s_js__classes_exports}
${s_js__other}
${s_js__instances}
`
await Deno.writeTextFile('./public/autogenerated.module.js', s_js)




// example rust struct
// #[derive(Clone)]
// pub struct O_image{
//     pub n_id: u32, 
//     pub s_path_abs_file: String,
//     pub n_ts_ms_ut__created: u64, 
//     pub n_ts_ms_ut__updated: u64
// }
console.log(o_info)

let s_rs__classes = Object.keys(o_o_model_info).map(s=>{
    let s_name_model = f_s_name_model(s);
    let a_s = o_o_model_info[s].a_s_name_prop;
    return `
    #[derive(Clone)]
    pub struct ${s_name_model}{
        ${a_s.map(s=>{
            return `pub ${s}: ${f_s_rust_type(s)}`
        }).join(',\n')}
    }`
}).join('\n');
let s_rs = `
use lazy_static::lazy_static;
${s_rs__classes}
lazy_static! {
    ${a_s_name_ws_action.map(s=>{
        let s_name_var = `s_name_ws_action__${s}`
        // pub static ref MY_STRING: String = String::from("Hello, world!");
        return `pub static ref ${s_name_var}: String = String::from("${s}");`
    }).join('\n')}
    ${s_rs__instances}
}
`


await Deno.writeTextFile('./src/bin/autogenerated.rs', s_rs)