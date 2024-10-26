use crate::types::HidStringType;
use hidapi::{DeviceInfo, HidApi};

use crate::error::{HidDeviceError, HidDeviceResult};

fn get_num_device(hidapi: &HidApi, vid: u16, pid: u16) -> u32 {
    let mut num_device = 0;
    for device in hidapi.device_list() {
        if (vid == 0 || vid == device.vendor_id()) && (pid == 0 || pid == device.product_id()) {
            num_device += 1;
        }
    }

    num_device
}

pub struct HidDeviceHandle(pub hidapi::HidDevice);

pub struct HidDevice {
    hidapi: HidApi,
}

impl HidDevice {
    pub fn new() -> HidDeviceResult<Self> {
        match HidApi::new() {
            Ok(h) => Ok(HidDevice { hidapi: h }),
            Err(_) => return Err(HidDeviceError::SystemError),
        }
    }

    fn is_index_valid(&self, index: u32, vid: u16, pid: u16) -> bool {
        let num_device = self.hid_device_get_num_hid_devices(vid, pid);
        index <= num_device
    }

    fn get_string(
        &self,
        device_info: &DeviceInfo,
        hid_string_type: HidStringType,
    ) -> HidDeviceResult<String> {
        let hid_string = match hid_string_type {
            HidStringType::Vid => format!("{:x}", device_info.vendor_id()),
            HidStringType::Pid => format!("{:x}", device_info.product_id()),
            HidStringType::Path => match device_info.path().to_str() {
                Ok(s) => s.to_string(),
                Err(_) => return Err(HidDeviceError::CannotGetHidInfo),
            },
            HidStringType::Product => device_info.product_string().unwrap_or("").to_string(),
            HidStringType::Serial => device_info.serial_number().unwrap_or("default").to_string(),
            HidStringType::Manufacturer => device_info
                .manufacturer_string()
                .unwrap_or("default")
                .to_string(),
        };
        return Ok(hid_string);
    }

    fn get_indexed_string(
        &self,
        device_handle: &hidapi::HidDevice,
        string_index: i32,
    ) -> HidDeviceResult<String> {
        let s = match device_handle.get_indexed_string(string_index) {
            Ok(ind_str) => ind_str,
            Err(_) => return Err(HidDeviceError::CannotGetHidInfo),
        };
        return Ok(s.unwrap_or("".to_string()));
    }

    fn get_hid_attributes(&self, device_info: &DeviceInfo) -> (u16, u16, u16) {
        (
            device_info.vendor_id(),
            device_info.product_id(),
            device_info.release_number(),
        )
    }

    pub fn hid_device_get_num_hid_devices(&self, vid: u16, pid: u16) -> u32 {
        get_num_device(&self.hidapi, vid, pid)
    }

    pub fn hid_device_get_hid_string(
        &self,
        index: u32,
        vid: u16,
        pid: u16,
        hid_string_type: HidStringType,
    ) -> HidDeviceResult<String> {
        if !self.is_index_valid(index, vid, pid) {
            return Err(HidDeviceError::DeviceNotFound);
        }

        let mut num_device = 0;
        for device in self.hidapi.device_list() {
            if num_device == index {
                return self.get_string(device, hid_string_type);
            }

            num_device += 1;
        }

        Err(HidDeviceError::CannotGetHidInfo)
    }

    pub fn hid_device_get_hid_indexed_string(
        &self,
        index: u32,
        vid: u16,
        pid: u16,
        string_index: i32,
    ) -> HidDeviceResult<String> {
        if !self.is_index_valid(index, vid, pid) {
            return Err(HidDeviceError::DeviceNotFound);
        }
        let mut num_device = 0;
        for device in self.hidapi.device_list() {
            if num_device == index {
                let d = match device.open_device(&self.hidapi) {
                    Ok(dev) => dev,
                    Err(_) => return Err(HidDeviceError::DeviceNotOpened),
                };

                return self.get_indexed_string(&d, string_index);
            }

            num_device += 1;
        }

        Err(HidDeviceError::CannotGetHidInfo)
    }

    pub fn hid_device_get_hid_attributes(
        &self,
        index: u32,
        vid: u16,
        pid: u16,
    ) -> HidDeviceResult<(u16, u16, u16)> {
        if !self.is_index_valid(index, vid, pid) {
            return Err(HidDeviceError::DeviceNotFound);
        }

        let mut num_device = 0;
        for device in self.hidapi.device_list() {
            if num_device == index {
                return Ok(self.get_hid_attributes(device));
            }

            num_device += 1;
        }

        Err(HidDeviceError::CannotGetHidInfo)
    }

    pub fn hid_device_open(
        &self,
        index: u32,
        vid: u16,
        pid: u16,
    ) -> HidDeviceResult<HidDeviceHandle> {
        if !self.is_index_valid(index, vid, pid) {
            return Err(HidDeviceError::DeviceNotFound);
        }

        let mut num_device = 0;
        for _ in self.hidapi.device_list() {
            if num_device == index {
                return match self.hidapi.open(vid, pid) {
                    Ok(d) => Ok(HidDeviceHandle(d)),
                    Err(_) => Err(HidDeviceError::SystemError),
                };
            }

            num_device += 1;
        }

        Err(HidDeviceError::SystemError)
    }

    pub fn hid_device_get_string(
        &self,
        device_handle: &HidDeviceHandle,
        hid_string_type: HidStringType,
    ) -> HidDeviceResult<String> {
        match device_handle.0.get_device_info() {
            Ok(device_info) => return self.get_string(&device_info, hid_string_type),
            Err(_) => return Err(HidDeviceError::SystemError),
        }
    }

    pub fn hid_device_get_indexed_string(
        &self,
        device_handle: &HidDeviceHandle,
        string_index: i32,
    ) -> HidDeviceResult<String> {
        return self.get_indexed_string(&device_handle.0, string_index);
    }

    pub fn hid_device_set_feature_report_control(
        &self,
        device_handle: &HidDeviceHandle,
        data: &[u8],
    ) -> HidDeviceResult<()> {
        match device_handle.0.send_feature_report(data) {
            Ok(_) => Ok(()),
            Err(_) => Err(HidDeviceError::TransferFailed),
        }
    }

    pub fn hid_device_get_feature_report_control(
        &self,
        device_handle: &HidDeviceHandle,
        data: &mut [u8],
    ) -> HidDeviceResult<usize> {
        match device_handle.0.get_feature_report(data) {
            Ok(report_size) => Ok(report_size),
            Err(_) => Err(HidDeviceError::TransferFailed),
        }
    }

    pub fn hid_device_set_output_report(
        &self,
        device_handle: &HidDeviceHandle,
        data: &[u8],
    ) -> HidDeviceResult<()> {
        match device_handle.0.write(data) {
            Ok(_) => Ok(()),
            Err(_) => Err(HidDeviceError::TransferFailed),
        }
    }

    pub fn hid_device_get_input_report(
        &self,
        device_handle: &HidDeviceHandle,
        buf: &mut [u8],
    ) -> HidDeviceResult<usize> {
        match device_handle.0.read(buf) {
            Ok(report_size) => Ok(report_size),
            Err(_) => Err(HidDeviceError::TransferFailed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    //static INIT : Once = Once::new();
    static HID_DEVICE_CELL: OnceLock<HidDevice> = OnceLock::new();

    fn is_cp2112ek_connected() -> bool {
        let api = HidApi::new().expect("Unable to use hidapi. Please verify this library");
        let tmp = api
            .device_list()
            .any(|device| device.vendor_id() == 0x10c4 && device.product_id() == 0xea90);
        tmp
    }

    fn set_up() {
        HID_DEVICE_CELL.get_or_init(|| {
            if !is_cp2112ek_connected() {
                eprintln!("CP2112-EK not connected. Skipping test!");
                panic!();
            }
            HidDevice::new().expect("Test failed! Something went wrong on initializing the library")
        });
    }

    #[test]
    fn test_cp2112_get_num_device() {
        self::set_up();
        let cp2112_hid_device = HID_DEVICE_CELL
            .get()
            .expect("Something wrong with the test!");
        let vid = 0x10c4;
        let pid = 0xea90;
        assert_eq!(
            cp2112_hid_device.hid_device_get_num_hid_devices(vid, pid),
            1
        );
    }

    #[test]
    fn test_cp2112_get_manufacturer_string() {
        self::set_up();
        let cp2112_hid_device = HID_DEVICE_CELL
            .get()
            .expect("Something wrong with the test!");
        let vid = 0x10c4;
        let pid = 0xea90;

        assert_eq!(
            cp2112_hid_device
                .hid_device_get_hid_string(0, vid, pid, HidStringType::Manufacturer)
                .unwrap(),
            "Silicon Laboratories"
        );
    }

    #[test]
    fn test_cp2112_get_product_string() {
        self::set_up();
        let cp2112_hid_device = HID_DEVICE_CELL
            .get()
            .expect("Something wrong with the test!");
        let vid = 0x10c4;
        let pid = 0xea90;
        assert_eq!(
            cp2112_hid_device
                .hid_device_get_hid_string(0, vid, pid, HidStringType::Product)
                .unwrap(),
            "CP2112 HID USB-to-SMBus Bridge"
        );
    }

    #[test]
    fn test_cp2112_get_vid_string() {
        self::set_up();
        let cp2112_hid_device = HID_DEVICE_CELL
            .get()
            .expect("Something wrong with the test!");
        let vid = 0x10c4;
        let pid = 0xea90;
        assert_eq!(
            cp2112_hid_device
                .hid_device_get_hid_string(0, vid, pid, HidStringType::Vid)
                .unwrap(),
            "10c4"
        );
    }

    #[test]
    fn test_cp2112_get_pid_string() {
        self::set_up();
        let cp2112_hid_device = HID_DEVICE_CELL
            .get()
            .expect("Something wrong with the test!");
        let vid = 0x10c4;
        let pid = 0xea90;
        assert_eq!(
            cp2112_hid_device
                .hid_device_get_hid_string(0, vid, pid, HidStringType::Pid)
                .unwrap(),
            "ea90"
        );
    }

    #[test]
    fn test_cp2112_get_attributes() {
        self::set_up();
        let cp2112_hid_device = HID_DEVICE_CELL
            .get()
            .expect("Something wrong with the test!");
        let vid = 0x10c4;
        let pid = 0xea90;
        assert_eq!(
            cp2112_hid_device
                .hid_device_get_hid_attributes(0, vid, pid)
                .unwrap(),
            (0x10c4, 0xea90, 0)
        );
    }
}
