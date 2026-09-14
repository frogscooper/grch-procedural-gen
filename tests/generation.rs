use std::fs;
use grch_procedural_gen::*;
use grch_procedural_gen::types::*;

#[test]
fn generates_obj_successfully() {
    let path = "grch_export.obj";
    let _ = fs::remove_file(path);

    initbt(Point3(2048, 2048, 2048), 6);

    let metadata = fs::metadata(path)
        .expect("OBJ file was not created");

    assert!(
        metadata.len() > 0,
        "OBJ file was created but is empty"
    );
}

#[test]
fn deterministic_generation() {
    initbt(Point3(2048, 2048, 2048), 6);

    let first = fs::read("grch_export.obj")
        .expect("Failed to read first OBJ");

    initbt(Point3(2048, 2048, 2048), 6);

    let second = fs::read("grch_export.obj")
        .expect("Failed to read second OBJ");

    assert_eq!(first, second);
}

#[test]
fn obj_contains_geometry() {
    initbt(Point3(2048, 2048, 2048), 6);

    let obj = fs::read_to_string("grch_export.obj")
        .expect("Failed to read generated OBJ");
    
    assert!(
        obj.lines().any(|line| line.starts_with("v ")),
        "OBJ contains no vertices"
    );
    assert!(
        obj.lines().any(|line| line.starts_with("f ")),
        "OBJ contains no faces"
    );
}