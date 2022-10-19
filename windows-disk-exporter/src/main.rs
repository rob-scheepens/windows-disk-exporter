use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use windows::core::*;
use windows::Win32::System::Performance::*;
use winapi::um::pdh::*;

fn main() {
    
    let hQuery: PDH_HQUERY;

    pdhStatus = PdhOpenQueryW(PCWSTR::null(), 0, &hQuery);
}
