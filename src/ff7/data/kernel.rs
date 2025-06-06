use crate::ff7::addresses::FF7Addresses;
use crate::ff7::types::kernel::ItemData;
use crate::utils::memory::*;
use crate::utils::flip_bits;

pub fn read_name(address: u32, max_length: u32) -> Result<String, String> {
    let mut name = Vec::new();
    let mut i = 0;
    while i < max_length {
        let byte = read_memory_byte(address + i)?;
        if byte == 0xFF {
            break;
        }
        name.push(byte);
        i += 1;
    }
    let decoded_name = crate::ff7::ff7text::decode_text(&name);
    Ok(decoded_name.unwrap_or_else(|_| String::from_utf8(name).unwrap()))
}

fn read_kernel_section(
    items: &mut Vec<String>,
    addresses: &FF7Addresses,
    section_id: u32,
    count: u32,
) -> Result<(), String> {
    let mut addr = addresses.kernel_texts_base;
    let ffnx_check = read_memory_int(addr)? as u32;

    if ffnx_check == 0 {
        let kernel_read_fn_addr = read_memory_int(addresses.kernel_read_fn_call)? as u32
            + addresses.kernel_read_fn_call
            + 4;
        let kernel_sections_tbl = read_memory_int(kernel_read_fn_addr + 0x1B)? as u32;
        addr = read_memory_int(kernel_sections_tbl + (4 * section_id))? as u32;
    } else {
        addr += read_memory_short(addresses.kernel_section_offsets + (2 * section_id))? as u32;
    }

    for i in 0..count {
        let offset = read_memory_short(addr + i * 2)? as u16;
        let address = addr + offset as u32;
        let name = read_name(address, 24).unwrap_or_else(|_| String::from("???"));
        items.push(name);
    }

    Ok(())
}

pub fn read_item_names(addresses: &FF7Addresses) -> Result<Vec<String>, String> {
    let mut items: Vec<String> = Vec::new();

    // Items
    read_kernel_section(&mut items, addresses, 10, 128)?;

    // Weapons
    read_kernel_section(&mut items, addresses, 11, 128)?;

    // Armors
    read_kernel_section(&mut items, addresses, 12, 32)?;

    // Accessories
    read_kernel_section(&mut items, addresses, 13, 32)?;

    Ok(items)
}

pub fn read_materia_names(addresses: &FF7Addresses) -> Result<Vec<String>, String> {
    let mut materia: Vec<String> = Vec::new();
    read_kernel_section(&mut materia, addresses, 14, 96)?;
    Ok(materia)
}

pub fn read_key_item_names(addresses: &FF7Addresses) -> Result<Vec<String>, String> {
    let mut items: Vec<String> = Vec::new();
    read_kernel_section(&mut items, addresses, 15, 64)?;
    Ok(items)
}

pub fn read_command_names(addresses: &FF7Addresses) -> Result<Vec<String>, String> {
    let mut items: Vec<String> = Vec::new();
    read_kernel_section(&mut items, addresses, 8, 32)?;
    Ok(items)
}

pub fn read_attack_names(addresses: &FF7Addresses) -> Result<Vec<String>, String> {
    let mut items: Vec<String> = Vec::new();
    read_kernel_section(&mut items, addresses, 9, 128)?;
    Ok(items)
}

pub fn read_item_data(addresses: &FF7Addresses) -> Result<Vec<ItemData>, String> {
    let mut items: Vec<ItemData> = Vec::new();
    let addr = addresses.kernel_item_data;
    for i in 0..128 {
        let item_addr = addr + i * 28;
        let item_data = ItemData {
            camera_move_id: read_memory_short(item_addr + 0x08)? as u16,
            restriction_mask: read_memory_short(item_addr + 0x0a)? as u16,
            target_flags: read_memory_byte(item_addr + 0x0c)? as u8,
            attack_effect_id: read_memory_byte(item_addr + 0x0d)? as u8,
            damage_func: read_memory_byte(item_addr + 0x0e)? as u8,
            power: read_memory_byte(item_addr + 0x0f)? as u8,
            condition: read_memory_byte(item_addr + 0x10)? as u8,
            status_effect_change: flip_bits(read_memory_byte(item_addr + 0x11)? as u8),
            attack_additional_effect: flip_bits(read_memory_byte(item_addr + 0x12)? as u8),
            additional_effect_modifier: flip_bits(read_memory_byte(item_addr + 0x13)? as u8),
            status_effects: flip_bits(read_memory_int(item_addr + 0x14)? as u32),
            attack_element: read_memory_short(item_addr + 0x18)? as u16,
            special_attack_flags: flip_bits(read_memory_short(item_addr + 0x1a)? as u16),
        };
        items.push(item_data);
    }
    Ok(items)
}
