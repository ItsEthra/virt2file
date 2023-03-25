use argh::FromArgs;
use eyre::Result;
use std::fs;

#[derive(Debug, FromArgs)]
/// Convert from virtal address to file offset and back.
struct Args {
    /// convert from file offset to virtual address instead.
    #[argh(switch, short = 'r')]
    reverse: bool,
    /// binary file to work with.
    #[argh(option, short = 'f')]
    file: String,
    /// addresses to convert, accepts addresses like `0xAABB` `AABB`.
    #[argh(positional)]
    addresses: Vec<String>,
    /// changes base of the image.
    #[argh(option, short = 'b')]
    base: Option<String>,
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
    let file = fs::read(args.file)?;

    let results = virt2file::convert(file, &addresses[..], args.reverse, base)?;
    results.into_iter().for_each(|addr| println!("{addr:#X}"));

    Ok(())
}
