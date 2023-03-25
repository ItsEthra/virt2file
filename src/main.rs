use argh::FromArgs;
use eyre::Result;
use goblin::{pe::PE, Hint};
use std::{fs, io::Cursor};

mod windows;

#[derive(Debug, FromArgs)]
/// Convert from virtal address to file offset and back.
struct Args {
    /// convert from file offset to virtual address instead.
    #[argh(switch, short = 'r')]
    pub reverse: bool,
    /// binary file to work with.
    #[argh(option, short = 'f')]
    pub file: String,
    /// addresses to convert, accepts addresses like `0xAABB` `AABB`.
    #[argh(positional)]
    pub addresses: Vec<String>,
    /// changes base of the image.
    #[argh(option, short = 'b')]
    pub base: Option<String>,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let addresses: Vec<usize> = args
        .addresses
        .iter()
        .map(|addr| addr.strip_prefix("0x").unwrap_or(addr))
        .map(|addr| usize::from_str_radix(addr, 16))
        .collect::<Result<Vec<_>, _>>()?;
    let base = args
        .base
        .as_ref()
        .map(|base| {
            let stipped = base.strip_prefix("0x").unwrap_or(base);
            usize::from_str_radix(stipped, 16)
        })
        .transpose()?;

    let mut binary = Cursor::new(fs::read(&args.file)?);
    let hint = goblin::peek(&mut binary)?;

    let results = match hint {
        Hint::PE => {
            let pe = PE::parse(&binary.get_ref()[..])?;
            windows::output(&pe, &addresses[..], args.reverse, base)?
        }
        _ => eyre::bail!("Unsupported file format, supported formats are: PE"),
    };
    results.into_iter().for_each(|addr| println!("{addr:#X}"));

    Ok(())
}
