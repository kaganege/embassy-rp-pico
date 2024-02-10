#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use defmt::*;
use embassy_executor as executor;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb as rp_usb;
use embassy_usb::class::cdc_acm;

use {defmt_rtt as _, panic_probe as _};

mod defmt_usb;
// mod allocator;
mod usb;

bind_interrupts!(struct Irqs {
  USBCTRL_IRQ => rp_usb::InterruptHandler<USB>;
});

// https://www.usb.org/defined-class-codes
const DEVICE_CLASS: usb::Class = usb::Class {
  base: 0xFE,
  subclass: 0x02,
  protocol: 0x01,
};
const VID: u16 = 0xC0DE;
const PID: u16 = 0xCAFE;

#[embassy_executor::main]
async fn main(_spawner: executor::Spawner) {
  let p = embassy_rp::init(Default::default());
  let usb_driver = rp_usb::Driver::new(p.USB, Irqs);

  let mut usb_config = embassy_usb::Config::new(VID, PID);
  usb_config.product = Some("Raspberry Pi Pico W");
  usb_config.max_power = 100;
  usb_config.max_packet_size_0 = 64;

  // configs for windows
  usb_config.device_class = DEVICE_CLASS.base;
  usb_config.device_sub_class = DEVICE_CLASS.subclass;
  usb_config.device_protocol = DEVICE_CLASS.protocol;
  usb_config.composite_with_iads = true;

  // let mut device_descriptor = [0; 256];
  // let mut config_descriptor = [0; 256];
  // let mut bos_descriptor = [0; 256];
  // let mut control_buf = [0; 64];
  // let mut usb_state = cdc_acm::State::new();

  let mut usb_builder = embassy_usb::Builder::new(
    usb_driver,
    usb_config,
    &mut [],
    &mut [],
    &mut [],
    &mut [], // no msos descriptors
    &mut [],
  );

  // Why don't you live long enough? Holy shit
  // let USB_STATE: StaticCell<cdc_acm::State> = StaticCell::new();
  // let usb_state = USB_STATE.init(cdc_acm::State::new());
  let usb_state = Box::leak(Box::new(cdc_acm::State::new()));

  // Create classes on the builder.
  // static CLASS: StaticCell<cdc_acm::CdcAcmClass<'_, rp_usb::Driver<'_, USB>>> = StaticCell::new();
  // let class = CLASS.init(cdc_acm::CdcAcmClass::new(&mut usb_builder, usb_state, 64));
  let class = Box::leak(Box::new(cdc_acm::CdcAcmClass::new(
    &mut usb_builder,
    usb_state,
    64,
  )));
  // let mut class = cdc_acm::CdcAcmClass::new(&mut usb_builder, &mut usb_state, 64);
  // Come back my little class
  let class = defmt_usb::defmt_usb(class);

  // Build the builder.
  let mut usb = usb_builder.build();

  // // Run the USB device.
  let usb_fut = usb.run();

  let task = async {
    loop {
      class.wait_connection().await;
      println!("Is this work?");
    }
  };

  join(usb_fut, task).await;
}
