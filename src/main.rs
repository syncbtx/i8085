use i8085::memory::DEFAULT_IMAGE;
use i8085::motherboard::Motherboard;
use i8085::utils::{DataFormat, ToFormatted};

fn main() {
    let mut motherboard = Motherboard::with_bootloader(&DEFAULT_IMAGE);

    let user_program = [0x3E, 0x42, 0x76]; // MVI A, 42; HLT
    motherboard.load_to_ram(0x0000, &user_program).expect("TODO: panic message");

    motherboard.run_until_halt();

    println!("\nFinal CPU State:");
    println!("{}", motherboard.cpu);

    let ram_data = motherboard.mem.read_slice(0x1000, 3);
    println!("\nProgram at 0x1000:");
    println!("Hex:     {}", ram_data.format(DataFormat::Hex));
    println!("Binary:  {}", ram_data.format(DataFormat::Bin));
    println!("Decimal: {}", ram_data.format(DataFormat::Dec));

    println!("\nRegister Views:");
    println!("PC (Binary):  {}", motherboard.cpu.regs.pc.format(DataFormat::Bin));
    println!("A (Decimal):   {}", motherboard.cpu.regs.a.format(DataFormat::Dec));
    println!("RAM (Decimal):   {}", motherboard.mem.ram.read(0x1000).format(DataFormat::Hex));

}
