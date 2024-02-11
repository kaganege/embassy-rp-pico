#![no_std]
#![no_main]

use embassy_executor as executor;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb;
use embassy_time::Timer;

macro_rules! println {
  ( $( $x:expr ),+ ) => {
    log::info!($($x),+)
  };
}

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
  USBCTRL_IRQ => usb::InterruptHandler<USB>;
});

#[executor::task]
async fn logger_task(driver: usb::Driver<'static, USB>) {
  embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[executor::main]
async fn main(spawner: executor::Spawner) {
  let p = embassy_rp::init(Default::default());
  let usb_driver = usb::Driver::new(p.USB, Irqs);
  spawner.spawn(logger_task(usb_driver)).unwrap();

  let mut counter = 0;
  loop {
    println!("Count: {}", counter);
    counter += 1;
    Timer::after_ticks(1).await;
  }
}
