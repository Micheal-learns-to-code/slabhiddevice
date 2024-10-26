use slabhiddevice::hiddevice;

fn main() {
    let hiddev = hiddevice::HidDevice::new().unwrap();

    let num_dev = hiddev.hid_device_get_num_hid_devices(0x10c4, 0xea90);
    println!("Number of CP2112 device {} ", num_dev);

    let hid_str = hiddev
        .hid_device_get_hid_string(
            0,
            0x10c4,
            0xea90,
            slabhiddevice::types::HidStringType::Manufacturer,
        )
        .unwrap();
    println!("Manufacturer string of CP2112 device {} ", hid_str);
}
