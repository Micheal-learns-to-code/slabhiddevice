/*
 *
 * This file is part of the rust-slabhid project.
 *
 * Licensed under the MIT License. You may obtain a copy of the License at
 *
 *     https://opensource.org/licenses/MIT
 *
 * This code is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement.
 *
 * Note: This project is not affiliated with, endorsed by, or in any way associated with Silicon Labs. For more information about Silicon Labs and their products, please visit their official website at https://www.silabs.com.
 */
use std::{fmt, result};

/// A result of a function that may return a `HidDeviceError`.
pub type HidDeviceResult<T> = result::Result<T, HidDeviceError>;

/// Error returned by the library
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HidDeviceError {
    /// Invalid handle
    InvalidHandle,

    /// Invalid parameter
    InvalidParameter,

    /// Specified buffer is not large enough to return requested data.
    InvalidBufferSize,

    /// Function not supported or umimplemted in this platform
    FunctionNotSupported,

    /// A system error occurred
    SystemError,

    /// The specified device index was invalid or the device does not exist or is inaccessible.
    DeviceNotFound,

    /// The device must be opened prior to calling the function.
    DeviceNotOpened,

    /// The device is already opened and cannot be re-opened.
    DeviceAlreadyOpened,

    /// The get or set report function returned due to a timeout.
    TransferTimeout,

    /// The host failed to communicate with the device or function parameters are incorrect.
    TransferFailed,

    /// Cannot retrieve device path
    CannotGetHidInfo,

    /// Other error
    Other,
}

impl fmt::Display for HidDeviceError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        fmt.write_str(match self {
            HidDeviceError::InvalidHandle => "Invalid Handle",
            HidDeviceError::InvalidParameter => "Specified (typically non-buffer related) parameter is wrong or unexpected",
            HidDeviceError::InvalidBufferSize => "Specified buffer is not large enough to return requested data",
            HidDeviceError::FunctionNotSupported => {
                "Function not supported or unimplemented on this platform"
            }
            HidDeviceError::SystemError => "System error ",
            HidDeviceError::DeviceNotFound => "The specified device index was invalid or the device does not exist or is inaccessible",
            HidDeviceError::DeviceNotOpened => "The device must be opened prior to calling the function",
            HidDeviceError::DeviceAlreadyOpened => "The device is already opened and cannot be re-opened.",
            HidDeviceError::TransferTimeout => "The get or set report function returned due to a timeout",
            HidDeviceError::TransferFailed => "The host failed to communicate with the device or function parameters are incorrect",
            HidDeviceError::CannotGetHidInfo => "Cannot retrieve device path",
            HidDeviceError::Other => "Other error",
        })
    }
}

/*
impl From<rusb::Error> for HidDeviceError {
    fn from(e: rusb::Error) -> Self {
        match e {
            rusb::Error::Io => Self::DeviceIoFailed,
            rusb::Error::InvalidParam => Self::InvalidParameter,
            rusb::Error::Access => Self::InvalidAccessType,
            rusb::Error::NoDevice => Self::DeviceNotFound,
            rusb::Error::NotFound => Self::DeviceNotFound,
            rusb::Error::Busy => Self::Other,
            rusb::Error::Timeout => Self::CommandFailed,
            rusb::Error::Overflow => Self::GlobalDataError,
            rusb::Error::Pipe => Self::GlobalDataError,
            rusb::Error::Interrupted => Self::GlobalDataError,
            rusb::Error::NoMem => Self::GlobalDataError,
            rusb::Error::NotSupported => Self::FunctionNotSupported,
            rusb::Error::BadDescriptor => Self::CommandFailed,
            rusb::Error::Other => Self::Other,
        }
    }
}
*/
impl std::error::Error for HidDeviceError {}
