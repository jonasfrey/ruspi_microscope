let f_b_create = function(
    a_o, 
    o, 
    s_name_prop_id = 'n_u32_id'
){
    let v_n_id = o?.[s_name_prop_id];
    let v_o = a_o.find(o=>v_n_id == o?.[s_name_prop_id]);
    if(v_o){
        throw Error(`object with id ${v_n_id} already exists ${v_o}`)
    }
    a_o.push(o);
    return true;
}
let f_b_read = function(){
    // simply a a_o.find() or a_o.filter()
}
let f_b_update = function(
    a_o, 
    o, 
    s_name_prop_id = 'n_u32_id'

){
    let v_n_id = o?.[s_name_prop_id];
    let v_n_idx_found = null;
    let v_o = a_o.find((o, n_idx)=>{
        if(v_n_id == o?.[s_name_prop_id]){
            v_n_idx_found = n_idx;
            return true
        }
    });
    if(!v_n_idx_found){
        throw Error(`cannot update, object with id ${v_n_id} not found in array ${v_o}`)
    }
    a_o[v_n_idx_found] = o;
}

let f_s_name_file_from_s_name_array = function(
    s_name_array
){
    return `${s_name_array}.json`
}


let f_a_o_read_from_file = async function(
    f_s_read_text_file,
    s_name_array
){
    let s_name_file = f_s_name_file_from_s_name_array(s_name_array)
    let a_o = [];
    try {
        let v_s_json = await f_s_read_text_file(
            s_name_file
        );
        a_o = JSON.parse(v_s_json);
    } catch (error) {
        a_o = []
    }
    return a_o
}
let f_b_create_to_file = async function(
    f_s_read_text_file,
    f_b_write_text_file,
    s_name_array, 
    o, 
    s_name_prop_id = 'n_u32_id'
){
    let s_name_file = f_s_name_file_from_s_name_array(s_name_array)

    let a_o = await f_a_o_read_from_file(
        f_s_read_text_file,
        s_name_array
    );
    f_b_create(
        a_o, 
        o,
        s_name_prop_id
    );
    return await f_b_write_text_file(
        s_name_file, 
        JSON.stringify(a_o, null, 4)
    );
}