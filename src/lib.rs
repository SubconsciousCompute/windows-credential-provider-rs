#![allow(nonstandard_style)]

mod credential_provider;
mod provider_factory;
mod tile;

use std::{
    ffi, mem, ptr,
    sync::atomic::{AtomicUsize, Ordering},
};

use provider_factory::ProviderFactory;

use windows::{
    core::{ComInterface, GUID, HRESULT, PCWSTR},
    Win32::{
        Foundation::{CLASS_E_CLASSNOTAVAILABLE, E_INVALIDARG, E_POINTER, S_FALSE, S_OK},
        System::{
            Com::{CoTaskMemAlloc, IClassFactory},
            Diagnostics::Debug::OutputDebugStringW,
        },
    },
};

static provider_factory_reference_count: AtomicUsize = AtomicUsize::new(0);
const WCP_CLSID: GUID = GUID::from_u128(0x12345678_1234_1234_1234_123456789123);

#[no_mangle]
extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut ffi::c_void,
) -> HRESULT {
    windbg_print("CredProvider : DllGetClassObject\n");

    // The "class ID" this credential provider is identified by. This value needs to
    // match the value used when registering the credential provider (see the .reg
    // script above)

    // Validate arguments
    if ppv.is_null() {
        return E_POINTER;
    }
    unsafe { *ppv = ptr::null_mut() };
    if rclsid.is_null() || riid.is_null() {
        return E_INVALIDARG;
    }

    let rclsid = unsafe { *rclsid };
    let riid = unsafe { *riid };
    // The following isn't strictly correct; a client *could* request an interface other
    // than `IClassFactory::IID`, which this implementation is simply failing.
    // This is safe, even if overly restrictive
    if rclsid != WCP_CLSID || riid != IClassFactory::IID {
        return CLASS_E_CLASSNOTAVAILABLE;
    }

    // Construct the factory object and return its `IClassFactory` interface
    let factory: IClassFactory = ProviderFactory.into();
    unsafe { *ppv = mem::transmute(factory) };
    S_OK
}

//System calls this function before unloading the Dll .
#[no_mangle]
extern "system" fn DllCanUnloadNow() -> HRESULT {
    windbg_print(
        format!(
            "CredProvider : DllCanUnloadNow called, Dll ref count = {}\n",
            provider_factory_reference_count.load(Ordering::SeqCst)
        )
        .as_str(),
    );
    //Only unload the dll if there are no more references
    if provider_factory_reference_count.load(Ordering::SeqCst) == 0 {
        return S_OK;
    };
    S_FALSE
}

//This function is used for printing to the WinDbg console
//Better to use tracing or smthing similar in the future to log stuff. Just add this function as a layer so we can see info on windbg console
fn windbg_print(message: &str) {
    // Convert the Rust string to a wide (UTF-16) string
    //We are adding null to the end of the string because we have to let the windows system know when the string ends
    let wide_message: Vec<u16> = message.encode_utf16().chain(Some(0)).collect();

    unsafe {
        OutputDebugStringW(PCWSTR::from_raw(wide_message.as_ptr()));
    }
}

fn create_wide_str_ptr(string: &str) -> ::windows::core::Result<*mut u16> {
    let wide_string: Vec<u16> = string.encode_utf16().chain(Some(0)).collect();
    let str_ptr: *mut u16 =
        unsafe { CoTaskMemAlloc(std::mem::size_of::<u16>() * wide_string.len()) as *mut u16 };
    if str_ptr.is_null() {
        return Err(E_POINTER.into());
    }
    unsafe {
        std::ptr::copy_nonoverlapping(wide_string.as_ptr(), str_ptr, wide_string.len());
    }
    Ok(str_ptr)
}
