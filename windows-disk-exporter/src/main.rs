extern crate winapi;

use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use windows::core::*;
use windows::Win32::System::Performance::PdhOpenQueryW;
use winapi::um::pdh::*;

fn main() {
    
    let mut hQuery:isize = 0;
    
    let pdhStatus = PdhOpenQueryW(PCWSTR::null(), 0, &mut hQuery);
}
