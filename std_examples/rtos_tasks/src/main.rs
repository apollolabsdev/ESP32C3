/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/esp-embedded-rust-multithreading-with-freertos-bindings

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys::{self as _};
use esp_idf_sys::{esp, vTaskDelay, xPortGetTickRateHz, xTaskCreatePinnedToCore, xTaskDelayUntil};
use std::ffi::CString;

unsafe extern "C" fn task1(_: *mut core::ffi::c_void) {
    loop {
        println!("Task 1 Entered");
        FreeRtos::delay_ms(1000);
    }
}

unsafe extern "C" fn task2(_: *mut core::ffi::c_void) {
    loop {
        println!("Task 2 Entered");
        FreeRtos::delay_ms(2000);
    }
}

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    unsafe {
        xTaskCreatePinnedToCore(
            Some(task1),
            CString::new("Task 1").unwrap().as_ptr(),
            1000,
            std::ptr::null_mut(),
            10,
            std::ptr::null_mut(),
            1,
        );
    }
    FreeRtos::delay_ms(1000);
    unsafe {
        xTaskCreatePinnedToCore(
            Some(task2),
            CString::new("Task 2").unwrap().as_ptr(),
            1000,
            std::ptr::null_mut(),
            9,
            std::ptr::null_mut(),
            1,
        );
    }
    loop {
        println!("Hello From Main");
        FreeRtos::delay_ms(500);
    }
}
