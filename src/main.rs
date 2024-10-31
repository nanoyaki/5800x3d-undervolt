use std::{
	env, fs, path::Path, thread::sleep, time::Duration
};

const SMU_DRIVER_PATH: &str = "/sys/kernel/ryzen_smu_drv";

const VERSION_FILE: &str = "version";
const MP1_SMU_CMD_FILE: &str = "mp1_smu_cmd";
const SMU_ARGS_FILE: &str = "smu_args";

fn main() {
	if !Path::new(&SMU_DRIVER_PATH).join(VERSION_FILE).exists() {
		panic!("Ryzen SMU is not loaded.")
	};

	if env::args().len() <= 1 {
		for i in 0u8..8 {
			format_response(get_core_offset(i), i);
		}
		return;
	}

	let cores: u8 = env::args().nth(1).expect("No core count given.").parse().expect("Core count must be an 8 bit unsigned int");
	let offset: i8 = env::args().nth(2).expect("No offset given.").parse().expect("Offset must be an 8 bit signed int");

	if offset > 0 {
		panic!("Please do not use this as an overclocking utility. There's better programs for that out there");
	}

	println!("Setting {} cores to a {} milivolt offset", cores, offset);

	for i in 0u8..8 {
		format_response(set_core_offset(cores, offset), i);
	}
}

fn format_response(response: SmuArgs, core: u8) {
	println!("Core offset of Core: {} readback: {}", core + 1, (response[0] as i64 - u32::MAX as i64 - 1) as i8);
} 

fn get_core_offset(core: u8) -> SmuArgs {
	let arg: u32 = ((core as u32 & 0b1000) << 0b101 | core as u32 & 0b0111) << 0b10100;
	mp1_smu_command(0x48, [arg, 0, 0, 0, 0, 0])
}

fn set_core_offset(core: u8, offset: i8) -> SmuArgs {
	let arg: u32 = ((core as u32 & 0b1000) << 0b101 | core as u32 & 0b0111) << 0b10100 | offset as u32 & u16::MAX as u32;
	mp1_smu_command(0x35, [arg, 0, 0, 0, 0, 0])
}

fn mp1_smu_command(op: u8, args: SmuArgs) -> SmuArgs {
	wait_for_smu_cmd();

	write_smu_args(args);
	write_mp1_smu_cmd(op);

	let cmd_result = wait_for_smu_cmd();
	if cmd_result != 1 {
		panic!("SMU Command failed with code: {}", cmd_result);
	}

	read_smu_args()
}

fn wait_for_smu_cmd() -> u32 {
	let mut mp1_smu_cmd_result = read_mp1_smu_command();
	while mp1_smu_cmd_result == 0x00 {
		sleep(Duration::new(1, 0));
		mp1_smu_cmd_result = read_mp1_smu_command();
	}

	mp1_smu_cmd_result
}

fn write_mp1_smu_cmd(op: u8) {
	let mp1_smu_cmd_path = Path::new(&SMU_DRIVER_PATH).join(MP1_SMU_CMD_FILE);
	fs::write(mp1_smu_cmd_path, [op]).expect("Couldn't write to the mp1_smu_cmd file");
}

fn write_smu_args(args: SmuArgs) {
	let smu_args_path = Path::new(&SMU_DRIVER_PATH).join(SMU_ARGS_FILE);
	fs::write(smu_args_path, args.map(|num| num.to_le_bytes()).as_flattened()).expect("Couldn't write to the smu_args file");
}

fn read_smu_args() -> SmuArgs {
	let smu_args_path = Path::new(&SMU_DRIVER_PATH).join(SMU_ARGS_FILE);
	let raw_bytes = fs::read(smu_args_path.as_path()).expect("Couldn't read the smu_args file");

	u8_vec_to_u32(&raw_bytes).try_into().expect("Something went horribly wrong whilst trying to read the smu_args file")
}

fn read_mp1_smu_command() -> u32 {
	let mp1_smu_cmd_path = Path::new(&SMU_DRIVER_PATH).join(MP1_SMU_CMD_FILE);
	let raw_bytes = fs::read(mp1_smu_cmd_path.as_path()).expect("Couldn't read the mp1_smu_cmd file");
	
	u8_vec_to_u32(&raw_bytes)[0]
}

fn u8_vec_to_u32(vec: &[u8]) -> Vec<u32> {
	assert!(vec.len() % 4 == 0, "Input vector must be a multiple of 4");

	vec.chunks_exact(4)
		.map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
		.collect()
}

type SmuArgs = [u32; 6];
