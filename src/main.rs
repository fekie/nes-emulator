use apu::Apu;
use cartridge::Cartridge;
use clap::Parser;
use cpu::CpuContainer;
use debug::Tile;
use ines::Ines;
use ppu::Ppu;
use std::cell::RefCell;
use std::rc::Rc;

mod apu;
mod cartridge;
mod cpu;
mod debug;
mod display;
mod graphical_debug;
mod ines;
mod ppu;
mod runtime;

pub struct MapperType {}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the rom to load into the program.
    rom: String,
    /// Prints the CHR-ROM pattern table to the terminal.
    #[clap(short, long, default_value = None)]
    pattern_table: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let rom = Ines::parse(&std::fs::read(&args.rom)?);

    if check_and_run_debug(&args, &rom) {
        return Ok(());
    }

    runtime::run(move || initialize_emulator(rom))?;
    Ok(())
}

fn initialize_emulator(rom: Ines) -> runtime::Emulator {
    let cpu = Rc::new(RefCell::new(CpuContainer::new()));
    let ppu = Rc::new(RefCell::new(Ppu::new()));
    let apu = Rc::new(RefCell::new(Apu::new()));
    let cartridge = Rc::new(RefCell::new(Cartridge::new(rom)));

    cpu.borrow_mut()
        .initialize(ppu.clone(), apu.clone(), cartridge.clone());
    ppu.borrow_mut().initialize(cpu.clone());
    apu.borrow_mut().initialize();
    cartridge.borrow_mut().initialize(cpu.clone(), ppu.clone());

    assert!(cpu.borrow().initialized());
    assert!(ppu.borrow().initialized());
    assert!(apu.borrow().initialized());
    assert!(cartridge.borrow().initialized());

    runtime::Emulator::from_initialized(cpu, ppu, cartridge)
}

fn check_and_run_debug(args: &Args, rom: &Ines) -> bool {
    if args.pattern_table {
        print_pattern_tables(rom);
        return true;
    }

    false
}

fn print_pattern_tables(rom: &Ines) {
    let pattern_bytes = &rom.character_rom[0..=0x1FFF];
    let tiles = pattern_bytes
        .chunks(16)
        .map(debug::deinterlace_tile_bytes)
        .collect::<Vec<Tile>>();

    let img = debug::stitch_tiles(&tiles);
    img.save("pattern_table.png").unwrap();
    println!("Saved pattern table to pattern_table.png");
}
