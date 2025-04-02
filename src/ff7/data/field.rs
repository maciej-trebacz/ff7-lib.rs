use crate::ff7::addresses::FF7Addresses;
use crate::ff7::types::field::{FieldData, FieldModel, FieldLineObj};
use crate::utils::memory::*;

pub fn read_field_models(addresses: &FF7Addresses) -> Result<Vec<FieldModel>, String> {
    let mut models: Vec<FieldModel> = Vec::new();

    let models_num = read_memory_byte(addresses.field_num_models)? as u32;
    let model_ptr = read_memory_int(addresses.field_models_ptr)?;
    if model_ptr == 0 {
        return Ok(models);
    }

    for i in 0..models_num {
        let base_address = model_ptr + i * 400;
        let base_address_2 = addresses.field_models_objs + i * 0x88;
        let model = FieldModel {
            x: read_memory_signed_int(base_address + 4)?,
            y: read_memory_signed_int(base_address + 8)?,
            z: read_memory_signed_int(base_address + 0xc)? + 10, // field model Z coords are offset by 10
            direction: read_memory_byte(base_address + 0x1c)?,
            triangle: read_memory_short(base_address_2 + 0x78)?,
            collision: read_memory_byte(base_address_2 + 0x5f)?,
            interaction: read_memory_byte(base_address_2 + 0x61)?,
            visible: read_memory_byte(base_address_2 + 0x62)?,
        };
        models.push(model);
    }
    Ok(models)
}

pub fn read_field_data(addresses: &FF7Addresses) -> Result<FieldData, String> {
    let field_id = read_memory_short(addresses.field_id)?;
    let field_name = read_memory_buffer(addresses.field_name, 16)?;
    let field_data_addr = read_memory_int(addresses.field_data_ptr)?;
    if field_data_addr == 0 {
        return Ok(FieldData {
            field_id,
            field_name,
            field_model_count: 0,
            field_model_names: Vec::new(),
        });
    }

    let section3_offset = read_memory_int(field_data_addr + 0x0e)?;
    let section3_addr = field_data_addr + section3_offset + 4;
    let field_model_count = read_memory_short(section3_addr + 2)?;
    let models_addr = section3_addr + 6;

    let mut field_model_names = Vec::new();
    let mut offset = 0;
    for _i in 0..field_model_count {
        let model_name_size = read_memory_short(models_addr + offset)?;
        let model_name = read_memory_buffer(models_addr + offset + 2, model_name_size as usize)?;
        let model_animation_count =
            read_memory_short(models_addr + offset + model_name_size as u32 + 16)?;
        field_model_names.push(String::from_utf8(model_name).unwrap_or(String::from("???")));
        offset += model_name_size as u32 + 48;

        for _j in 0..model_animation_count {
            let animation_name_size = read_memory_short(models_addr + offset)?;
            offset += animation_name_size as u32 + 4;
        }
    }

    Ok(FieldData {
        field_id,
        field_name,
        field_model_count,
        field_model_names,
    })
}

pub fn read_field_line_objs(addresses: &FF7Addresses) -> Result<Vec<FieldLineObj>, String> {
    let mut field_line_objs = Vec::new();

    for i in 0..32 {
        let base_address = addresses.field_line_objs + i * 0x18;
        let field_line_obj = FieldLineObj {
            x1: read_memory_signed_short(base_address + 0x00)?,
            y1: read_memory_signed_short(base_address + 0x02)?,
            z1: read_memory_signed_short(base_address + 0x04)?,
            x2: read_memory_signed_short(base_address + 0x06)?,
            y2: read_memory_signed_short(base_address + 0x08)?,
            z2: read_memory_signed_short(base_address + 0x0a)?,
            enabled: read_memory_byte(base_address + 0x0c)?,
            entity: read_memory_byte(base_address + 0x0d)?,
        };
        if field_line_obj.entity > 0 {
            field_line_objs.push(field_line_obj);
        }
    }
    Ok(field_line_objs)
}
