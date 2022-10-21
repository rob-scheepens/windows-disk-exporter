use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use windows::core::*;
use windows::Win32::System::Performance::PdhOpenQueryW;
use winapi::um::pdh::*;
use winapi::shared::winerror::*;

fn main() {
    
    unsafe {

    let counter_path = PCWSTR::from_raw("\\Processor(_Total)\\% Processor Time".as_ptr() as *const u16);
    let mut h_query = 0;
    let mut h_counter = 0;
    let pdh_status = PdhOpenQueryW(PCWSTR::null(), 0, &mut h_query);

    if pdh_status != ERROR_SUCCESS as i32
	{
		println!("pdh_status: {}", pdh_status);
	}
    let mut h_query = winapi::ctypes::c_void.as_ptr();
    let pdh_status = PdhAddCounterW(&mut h_query, counter_path.as_ptr(), 0, &mut h_counter );

    let pdh_status = PdhCollectQueryData(&mut h_query);

    println!("{:?}",counter_path);
  }
}
