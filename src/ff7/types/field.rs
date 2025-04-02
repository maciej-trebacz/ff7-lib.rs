use serde::Serialize;

#[derive(Serialize)]
pub struct FieldModel {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub direction: u8,
    pub triangle: u16,
    pub collision: u8,
    pub interaction: u8,
    pub visible: u8,
}

#[derive(Serialize)]
pub struct FieldData {
    pub field_id: u16,
    pub field_name: Vec<u8>,
    pub field_model_count: u16,
    pub field_model_names: Vec<String>,
}

#[derive(Serialize)]
pub struct FieldLineObj {
    pub x1: i16,
    pub y1: i16,
    pub z1: i16,
    pub x2: i16,
    pub y2: i16,
    pub z2: i16,
    pub enabled: u8,
    pub entity: u8,
}