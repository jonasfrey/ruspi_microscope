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

let o_info = JSON.parse(await Deno.readTextFile('./o_info.json'));
let o_class_o_image = {}

let o_s_prefix_s_rust_type__exception = {
    's': 'String', 
    'b': 'bool'
}
let f_s_rust_type = function(s){
    let a_s_part = s.split("_")
    if(s.indexOf('n_') == 0){
        return a_s_part[1];
    }
    return o_s_prefix_s_rust_type__exception[a_s_part[0]];
}
let f_s_name_model = function(s){
    return s.split('__').slice(1).join('__');
}
let s_name_prop_id_default = 'n_u32_id';
let s_name_prop_n_ts_ms_ut__created = 'n_u64_ts_ms_ut__created';
let s_name_prop_n_ts_ms_ut__updated = 'n_u64_ts_ms_ut__updated';
let s_name_prop_s_name = 's_name'
let s_name_prop_s_description = `s_description`
let s_name_prop_b_ai_estimated = `b_ai_estimated`
let f_s_name_prop_id_foreign = function(o){
    let n_idx = Object.values(o_class_o_image).indexOf(o);
    let s_name_model = f_s_name_model(Object.keys(o_class_o_image)[n_idx])
    // console.log(Object.keys(o))
    return `${s_name_prop_id_default}__${s_name_model}`;
}

o_class_o_image.a_s_name_prop__O_microscope_brand = [
    s_name_prop_id_default,
    `s_name`, 
    s_name_prop_n_ts_ms_ut__created,
    s_name_prop_n_ts_ms_ut__updated,
]

o_class_o_image.a_s_name_prop__O_microscope_objective = [
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
]; 

o_class_o_image.a_s_name_prop__O_microscope = [
    s_name_prop_id_default, 
    `s_model`, 
    `${f_s_name_prop_id_foreign(o_class_o_image.a_s_name_prop__O_microscope_brand)}`,
    s_name_prop_n_ts_ms_ut__created
]

o_class_o_image.a_s_name_prop__O_image = [
    s_name_prop_id_default, 
    's_path_abs_file',
    `${f_s_name_prop_id_foreign(o_class_o_image.a_s_name_prop__O_microscope)}`, 
    `${f_s_name_prop_id_foreign(o_class_o_image.a_s_name_prop__O_microscope_objective)}`, 
    s_name_prop_n_ts_ms_ut__created
]

o_class_o_image.a_s_name_prop__O_image_description = [
    s_name_prop_id_default, 
    s_name_prop_s_description,
    `n_f64_micrometer_x_axis`,
    s_name_prop_b_ai_estimated,
    `${f_s_name_prop_id_foreign(o_class_o_image.a_s_name_prop__O_image)}`, 
    s_name_prop_n_ts_ms_ut__created, 
    s_name_prop_n_ts_ms_ut__updated
]


o_class_o_image.a_s_name_prop__O_vec2 = [
    'n_f64_x',
    `n_f64_y`
]
o_class_o_image.a_s_name_prop__O_spacial_information_nor = [
    s_name_prop_id_default, 
    `${f_s_name_prop_id_foreign(o_class_o_image.a_s_name_prop__O_vec2)}__trn`,
    `${f_s_name_prop_id_foreign(o_class_o_image.a_s_name_prop__O_vec2)}__scl`,
]



o_class_o_image.a_s_name_prop__O_image_object = [
    s_name_prop_id_default, 
    s_name_prop_s_name, 
    s_name_prop_s_description, 
    `${f_s_name_prop_id_foreign(o_class_o_image.a_s_name_prop__O_spacial_information_nor)}`,
    s_name_prop_b_ai_estimated
]

o_class_o_image.a_s_name_prop__O_image_description_o_image_object = [
    s_name_prop_id_default, 
    f_s_name_prop_id_foreign(o_class_o_image.a_s_name_prop__O_image_description), 
    f_s_name_prop_id_foreign(o_class_o_image.a_s_name_prop__O_image_object),
    s_name_prop_n_ts_ms_ut__created
]

let s_js__classes = Object.keys(o_class_o_image).map(s=>{
    let s_name_model = f_s_name_model(s);
    let a_s = o_class_o_image[s];
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
    ${Object.keys(o_class_o_image).map(s=>{
        return f_s_name_model(s);
    }).join(',\n')}
}
`
let s_js__other =  `
    ${o_info.a_o_name_synonym.map(o=>{
        return `let s_name_o_input_sensor__${o.s_name} = '${o.s_name}'`
    }).join(';\n')}
    let a_s_name_o_input_sensor = [
        ${
            o_info.a_o_name_synonym.map(o=>`s_name_o_input_sensor__${o.s_name}`).join(',\n')
        }
    ]
    export { 
        a_s_name_o_input_sensor,
        ${
            o_info.a_o_name_synonym.map(o=>{
                return `s_name_o_input_sensor__${o.s_name}`
            }).join(',\n')
        }
    }
    
`

let s_js =
`
${s_js__classes}
${s_js__classes_exports}
${s_js__other}
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

let s_rs = Object.keys(o_class_o_image).map(s=>{
    let s_name_model = f_s_name_model(s);
    let a_s = o_class_o_image[s];
    return `
    #[derive(Clone)]
    pub struct ${s_name_model}{
        ${a_s.map(s=>{
            return `pub ${s}: ${f_s_rust_type(s)}`
        }).join(',\n')}
    }
    `
}).join('\n');


await Deno.writeTextFile('./src/bin/autogenerated.module.rs', s_rs)