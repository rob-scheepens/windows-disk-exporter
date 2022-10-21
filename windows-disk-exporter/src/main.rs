use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use windows::core::*;
use windows::Win32::System::Performance::PdhOpenQueryW;
use winapi::um::pdh::*;
use winapi::shared::winerror::*;

fn main() {
    
    unsafe {

    let counter_path = String::from("\\Processor(_Total)\\% Processor Time");
    let mut h_query = 0;
    let mut h_counter = 0;
    let pdh_status = PdhOpenQueryW(PCWSTR::null(), 0, &mut h_query);

    if pdh_status != ERROR_SUCCESS as i32
	{
		println!("pdh_status: {}", pdh_status);
	}

    let pdh_status = PdhAddCounterW(&mut h_query, &counter_path, 0, h_counter);

    let pdh_status = PdhCollectQueryData(&mut h_query);

    println!("{}",counter_path);
  }
}
