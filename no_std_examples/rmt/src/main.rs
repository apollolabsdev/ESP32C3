#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    esp_riscv_rt::entry,
    peripherals::Peripherals,
    prelude::*,
    pulse_control::{ClockSource, ConfiguredChannel, OutputChannel, PulseCode, RepeatMode},
    timer::TimerGroup,
    PulseControl, Rtc, IO,
};
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    // Take Peripherals, Initialize Clocks, and Create a Handle for Each
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Instantiate and Create Handles for the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;

    // Disable the RTC and TIMG watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Instantiate and Create Handle for IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Configure RMT peripheral
    let pulse = PulseControl::new(
        peripherals.RMT,
        &mut system.peripheral_clock_control,
        ClockSource::APB,
        0,
        0,
        0,
    )
    .unwrap();

    // Get reference to channel
    let mut rmt_channel0 = pulse.channel0;
    let mut rmt_channel1 = pulse.channel1;

    // Set up channel
    rmt_channel0
        .set_idle_output_level(false)
        .set_carrier_modulation(false)
        .set_channel_divider(1)
        .set_idle_output(true);

    rmt_channel1
        .set_idle_output_level(false)
        .set_carrier_modulation(false)
        .set_channel_divider(1)
        .set_idle_output(true);

    // Assign GPIO pin where pulses should be sent to
    let mut rmt_channel0 = rmt_channel0.assign_pin(io.pins.gpio6);
    let mut rmt_channel1 = rmt_channel1.assign_pin(io.pins.gpio5);

    // Create pulse sequence
    let seq = [PulseCode {
        level1: true,
        length1: 10u32.nanos(),
        level2: false,
        length2: 90u32.nanos(),
    }; 3];

    let seq1 = [PulseCode {
        level1: true,
        length1: 50u32.nanos(),
        level2: false,
        length2: 50u32.nanos(),
    }; 3];

    rmt_channel0
        .send_pulse_sequence(RepeatMode::SingleShot, &seq)
        .unwrap();

    rmt_channel1
        .send_pulse_sequence(RepeatMode::SingleShot, &seq1)
        .unwrap();

    // Application Loop
    loop {}
}
