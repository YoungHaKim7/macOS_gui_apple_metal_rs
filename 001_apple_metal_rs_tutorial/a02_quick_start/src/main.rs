use apple_metal::{MetalDevice, resource_options};

fn main() {
    let device = MetalDevice::system_default().expect("no Metal-capable GPU");
    println!("{} (registry id {})", device.name(), device.registry_id());

    let _queue = device.new_command_queue().expect("command queue");
    let buffer = device
        .new_buffer(4096, resource_options::STORAGE_MODE_SHARED)
        .expect("shared buffer");
    println!("allocated {} bytes", buffer.length());

    unsafe {
        buffer
            .write_bytes(0, b"CPU data")
            .expect("exclusive CPU write");
    }
}
