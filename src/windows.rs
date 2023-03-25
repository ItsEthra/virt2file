use eyre::{eyre, Result};
use goblin::pe::{section_table::SectionTable, PE};

pub fn output(
    file: &PE,
    addresses: &[usize],
    reverse: bool,
    base: Option<usize>,
) -> Result<Vec<usize>> {
    let base = base.unwrap_or(file.image_base);

    let sections = addresses
        .iter()
        .map(|addr| {
            if reverse {
                find_file_offset_section(file, *addr)
            } else {
                find_virt_addr_section(file, *addr, base)
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    let output = sections
        .into_iter()
        .zip(addresses.iter())
        .map(|(section, address)| {
            if reverse {
                let offset_in_section = address - section.pointer_to_raw_data as usize;
                section.virtual_address as usize + offset_in_section + base
            } else {
                let offset_in_section = address - base - section.virtual_address as usize;
                section.pointer_to_raw_data as usize + offset_in_section
            }
        })
        .collect::<Vec<_>>();

    Ok(output)
}

// finds the section that contains the virtal address.
fn find_virt_addr_section<'pe>(
    file: &'pe PE,
    addr: usize,
    base: usize,
) -> Result<&'pe SectionTable> {
    file.sections
        .iter()
        .find(|section| {
            let start = section.virtual_address as usize + base;
            let end = section.virtual_address as usize + base + section.virtual_size as usize;
            (start..=end).contains(&addr)
        })
        .ok_or(eyre!("Table for address: {addr:#X} is missing"))
}

// finds the section that contains the file offset.
fn find_file_offset_section<'pe>(file: &'pe PE, addr: usize) -> Result<&'pe SectionTable> {
    file.sections
        .iter()
        .find(|section| {
            let start = section.pointer_to_raw_data as usize;
            let end = section.pointer_to_raw_data as usize + section.size_of_raw_data as usize;
            (start..=end).contains(&addr)
        })
        .ok_or(eyre!("Table for reverse address: {addr:#X} is missing"))
}
