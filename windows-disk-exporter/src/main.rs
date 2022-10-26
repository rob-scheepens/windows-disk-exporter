use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};

use windows::{core::*, Win32::System::Performance::*};
 
fn main() {
    unsafe {
        let mut query = 0;
        PdhOpenQueryW(None, 0, &mut query);
 
        let mut disksecwrite = 0;
        PdhAddCounterW(
            query,
            w!("\\physicaldisk(1)\\avg. disk sec/write"),
            0,
            &mut disksecwrite
        );
 
        let mut disksecread = 0;
        PdhAddCounterW(
            query,
            w!("\\physicaldisk(1)\\avg. disk sec/read"),
            0,
            &mut disksecread
        );

        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
            PdhCollectQueryData(query);
 
            let mut value_write = Default::default();
            if 0 == PdhGetFormattedCounterValue(disksecwrite, PDH_FMT_DOUBLE, None, &mut value_write) {
	    
	    let mut value_read = Default::default();
	    if 0 == PdhGetFormattedCounterValue(disksecread, PDH_FMT_DOUBLE, None, &mut value_read) {

                println!("{:.4}\t  {:.4}", value_write.Anonymous.doubleValue * 1000.0, value_read.Anonymous.doubleValue * 1000.0);
            }}
        }
    }
}