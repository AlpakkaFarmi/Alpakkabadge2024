pub mod prelude;

pub use rp2040_hal as hal;

#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

hal::bsp_pins!(Gpio25 {
    name: led,
    aliases: { PushPullOutput: Led }
},Gpio7 {
    name: pwm7,
    aliases: { FunctionPwm: PWM7 }
},Gpio8 {
    name: pwm8,
    aliases: { FunctionPwm: PWM8 }
},Gpio9 {
    name: pwm9,
    aliases: { FunctionPwm: PWM9 }
},Gpio10 {
    name: pwm10,
    aliases: { FunctionPwm: PWM10 }
},Gpio11 {
    name: pwm11,
    aliases: { FunctionPwm: PWM11 }
},Gpio12 {
    name: pwm12,
    aliases: { FunctionPwm: PWM12 }
},Gpio13 {
    name: pwm13,
    aliases: { FunctionPwm: PWM13 }
},Gpio14 {
    name: pwm14,
    aliases: { FunctionPwm: PWM14 }
},Gpio15 {
    name: pwm15,
    aliases: { FunctionPwm: PWM15 }
},);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
