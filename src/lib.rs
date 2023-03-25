use eyre::Result;
use goblin::{pe::PE, Hint};
use std::io::Cursor;

mod windows;

/// Converts between virtual addresses and file offests.
/// `reverse`: converts from file offsets to virtual addresses instead.
/// `base`: changes base of the image, by default uses the one from file header.
pub fn convert(
    file: impl AsRef<[u8]>,
    addresses: &[usize],
    reverse: bool,
    base: Option<usize>,
) -> Result<Vec<usize>> {
    let mut binary = Cursor::new(file);
    let hint = goblin::peek(&mut binary)?;

    match hint {
        Hint::PE => {
            let pe = PE::parse(binary.get_ref().as_ref())?;
            windows::output(&pe, addresses, reverse, base)
        }
        _ => eyre::bail!("Unsupported file format, supported formats are: PE"),
    }
}
