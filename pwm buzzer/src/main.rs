#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    delay::Delay,
    ledc::{
        channel,
        timer::{self},
        LSGlobalClkSource, LowSpeed, LEDC,
    },
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Rtc, IO,
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
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    // Disable the RTC and TIMG watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Instantiate and Create Handle for IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Instantiate and Create Handle for LED output & Button Input
    let mut buzzer_pin = io.pins.gpio1.into_push_pull_output();

    // Define the notes and their frequencies
    let tones = [
        ('c', 261_u32.Hz()),
        ('d', 294_u32.Hz()),
        ('e', 329_u32.Hz()),
        ('f', 349_u32.Hz()),
        ('g', 329_u32.Hz()),
        ('a', 440_u32.Hz()),
        ('b', 493_u32.Hz()),
    ];

    // Define the notes to be played and the beats per note
    let tune = [
        ('c', 1),
        ('c', 1),
        ('g', 1),
        ('g', 1),
        ('a', 1),
        ('a', 1),
        ('g', 2),
        ('f', 1),
        ('f', 1),
        ('e', 1),
        ('e', 1),
        ('d', 1),
        ('d', 1),
        ('c', 2),
        (' ', 4),
    ];

    // Define the tempo
    let tempo = 300_u32;

    // Initialize and create handle for LEDC peripheral
    let mut buzzer = LEDC::new(
        peripherals.LEDC,
        &clocks,
        &mut system.peripheral_clock_control,
    );

    // Set up global clock source for LEDC to APB Clk
    buzzer.set_global_slow_clock(LSGlobalClkSource::APBClk);

    // Instantiate Delay handle
    let mut delay = Delay::new(&clocks);

    // Application Loop
    loop {
        // Obtain a note in the tune
        for note in tune {
            // Retrieve the freqeuncy and beat associated with the note
            for tone in tones {
                // Find a note match in the tones array and update frequency and beat variables accordingly
                if tone.0 == note.0 {
                    // Play the note for the desired duration (beats*tempo)
                    // Adjust period of the PWM output to match the new frequency
                    let mut lstimer0 = buzzer.get_timer::<LowSpeed>(timer::Number::Timer0);
                    lstimer0
                        .configure(timer::config::Config {
                            duty: timer::config::Duty::Duty13Bit,
                            clock_source: timer::LSClockSource::APBClk,
                            frequency: tone.1,
                        })
                        .unwrap();

                    let mut channel0 =
                        buzzer.get_channel(channel::Number::Channel0, &mut buzzer_pin);
                    channel0
                        .configure(channel::config::Config {
                            timer: &lstimer0,
                            duty_pct: 50,
                        })
                        .unwrap();

                    // Keep the output on for as long as required by note
                    delay.delay_ms(note.1 * tempo);
                } else if note.0 == ' ' {
                    // If ' ' tone is found disable output for one beat
                    let mut lstimer0 = buzzer.get_timer::<LowSpeed>(timer::Number::Timer0);
                    lstimer0
                        .configure(timer::config::Config {
                            duty: timer::config::Duty::Duty13Bit,
                            clock_source: timer::LSClockSource::APBClk,
                            frequency: 1_u32.Hz(),
                        })
                        .unwrap();
                    let mut channel0 =
                        buzzer.get_channel(channel::Number::Channel0, &mut buzzer_pin);

                    channel0
                        .configure(channel::config::Config {
                            timer: &lstimer0,
                            duty_pct: 0,
                        })
                        .unwrap();
                    // Keep the output off for as long as required by note
                    delay.delay_ms(tempo);
                }
            }
            // Silence for half a beat between notes
            let mut lstimer0 = buzzer.get_timer::<LowSpeed>(timer::Number::Timer0);
            lstimer0
                .configure(timer::config::Config {
                    duty: timer::config::Duty::Duty13Bit,
                    clock_source: timer::LSClockSource::APBClk,
                    frequency: 1_u32.Hz(),
                })
                .unwrap();
            let mut channel0 = buzzer.get_channel(channel::Number::Channel0, &mut buzzer_pin);

            channel0
                .configure(channel::config::Config {
                    timer: &lstimer0,
                    duty_pct: 0,
                })
                .unwrap();
            // Keep the output off for half a beat between notes
            delay.delay_ms(tempo / 2);
        }
    }
}
