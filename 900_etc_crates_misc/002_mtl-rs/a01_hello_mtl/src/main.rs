use metal::{MTLDevice, MTLDeviceExt};

fn main() {
    let device = <dyn MTLDevice>::system_default().expect("No Metal device found");

    println!("Device: {:?}", device.name());
    println!("Unified memory: {}", device.has_unified_memory());
    println!(
        "Max threadgroup size: {:?}",
        device.max_threads_per_threadgroup()
    );
}
