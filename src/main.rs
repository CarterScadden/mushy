use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::image::view::ImageView;
use vulkano::image::{ImageAccess, ImageUsage, SwapchainImage};
use vulkano::instance::Instance;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::{Framebuffer, RenderPass, Subpass};
use vulkano::swapchain::{self, AcquireError, Swapchain, SwapchainCreationError};
use vulkano::sync::{self, FlushError, GpuFuture};
use vulkano::Version;
use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() {
    let required_ext = vulkano_win::required_extensions();

    let instance = Instance::new(
        None,
        vulkano::Version::V1_1,
        &required_ext,
        //&InstanceExtensions::none(),
        None,
    )
    .unwrap();

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    // "choose device extention we are going to use" ?
    // swapchain is needed to draw images to a surface
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };

    let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
        .filter(|&device| {
            // if the device says it doesnt have or wont allow what we need to make this applicion
            // remove it
            device
                .supported_extensions()
                .is_superset_of(&device_extensions)
        })
        .filter_map(|device| {
            // for every device, we need to find a "queue_family" that will issue our draw
            // commands,
            //
            // in more real world projects, you would find more queues that support other things
            // like "computation" and a "data transfer queue"
            device
                .queue_families()
                .find(|&queue_family| {
                    // check if supports the graphics we need, and if our "surface" supports this queue
                    queue_family.supports_graphics()
                        && surface.is_supported(queue_family).unwrap_or(false)
                })
                // return devuce and queue
                .map(|queue_family| (device, queue_family))
        })
        // at this point we may have found multiple devices, if we did we need to pick the best
        // one,
        // this can be changes to store the value before this, and have the user choose the device
        // they want to use, in this case we choose for them
        .min_by_key(|(device, _)| match device.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
        })
        .unwrap();

    // Some little debug infos.
    println!(
        "Using[benutzen] device[Gerät] {} (type[art]: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    let (device, mut queues) = Device::new(
        physical_device,   // the device to connect to
        &Features::none(), // optional features to use
        &physical_device
            .required_extensions()
            .union(&device_extensions), // extentions to use
        [(queue_family, 0.5)].iter().cloned(), // list of queues to use, where (queue, float) and float is 0.0 -> 1.0 showing priority, (not sure if 1 > 0 || 0 > 1)
    )
    .unwrap();

    // Since we can request multiple queues, the `queues` variable is in fact an iterator. We
    // only use one queue in this example, so we just retrieve the first and only element of the
    // iterator.
    let queue = qeues.next().unwrap();

    // Before we can draw on the surface, we have to create what is called a swapchain. Creating
    // a swapchain allocates the color buffers that will contain the image that will ultimately
    // be visible on the screen. These images are returned alongside the swapchain.
    let (mut swapchain, images) = {
        let device_capabilities = surface.capabilities(physical_device).unwrap();
        // https://github.com/vulkano-rs/vulkano/blob/master/examples/src/bin/triangle.rs
    };

    println!("Hello, world!");
}
