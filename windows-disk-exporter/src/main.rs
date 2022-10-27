mod cli;

use clap::Parser;
use env_logger::{Builder, Env};
use prometheus_exporter::prometheus::{proto, register_int_gauge, IntGauge};
use std::net::IpAddr;
use std::net::SocketAddr;
use std::string::ToString;
use std::vec;
use windows::{core::*, Win32::System::Performance::*};

struct DiskMetric {
    prometheus_type: proto::MetricType, // TODO (cbwest): expand beyond Gauge types.
    windows_suffix: String,
    prometheus_name: String,
    prometheus_help: String,
    exported_metric: Option<IntGauge>,
}

fn get_metric_mapping() -> Vec<DiskMetric> {
    return vec![DiskMetric {
        prometheus_type: proto::MetricType::GAUGE,
        windows_suffix: "Current Disk Queue Length".to_string(),
        prometheus_name: "disk_queue_count".to_string(),
        prometheus_help: "Current Disk Queue Length".to_string(),
        exported_metric: Option::None,
    }];
}

unsafe fn update_metrics(disks: &Vec<String>, metric_mapping: &Vec<DiskMetric>) {
    // Create Performance Query
    let mut query = 0;
    match PdhOpenQueryW(None, 0, &mut query) {
        PDH_CSTATUS_VALID_DATA => println!("Successfully created query."),
        _ => panic!("Unable to create query."),
    }

    let mut disksecread = 0;
    match PdhAddCounterW(
        query,
        w!("\\physicaldisk(1)\\avg. disk sec/read"),
        0,
        &mut disksecread,
    ) {
        0 => println!("Successfully added counter to query."),
        _ => panic!("Unable to add counter to query."),
    }

    match PdhCollectQueryData(query) {
        0 => println!("Successfully executed query."),
        _ => panic!("Unable to execute query."),
    }

    let mut value_read = Default::default();
    PdhGetFormattedCounterValue(disksecread, PDH_FMT_DOUBLE, None, &mut value_read);
    match value_read.CStatus as i32 {
        PDH_CSTATUS_NEW_DATA => println!("New data has arrived."),
        PDH_CSTATUS_VALID_DATA => println!("Same old data, still valid."),
        PDH_CSTATUS_BAD_COUNTERNAME => panic!("PDH_CSTATUS_BAD_COUNTERNAME"),
        PDH_CSTATUS_ITEM_NOT_VALIDATED => panic!("PDH_CSTATUS_ITEM_NOT_VALIDATED"),
        PDH_CSTATUS_INVALID_DATA => panic!("PDH_CSTATUS_INVALID_DATA"),
        PDH_CSTATUS_NO_COUNTER => panic!("PDH_CSTATUS_NO_COUNTER"),
        PDH_CSTATUS_NO_COUNTERNAME => panic!("PDH_CSTATUS_NO_COUNTERNAME"),
        PDH_CSTATUS_NO_INSTANCE => panic!("PDH_CSTATUS_NO_INSTANCE"),
        PDH_CSTATUS_NO_MACHINE => panic!("PDH_CSTATUS_NO_MACHINE"),
        PDH_CSTATUS_NO_OBJECT => panic!("PDH_CSTATUS_NO_OBJECT"),
        _ => panic!("Unknown error!"),
    }
    println!("Retrieved longValue: {}", value_read.Anonymous.longValue);
    println!(
        "Retrieved doubleValue: {}",
        value_read.Anonymous.doubleValue
    );
    println!("Retrieved largeValue: {}", value_read.Anonymous.largeValue);
    // TODO (cbwest): Add the returned value to a Prometheus metric.
    // TODO (cbwest): Add labels to the Prometheus metric.
}

fn main() {
    // Setup logger with default level info so we can see the messages from
    // prometheus_exporter.
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let args = cli::Args::parse();
    let binding = SocketAddr::new(IpAddr::V4(args.ipaddr), args.port);
    // Will create an exporter and start the http server using the given binding.
    // If the webserver can't bind to the given binding it will fail with an error.
    // let mut exporter = prometheus_exporter::Builder::new(binding).with_registry(registry);
    let exporter = prometheus_exporter::start(binding).unwrap();

    // Get statically-created list of Windows->Prometheus disk metrics.
    let mut metric_mapping = get_metric_mapping();

    // Export Gauges for each metric. Gauges support assignment of values via set(),
    // whereas Counters only support manipulation via inc() and inc_by().
    for disk_metric in metric_mapping.iter_mut() {
        disk_metric.exported_metric = Some(
            register_int_gauge!(
                disk_metric.prometheus_name.to_string(),
                disk_metric.prometheus_help.to_string(),
            )
            .unwrap(),
        );
    }

    // Disks should be enumerated properly.
    let disks: Vec<String> = vec!["physicaldisk(1)".to_string()];

    loop {
        let guard = exporter.wait_request();
        unsafe {
            update_metrics(&disks, &metric_mapping);
        }
        drop(guard);
    }
}
