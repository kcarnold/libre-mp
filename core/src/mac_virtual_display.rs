//! extra desktop for the projector, via coregraphics' private CGVirtualDisplay (macos 11+, same as deskpad).
//! lives while this value lives; drop removes it and macos moves its windows back.

use std::ffi::c_void;

use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject};
use objc2::msg_send;
use objc2_foundation::{NSArray, NSSize, NSString};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

extern "C" {
    fn dispatch_get_global_queue(identifier: isize, flags: usize) -> *mut c_void;
}

pub struct VirtualDisplay {
    _display: Retained<AnyObject>,
    pub id: u32,
}

fn class(name: &str) -> Result<&'static AnyClass, String> {
    AnyClass::get(&std::ffi::CString::new(name).unwrap())
        .ok_or_else(|| format!("{name} not found; virtual displays need macOS 11 or later"))
}

impl VirtualDisplay {
    // one fixed mode, no hidpi, so pixels map 1:1 to the stream
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        unsafe {
            let desc: Retained<AnyObject> = msg_send![class("CGVirtualDisplayDescriptor")?, new];
            let queue = dispatch_get_global_queue(0, 0) as *mut AnyObject;
            let _: () = msg_send![&*desc, setDispatchQueue: queue];
            let name = NSString::from_str("LibreMP Projector");
            let _: () = msg_send![&*desc, setName: &*name];
            let _: () = msg_send![&*desc, setMaxPixelsWide: width];
            let _: () = msg_send![&*desc, setMaxPixelsHigh: height];
            // ~100 dpi, so macos picks sane default text size
            let mm = NSSize::new(width as f64 * 0.254, height as f64 * 0.254);
            let _: () = msg_send![&*desc, setSizeInMillimeters: mm];
            let _: () = msg_send![&*desc, setVendorID: 0x4c4du32];
            let _: () = msg_send![&*desc, setProductID: 0x5031u32];
            let _: () = msg_send![&*desc, setSerialNum: 1u32];

            let alloc: Allocated<AnyObject> = msg_send![class("CGVirtualDisplay")?, alloc];
            let display: Option<Retained<AnyObject>> = msg_send![alloc, initWithDescriptor: &*desc];
            let display = display.ok_or("macOS refused to create the virtual display")?;

            let alloc: Allocated<AnyObject> = msg_send![class("CGVirtualDisplayMode")?, alloc];
            let mode: Retained<AnyObject> =
                msg_send![alloc, initWithWidth: width, height: height, refreshRate: 60.0f64];
            let settings: Retained<AnyObject> = msg_send![class("CGVirtualDisplaySettings")?, new];
            let _: () = msg_send![&*settings, setHiDPI: 0u32];
            let modes = NSArray::from_retained_slice(&[mode]);
            let _: () = msg_send![&*settings, setModes: &*modes];
            let ok: bool = msg_send![&*display, applySettings: &*settings];
            if !ok {
                return Err("macOS refused the virtual display's settings".into());
            }
            let id: u32 = msg_send![&*display, displayID];
            Ok(VirtualDisplay { _display: display, id })
        }
    }
}
