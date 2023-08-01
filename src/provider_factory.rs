use crate::{
    credential_provider::CredentialProvider, provider_factory_reference_count, windbg_print,
};
use std::{mem, ptr, sync::atomic::Ordering};
use windows::{
    core::{implement, ComInterface},
    Win32::{
        Foundation::{CLASS_E_NOAGGREGATION, E_INVALIDARG, E_NOINTERFACE, E_POINTER},
        System::Com::IClassFactory_Impl,
        UI::Shell::ICredentialProvider,
    },
};

#[implement(windows::Win32::System::Com::IClassFactory)]
pub struct ProviderFactory;

impl IClassFactory_Impl for ProviderFactory {
    fn CreateInstance(
        &self,
        punkouter: std::option::Option<&windows::core::IUnknown>,
        riid: *const windows::core::GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows::core::Result<()> {
        windbg_print("CredProvider : New Credential ProviderFactory\n");
        // Validate arguments
        if ppvobject.is_null() {
            return Err(E_POINTER.into());
        }
        unsafe { *ppvobject = ptr::null_mut() };
        if riid.is_null() {
            return Err(E_INVALIDARG.into());
        }
        let riid = unsafe { *riid };
        if punkouter.is_some() {
            return Err(CLASS_E_NOAGGREGATION.into());
        }

        // We're only handling requests for `IID_ICredentialProvider`
        if riid != ICredentialProvider::IID {
            return Err(E_NOINTERFACE.into());
        }

        // Construct credential provider and return it as an `ICredentialProvider`
        // interface
        let provider: ICredentialProvider = CredentialProvider::new().into();
        unsafe { *ppvobject = mem::transmute(provider) };
        Ok(())
    }

    //Apparently we need to keep track of number of LockServer calls.
    //More information at https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iclassfactory-lockserver
    fn LockServer(&self, flock: windows::Win32::Foundation::BOOL) -> windows::core::Result<()> {
        windbg_print(
            format!(
                "CredProvider : LockServer with flock = {}\n",
                flock.as_bool()
            )
            .as_str(),
        );
        if flock.as_bool() {
            provider_factory_reference_count.fetch_add(1, Ordering::SeqCst);
        } else {
            provider_factory_reference_count.fetch_sub(1, Ordering::SeqCst);
        }

        Ok(())
    }
}
