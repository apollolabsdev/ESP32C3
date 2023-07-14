use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Take Peripherals
    let dp = Peripherals::take().unwrap();

    // Configure all LED pins to digital outputs
    let mut led1 = PinDriver::output(dp.pins.gpio1).unwrap();
    let mut led2 = PinDriver::output(dp.pins.gpio10).unwrap();
    let mut led3 = PinDriver::output(dp.pins.gpio19).unwrap();
    let mut led4 = PinDriver::output(dp.pins.gpio18).unwrap();
    let mut led5 = PinDriver::output(dp.pins.gpio4).unwrap();
    let mut led6 = PinDriver::output(dp.pins.gpio5).unwrap();
    let mut led7 = PinDriver::output(dp.pins.gpio6).unwrap();
    let mut led8 = PinDriver::output(dp.pins.gpio7).unwrap();
    let mut led9 = PinDriver::output(dp.pins.gpio8).unwrap();
    let mut led10 = PinDriver::output(dp.pins.gpio9).unwrap();

    // Configure Button pin to input with Pull Up
    let mut button = PinDriver::input(dp.pins.gpio3).unwrap();
    button.set_pull(Pull::Up).unwrap();

    // Initialize variable with starting delay
    let mut blinkdelay = 200_u32;

    loop {
        // Algo:
        // Starting with first LED in sequence
        // 1. Turn on LED
        // 2. Retrieve adjusted delay based on button press
        // 3. Delay with adjusted value
        // 4. Turn off LED
        // 5. Delay for 100ms (to make sure LED is turned off)
        // 6. Repeat steps 1-5 for next LED in sequence
        // 7. Once all LEDs are done loop back to first LED in sequence

        // LED 1
        led1.set_high().unwrap();
        blinkdelay = button_pressed(&button, &blinkdelay);
        FreeRtos::delay_ms(blinkdelay);
        led1.set_low().unwrap();
        FreeRtos::delay_ms(100_u32);

        // LED 2
        led2.set_high().unwrap();
        blinkdelay = button_pressed(&button, &blinkdelay);
        FreeRtos::delay_ms(blinkdelay);
        led2.set_low().unwrap();
        FreeRtos::delay_ms(100_u32);

        // LED 3
        led3.set_high().unwrap();
        blinkdelay = button_pressed(&button, &blinkdelay);
        FreeRtos::delay_ms(blinkdelay);
        led3.set_low().unwrap();
        FreeRtos::delay_ms(100_u32);

        // LED 4
        led4.set_high().unwrap();
        blinkdelay = button_pressed(&button, &blinkdelay);
        FreeRtos::delay_ms(blinkdelay);
        led4.set_low().unwrap();
        FreeRtos::delay_ms(100_u32);

        // LED 5
        led5.set_high().unwrap();
        blinkdelay = button_pressed(&button, &blinkdelay);
        FreeRtos::delay_ms(blinkdelay);
        led5.set_low().unwrap();
        FreeRtos::delay_ms(100_u32);

        // LED 6
        led6.set_high().unwrap();
        blinkdelay = button_pressed(&button, &blinkdelay);
        FreeRtos::delay_ms(blinkdelay);
        led6.set_low().unwrap();
        FreeRtos::delay_ms(100_u32);

        // LED 7
        led7.set_high().unwrap();
        blinkdelay = button_pressed(&button, &blinkdelay);
        FreeRtos::delay_ms(blinkdelay);
        led7.set_low().unwrap();
        FreeRtos::delay_ms(100_u32);

        // LED 8
        led8.set_high().unwrap();
        blinkdelay = button_pressed(&button, &blinkdelay);
        FreeRtos::delay_ms(blinkdelay);
        led8.set_low().unwrap();
        FreeRtos::delay_ms(100_u32);

        // LED 9
        led9.set_high().unwrap();
        blinkdelay = button_pressed(&button, &blinkdelay);
        FreeRtos::delay_ms(blinkdelay);
        led9.set_low().unwrap();
        FreeRtos::delay_ms(100_u32);

        // LED 10
        led10.set_high().unwrap();
        blinkdelay = button_pressed(&button, &blinkdelay);
        FreeRtos::delay_ms(blinkdelay);
        led10.set_low().unwrap();
        FreeRtos::delay_ms(100_u32);
    }
}

fn button_pressed(but: &PinDriver<'_, Gpio3, Input>, del: &u32) -> u32 {
    // Check if Button has been pressed
    // If not pressed, return the delay value unchanged
    if but.is_low() {
        // if the value of the delay passed is less of equal to 50 then reset it to initial value
        // else subtract 50 from the passed delay
        println!("Button Pressed!");
        if del <= &50_u32 {
            return 200_u32;
        } else {
            return del - 50_u32;
        }
    } else {
        return *del;
    }
}
