use serde::Serialize;

#[derive(Serialize)]
pub struct WorldModel {
    pub index: u8,
    pub x: u32,
    pub y: i32,
    pub z: u32,
    pub direction: i16,
    pub model_id: u8,
    pub walkmesh_type: u8,
    pub location_id: u8,
    pub chocobo_tracks: bool,
}

/*
short	x
short	y
short	triangleId
ushort	fieldId
byte	direction
byte[3]	padding
*/
#[derive(Serialize)]
pub struct WorldFieldTblItem {
    pub x: i16,
    pub y: i16,
    pub triangle_id: u16,
    pub field_id: u16,
    pub direction: u8,
}
