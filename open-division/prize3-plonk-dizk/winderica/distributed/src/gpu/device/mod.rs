use once_cell::sync::Lazy;
use rustacuda::error::CudaResult;

pub struct CudaContexts(Vec<rustacuda::context::Context>);

unsafe impl Send for CudaContexts {}
unsafe impl Sync for CudaContexts {}

pub fn build_device_list() -> CudaResult<(Vec<Device>, CudaContexts)> {
    let mut all_devices = Vec::new();
    let mut contexts = Vec::new();
    
    //let num_devices = rustacuda::device::Device::num_devices()?;
    //println!("Number of devices: {}", num_devices);
    println!("l {}", 1);
    rustacuda::init(rustacuda::CudaFlags::empty())?;
    println!("l {}", 2);
    for device in rustacuda::device::Device::devices()? {
    println!("l {}", 3);
        let device = device?;
        println!("l {}", 31);
        println!("{}", device.name().unwrap());
        let owned_context = rustacuda::context::Context::create_and_push(
            rustacuda::context::ContextFlags::MAP_HOST,
            device,
        )?;
        println!("l {}", 32);
        rustacuda::context::ContextStack::pop()?;
        println!("l {}", 4);
        let memory = device.total_memory()?;
        println!("l {}", 5);
        let compute_units =
            device.get_attribute(rustacuda::device::DeviceAttribute::MultiprocessorCount)? as u32;
        println!("l {}", 6);
        let compute_capability = (
            device.get_attribute(rustacuda::device::DeviceAttribute::ComputeCapabilityMajor)?
                as u32,
            device.get_attribute(rustacuda::device::DeviceAttribute::ComputeCapabilityMinor)?
                as u32,
               
        );
        println!("l {}", 7);
        let context = owned_context.get_unowned();
        println!("l {}", 8);
        contexts.push(owned_context);
        println!("l {}", 9);
        all_devices.push(Device { memory, compute_units, compute_capability, context });
    }
    println!("l {}", 10);
    let wrapped_contexts = CudaContexts(contexts);

    Ok((all_devices, wrapped_contexts))
}

pub struct Device {
    pub memory: usize,
    pub compute_units: u32,
    pub compute_capability: (u32, u32),
    pub context: rustacuda::context::UnownedContext,
}

pub static DEVICES: Lazy<(Vec<Device>, CudaContexts)> = Lazy::new(|| {
    build_device_list().unwrap()
});
