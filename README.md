# Undervolting utility

This is a small undervolting utility for the Vermeer Ryzen CPUs.
Use with caution. Any core offset above 0 will be disregarded.

## Requirements

- A Linux system
- Ryzen SMU
- Rust ^1.81
- Cargo ^1.81

## Installation

The package can be installed through either cargo or nix.

### Through cargo

```bash
cargo install --git https://github.com/nanoyaki/5800x3d-undervolt
```

### Through nix flakes

Add the repo to your flake's inputs and optionally import the 
module. The package itself is provided under 
`vermeer-undervolt.packages.<system>.default`.

```nix
{
	inputs = {
		# ...
    	vermeer-undervolt.url = "github:nanoyaki/5800x3d-undervolt/main";
		# ...
	};

	outputs = inputs@{ nixpkgs, vermeer-undervolt, ... }: {
		nixosConfigurations."some-hostname" = nixpkgs.lib.nixosSystem {
			# ...
			modules = [
				vermeer-undervolt.nixosModules.vermeer-undervolt
				# ...
			];
		};
	};
}
```

## Usage

Make sure the Ryzen SMU kernel module is loaded.

```bash
vermeer-undervolt <core_count> <offset>
```

Core count should be 8 or less and offset doesn't seem to work
below -30.
