use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use windows::core::*;
use windows::Win32::System::Performance::PdhOpenQueryW;
use winapi::um::pdh::*;
use winapi::shared::winerror::*;

fn main() {
    
    unsafe {

    let mut h_query:isize = 0;
    let pdh_status = PdhOpenQueryW(PCWSTR::null(), 0, &mut h_query);

    if pdh_status != ERROR_SUCCESS.try_into().unwrap()
	{
		println!("pdh_status: {}", pdh_status);
	}
  }
}
