use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use windows::core::*;
use windows::Win32::System::Performance::PdhOpenQueryW;
use winapi::um::pdh::*;
use winapi::um::pdh::PdhAddCounterW;
use winapi::shared::winerror::*;
use winapi::ctypes::*;

fn main() {
    
    unsafe {

      let ans = "\\Processor(_Total)\\% Processor Time".as_ptr() as *const u16;

      let counter_path = PCWSTR::from_raw(ans);

      let mut h_query = 0;
      let void_h_query: *mut c_void = h_query as *mut c_void;

      let h_counter = 0;
      let mut_h_counter = h_counter as *mut c_void;
      let void_h_counter: *mut *mut c_void = mut_h_counter as *mut *mut c_void;

      let pdh_status = PdhOpenQueryW(PCWSTR::null(), 0, &mut h_query);

      if pdh_status != ERROR_SUCCESS as i32
      {
        println!("pdh_status: {}", pdh_status);
      }
      
      let pdh_status = PdhAddCounterW(void_h_query, counter_path.as_ptr(), 0, void_h_counter);

      let pdh_status = PdhCollectQueryData(void_h_query);

      println!("{:?}",counter_path);
  }
}
