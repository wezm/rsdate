use std::ffi::CStr;
use std::mem;

use chrono::{DateTime, Local, TimeZone};
use libc::{c_int, suseconds_t, time_t, timeval, timezone};
use log::error;

use crate::error::get_errno;

pub fn change_system_time<Tz: TimeZone>(t: DateTime<Tz>) -> Result<(), c_int> {
    let date_time = t.with_timezone(&Local);
    let mut time_value: timeval = unsafe { mem::zeroed() };
    time_value.tv_sec = date_time.timestamp() as time_t;
    time_value.tv_usec = date_time.timestamp_subsec_micros() as suseconds_t;

    let res = unsafe {
        let mock_tz: *const timezone = std::ptr::null();
        if libc::settimeofday(&time_value as *const timeval, mock_tz) < 0 {
            let errno = get_errno();
            let strerr = libc::strerror(errno);
            if !strerr.is_null() {
                let err = CStr::from_ptr(strerr).to_string_lossy();
                error!("unable to set time: {}", err);
            } else {
                error!("unable to set time due to unknown error");
            }
            errno
        } else {
            0
        }
    };

    if res == 0 {
        Ok(())
    } else {
        Err(res)
    }
}
