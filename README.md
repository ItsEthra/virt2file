# virt2file
Simple utility to convert from virtual address(VA) to file offset and back.

## Installation
```sh
cargo install virt2file
```

## Usage
To convert from VA to file offset use:
```sh
virt2file -f target.exe 0x7F11223344
```

To convert from file offset to VA use:
```sh
virt2file -f target.exe 0x7F11223344 -r
```
