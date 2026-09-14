use crate::types::*;
// use crate::edges::*;

pub fn create_obj() -> String {
    return String::from("#OBJ file exported by grch-procedural-gen\n o grchDungeon\n")
}

pub fn add_obj_box(c1: Point3, c2: Point3, obj_data: &mut String, v: usize) -> usize {
    //Add vertices
    obj_data.push_str(&format!("v {} {} {}\n", c1.0, c1.1, c1.2)); //1
    obj_data.push_str(&format!("v {} {} {}\n", c2.0, c1.1, c1.2)); //2
    obj_data.push_str(&format!("v {} {} {}\n", c2.0, c2.1, c1.2)); //3 
    obj_data.push_str(&format!("v {} {} {}\n", c2.0, c2.1, c2.2)); //4
    obj_data.push_str(&format!("v {} {} {}\n", c1.0, c2.1, c2.2)); //5
    obj_data.push_str(&format!("v {} {} {}\n", c2.0, c1.1, c2.2)); //6
    obj_data.push_str(&format!("v {} {} {}\n", c1.0, c2.1, c1.2)); //7
    obj_data.push_str(&format!("v {} {} {}\n", c1.0, c1.1, c2.2));

    
    let o = v;

    //Add faces
    obj_data.push_str(&format!("f {} {} {}\n", o+1, o+7, o+5));
    obj_data.push_str(&format!("f {} {} {}\n", o+1, o+5, o+8)); // Left g
    obj_data.push_str(&format!("f {} {} {}\n", o+1, o+7, o+3));
    obj_data.push_str(&format!("f {} {} {}\n", o+1, o+3, o+2)); // Front g
    obj_data.push_str(&format!("f {} {} {}\n", o+2, o+3, o+4));
    obj_data.push_str(&format!("f {} {} {}\n", o+2, o+4, o+6)); // Right g
    obj_data.push_str(&format!("f {} {} {}\n", o+1, o+8, o+6));
    obj_data.push_str(&format!("f {} {} {} \n", o+1, o+6, o+2));  // Bottom g
    obj_data.push_str(&format!("f {} {} {}\n", o+7, o+5, o+4));
    obj_data.push_str(&format!("f {} {} {}\n", o+7, o+4, o+3)); // Top g
    obj_data.push_str(&format!("f {} {} {}\n", o+8, o+5, o+4));
    obj_data.push_str(&format!("f {} {} {}\n", o+8, o+4, o+6)); // Back gG

    return o+8;
}

