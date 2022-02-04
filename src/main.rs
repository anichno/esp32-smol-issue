use std::time::Duration;

use anyhow::Result;
use esp_idf_sys::{self as _};
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    esp_idf_sys::esp!(unsafe {
        esp_idf_sys::esp_vfs_eventfd_register(&esp_idf_sys::esp_vfs_eventfd_config_t {
            max_fds: 5,
            ..Default::default()
        })
    })?;

    smol::block_on(async {
        println!("main task started");

        loop {
            println!("main task print");
            smol::Timer::after(Duration::from_millis(100)).await;
        }
    });

    println!("All Done!");
    Ok(())
}
