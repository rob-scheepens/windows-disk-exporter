use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use windows::core::*;
use windows::Win32::System::Performance::PdhOpenQueryW;
use winapi::ctypes::*;
use winapi::um::pdh::*;
use winapi::shared::winerror::*;

fn main() {
    
    unsafe {
      let ans = "\\Processor(0)\\% Processor Time".as_ptr() as *const u16;
      let counter_path = PCWSTR::from_raw(ans);
      let mut h_query = 0;
      let h_counter: *mut *mut c_void = 0 as *mut c_void as *mut *mut c_void;


      let _pdh_status = PdhOpenQueryW(PCWSTR::null(), 0, &mut h_query);

      if _pdh_status != ERROR_SUCCESS as i32
      {
        println!("PdhOpenQueryW - _pdh_status: {}\n", _pdh_status);
      }

      let _pdh_status = PdhAddCounterW(h_query as *mut c_void , counter_path.as_ptr(), 0, h_counter);

      if _pdh_status != ERROR_SUCCESS as i32
      {
        println!("PdhAddCounterW - _pdh_status: {}\n", _pdh_status);
      }

      let _pdh_status = PdhAddCounterW(h_query as *mut c_void , counter_path.as_ptr(), 0, h_counter);
      let _pdh_status = PdhCollectQueryData(h_query as *mut c_void);
      println!("{:?}\n",counter_path);
  }
}
