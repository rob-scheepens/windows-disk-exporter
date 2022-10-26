use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};

use windows::{core::*, Win32::System::Performance::*};
 
fn main() {
    unsafe {
        let mut query = 0;
        PdhOpenQueryW(None, 0, &mut query);
 
        let mut counter = 0;
        PdhAddCounterW(
            query,
            w!("\\physicaldisk(1)\\avg. disk sec/write"),
            0,
            &mut counter,
        );
 
        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
            PdhCollectQueryData(query);
 
            let mut value = Default::default();
            if 0 == PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, None, &mut value) {
                println!("{:.4}", value.Anonymous.doubleValue * 1000.0);
            }
        }
    }
}