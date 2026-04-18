#![no_std]
#![no_main]

#[macro_use]
extern crate log;

use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, peripherals::USB, usb};
use embassy_time::{Duration, Timer};
use panic_halt as _;

bind_interrupts!(struct Irqs {
  USBCTRL_IRQ => usb::InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: usb::Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let usb_driver = usb::Driver::new(p.USB, Irqs);

    spawner.spawn(logger_task(usb_driver).unwrap());

    let mut counter = 0;
    loop {
        info!("Count: {counter}");
        counter += 1;

        Timer::after(Duration::from_secs(1)).await;
    }
}
