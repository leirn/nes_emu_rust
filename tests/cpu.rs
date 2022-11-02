use nes_emu_rust::nes_emulator::NesEmulator;

#[test]
fn nestest() {
    simple_logger::init().unwrap();
    let mut emulator = NesEmulator::new(String::from("tests/nestest/nestest.nes"));
    emulator.set_test_mode("tests/nestest/nestest.log");
    emulator.start(Some(0xc000));
}
