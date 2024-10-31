# Undervolting utility

This is a small undervolting utility for the Vermeer Ryzen CPUs.
Use with caution. Any core offset above 0 will be disregarded.

## Requirements

- A Linux system
- Ryzen SMU
- Rust ^1.81
- Cargo ^1.81

## Installation

Clone the repository

Build with

```bash
git clone https://github.com/nanoyaki/5800x3d-undervolt.git
cd 5800x3d-undervolt
cargo build -r
```

The binary can then be found in `target/release/` as
`vermeer-undervolt`

## Usage

Make sure the Ryzen SMU kernel module is loaded.

```bash
vermeer-undervolt <core count> <offset>
```

Core count should be 8 or less and offset doesn't seem to work
below -30.
