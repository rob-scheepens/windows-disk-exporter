use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use windows::core::*;
use windows::Win32::System::Performance::PdhOpenQueryW;
use winapi::um::pdh::*;
use winapi::shared::winerror::*;

fn main() {
    
    unsafe {

    let counter_path = String::from("Test");
    let mut h_query:isize = 0;
    let mut h_counter:isize = 0;
    let pdh_status = PdhOpenQueryW(PCWSTR::null(), 0, &mut h_query);

    if pdh_status != ERROR_SUCCESS as i32
	{
		println!("pdh_status: {}", pdh_status);
	}

    let pdh_status = PdhAddCounterW(h_query, counter_path, 0, h_counter);

    println!("{}",counter_path);
  }
}
