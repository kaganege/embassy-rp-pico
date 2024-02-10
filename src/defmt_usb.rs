use alloc::boxed::Box;
use core::sync::atomic::{AtomicBool, Ordering};
use defmt::{global_logger, Encoder};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::class::cdc_acm::CdcAcmClass;

static mut ENCODER: Encoder = Encoder::new();
static TAKEN: AtomicBool = AtomicBool::new(false);
static mut CS_RESTORE: critical_section::RestoreState = critical_section::RestoreState::invalid();

static mut ERASED_WRITE: Option<&mut CdcAcmClass<'_, Driver<'_, USB>>> = None;

pub fn defmt_usb<'d>(
  class: &'d mut CdcAcmClass<'d, Driver<'d, USB>>,
) -> &'d mut CdcAcmClass<'d, Driver<'d, USB>> {
  unsafe {
    critical_section::with(|_| {
      assert!(
        ERASED_WRITE.is_none(),
        "Tried to assign serial port when one was already assigned."
      );
    });

    ERASED_WRITE = Some(class);
  }

  class
}

pub fn release() {
  unsafe {
    critical_section::with(|_| {
      if TAKEN.load(Ordering::Relaxed) {
        panic!("defmt logger taken reentrantly");
      }

      ERASED_WRITE = None;
    });
  }
}

#[global_logger]
struct GlobalUSBLogger;

unsafe impl defmt::Logger for GlobalUSBLogger {
  fn acquire() {
    let restore = unsafe { critical_section::acquire() };

    if TAKEN.load(Ordering::Relaxed) {
      panic!("defmt logger taken reentrantly");
    }

    TAKEN.store(true, Ordering::Relaxed);

    unsafe {
      CS_RESTORE = restore;
    }

    unsafe { ENCODER.start_frame(write_usb) }
  }

  unsafe fn release() {
    ENCODER.end_frame(write_usb);
    TAKEN.store(false, Ordering::Relaxed);

    let restore = CS_RESTORE;
    critical_section::release(restore);
  }

  unsafe fn write(bytes: &[u8]) {
    ENCODER.write(bytes, write_usb);
  }

  unsafe fn flush() {
    // if let Some(writer) = &mut ERASED_WRITE {
    //   (*writer).flush();
    // }
  }
}

fn write_usb(remaining: &[u8]) {
  unsafe {
    if let Some(writer) = &mut ERASED_WRITE {
      Box::pin(async move { (*writer).write_packet(remaining).await });
    }
  }
}
