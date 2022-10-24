use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use windows::core::*;
use windows::Win32::System::Performance::PdhOpenQueryW;
use winapi::ctypes::*;
use winapi::um::pdh::*;
use winapi::shared::winerror::*;

fn main() {
    
    unsafe {
      let ans = "\\Processor(_Total)\\% Processor Time".as_ptr() as *const u16;
      let counter_path = PCWSTR::from_raw(ans);
      let mut h_query = 0;
      let h_counter: *mut *mut c_void = 0 as *mut c_void as *mut *mut c_void;

      let pdh_status = PdhOpenQueryW(PCWSTR::null(), 0, &mut h_query);
      if pdh_status != ERROR_SUCCESS as i32
      {
        println!("pdh_status: {}", pdh_status);
      }
      let pdh_status = PdhAddCounterW(h_query as *mut c_void , counter_path.as_ptr(), 0, h_counter);
      let pdh_status = PdhCollectQueryData(h_query as *mut c_void);
      println!("{:?}",counter_path);
  }
}
