use metal_oxide::device::{DisjointBuffer, ReadBuffer, thread_index};
use metal_oxide::{Device, LaunchConfig, Operation};

#[metal_oxide::kernel(requires(a.len() == out.len(), b.len() == out.len()))]
pub fn vec_add(a: ReadBuffer<f32>, b: ReadBuffer<f32>, mut out: DisjointBuffer<f32>) {
    let index = thread_index();
    if let Some(out) = out.get_mut(&index) {
        let i = index.get();
        *out = a.as_slice()[i] + b.as_slice()[i];
    }
}

fn main() -> metal_oxide::Result<()> {
    let device = Device::system_default()?;
    let a = device.buffer_from_slice(&[1.0_f32, 2.0, 3.0])?;
    let b = device.buffer_from_slice(&[10.0_f32, 20.0, 30.0])?;
    let mut out = device.buffer_zeroed::<f32>(3)?;
    let config = LaunchConfig::<256>::for_elements(3)?;

    vec_add(&device, config, &a, &b, &mut out).wait()?;

    assert_eq!(out.read().wait(), &[11.0, 22.0, 33.0]);
    Ok(())
}
