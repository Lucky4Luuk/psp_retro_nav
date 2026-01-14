pub fn init_gps() {
    unsafe {
        psp::dprintln!(
            "load_usb_module_acc: {}",
            psp::sys::sceUtilityLoadUsbModule(psp::sys::UsbModule::UsbAcc)
        );
        psp::dprintln!(
            "load_usb_module_gps: {}",
            psp::sys::sceUtilityLoadUsbModule(psp::sys::UsbModule::UsbGps)
        );

        // sceUsbStart(PSP_USBBUS_DRIVERNAME,0,0)
        // sceUsbStart("USBAccBaseDriver",0,0)
        // sceUsbStart(PSP_USBGPS_DRIVERNAME,0,0)
        // TODO: These all fail if they return > 0
        //       Handle this ASAP
        psp::sys::sceUsbStart(
            psp::sys::USB_BUS_DRIVER_NAME.as_ptr(),
            0,
            core::ptr::null_mut(),
        );
        psp::sys::sceUsbStart("USBAccBaseDriver".as_ptr(), 0, core::ptr::null_mut());
        psp::sys::sceUsbStart(
            psp::sys::USB_GPS_DRIVER_NAME.as_ptr(),
            0,
            core::ptr::null_mut(),
        );

        // sceUsbGpsOpen()
        psp::sys::sceUsbGpsOpen();

        psp::sys::sceUsbActivate(psp::sys::USB_GPS_PID as u32);
        psp::sys::sceUsbGpsSetInitDataLocation(1);
    }

    let mut init_data_location = 0u32;
    unsafe {
        psp::sys::sceUsbGpsGetInitDataLocation(&mut init_data_location);
    }
    psp::dprintln!("{} (should be 1)", init_data_location);

    // let usb_state = unsafe { psp::sys::sceUsbGetState() };
    // psp::dprintln!("{:?}", usb_state);

    psp::dprintln!("Waiting for the gps...");
    // TODO: Add some sort of delay here to avoid running the PSP at 100% usage lol
    //       Also perhaps add a timeout?
    loop {
        let mut gps_state = 0u32;
        let gps_state_ret = unsafe { psp::sys::sceUsbGpsGetState(&mut gps_state) };

        if gps_state == 0x3 {
            psp::dprintln!("state: {:?}", gps_state);
            psp::dprintln!("ret: {:?}", gps_state_ret);
            psp::dprintln!("----------------");
            break;
        }
    }
}

pub fn get_raw_data() -> (psp::sys::ScePspGpsData, psp::sys::ScePspSatData) {
    let mut gps_data = psp::sys::ScePspGpsData::default();
    let mut sat_data = psp::sys::ScePspSatData::default();

    unsafe {
        psp::sys::sceUsbGpsGetData(&mut gps_data, &mut sat_data);
    }

    (gps_data, sat_data)
}
