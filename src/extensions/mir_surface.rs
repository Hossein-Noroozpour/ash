#![allow(dead_code)]
use prelude::*;
use std::mem;
use instance::Instance;
use entry::Entry;
use vk;
use std::ffi::CStr;
use ::RawPtr;

pub struct MirSurface {
    handle: vk::Instance,
    mir_surface_fn: vk::MirSurfaceFn,
}

impl MirSurface {
    pub fn new(entry: &Entry, instance: &Instance) -> Result<MirSurface, String> {
        let surface_fn = vk::MirSurfaceFn::load(|name| {
            unsafe {
                mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
            }
        })?;
        Ok(MirSurface {
            handle: instance.handle(),
            mir_surface_fn: surface_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_mir_surface\0").expect("Wrong extension string")
    }

    pub unsafe fn create_mir_surface_khr(&self,
                                           create_info: &vk::MirSurfaceCreateInfoKHR,
                                           allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                           -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::uninitialized();
        let err_code = self.mir_surface_fn
            .create_mir_surface_khr(self.handle,
                                      create_info,
                                      allocation_callbacks.as_raw_ptr(),
                                      &mut surface);
        match err_code {
            vk::Result::Success => Ok(surface),
            _ => Err(err_code),
        }
    }
}