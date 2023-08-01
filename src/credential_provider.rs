use crate::{create_wide_str_ptr, tile::Tile, windbg_print};
use windows::{
    core::{implement, PWSTR},
    Win32::{
        Foundation::{E_INVALIDARG, E_NOTIMPL, E_POINTER, S_OK},
        System::Com::CoTaskMemAlloc,
        UI::Shell::{
            ICredentialProvider, ICredentialProviderCredential, ICredentialProvider_Impl,
            CPFG_CREDENTIAL_PROVIDER_LABEL, CPFG_LOGON_PASSWORD, CPFG_LOGON_USERNAME,
            CPFT_EDIT_TEXT, CPFT_LARGE_TEXT, CPFT_PASSWORD_TEXT, CPFT_SUBMIT_BUTTON,
            CPUS_CHANGE_PASSWORD, CPUS_CREDUI, CPUS_LOGON, CPUS_UNLOCK_WORKSTATION,
            CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR,
            CREDENTIAL_PROVIDER_USAGE_SCENARIO,
        },
    },
};

#[implement(ICredentialProvider)]
pub struct CredentialProvider {}

impl CredentialProvider {
    pub fn new() -> Self {
        windbg_print("CredProvider : New Credential Provider\n");
        Self {}
    }
}

impl Default for CredentialProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ICredentialProvider_Impl for CredentialProvider {
    //SetUsageScenario is called by the system to inform us about which scenario is currently occuring
    fn SetUsageScenario(
        &self,
        cpus: CREDENTIAL_PROVIDER_USAGE_SCENARIO,
        _dwflags: u32,
    ) -> windows::core::Result<()> {
        windbg_print(format!("CredProvider : SetUsageScenario  {:?} \n", cpus).as_str());

        //Just returning E_NOTIMPL for change password and CRED_UI scenarios, because we are not targetting them.
        match cpus {
            CPUS_LOGON | CPUS_UNLOCK_WORKSTATION => Err(S_OK.into()),
            CPUS_CHANGE_PASSWORD | CPUS_CREDUI => Err(E_NOTIMPL.into()),
            _ => Err(E_INVALIDARG.into()),
        }
    }

    fn SetSerialization(
        &self,
        _pcpcs: *const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION,
    ) -> ::windows::core::Result<()> {
        windbg_print(format!(" {} \n", std::line!()).as_str());
        todo!()
    }

    fn Advise(
        &self,
        _pcpe: std::option::Option<&windows::Win32::UI::Shell::ICredentialProviderEvents>,
        _upadvisecontext: usize,
    ) -> ::windows::core::Result<()> {
        windbg_print("CredProvider : Advise\n");
        Err(E_NOTIMPL.into())
    }

    fn UnAdvise(&self) -> ::windows::core::Result<()> {
        windbg_print("CredProvider : UnAdvise\n");
        Err(E_NOTIMPL.into())
    }

    fn GetFieldDescriptorAt(
        &self,
        dwindex: u32,
    ) -> ::windows::core::Result<*mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR> {
        windbg_print(format!("CredProvider : GetFieldDescriptorAt index {} \n", dwindex).as_str());
        match dwindex {
            0 => {
                let field_descriptor = unsafe {
                    CoTaskMemAlloc(std::mem::size_of::<CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR>())
                        as *mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR
                };
                if field_descriptor.is_null() {
                    return Err(E_POINTER.into());
                }

                unsafe {
                    *field_descriptor = CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
                        dwFieldID: 0,
                        cpft: CPFT_LARGE_TEXT,
                        pszLabel: PWSTR::from_raw(create_wide_str_ptr(
                            "Windows Credential Provider in Rust",
                        )?),
                        guidFieldType: CPFG_CREDENTIAL_PROVIDER_LABEL,
                    };
                }
                return Ok(field_descriptor);
            }
            1 => {
                let field_descriptor = unsafe {
                    CoTaskMemAlloc(std::mem::size_of::<CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR>())
                        as *mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR
                };
                if field_descriptor.is_null() {
                    return Err(E_POINTER.into());
                }

                unsafe {
                    *field_descriptor = CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
                        dwFieldID: 1,
                        cpft: CPFT_SUBMIT_BUTTON,
                        pszLabel: PWSTR::from_raw(create_wide_str_ptr(
                            "Windows Credential Provider in Rust",
                        )?),
                        guidFieldType: windows::core::GUID::zeroed(),
                    };
                }

                return Ok(field_descriptor);
            }
            2 => {
                let field_descriptor = unsafe {
                    CoTaskMemAlloc(std::mem::size_of::<CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR>())
                        as *mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR
                };
                if field_descriptor.is_null() {
                    return Err(E_POINTER.into());
                }

                unsafe {
                    *field_descriptor = CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
                        dwFieldID: 2,
                        cpft: CPFT_EDIT_TEXT,
                        pszLabel: PWSTR::from_raw(create_wide_str_ptr("Username")?),
                        guidFieldType: CPFG_LOGON_USERNAME,
                    };
                }

                return Ok(field_descriptor);
            }
            3 => {
                let field_descriptor = unsafe {
                    CoTaskMemAlloc(std::mem::size_of::<CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR>())
                        as *mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR
                };
                if field_descriptor.is_null() {
                    return Err(E_POINTER.into());
                }

                unsafe {
                    *field_descriptor = CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
                        dwFieldID: 3,
                        cpft: CPFT_PASSWORD_TEXT,
                        pszLabel: PWSTR::from_raw(create_wide_str_ptr("Password")?),
                        guidFieldType: CPFG_LOGON_PASSWORD,
                    };
                }

                return Ok(field_descriptor);
            }
            _ => {}
        }

        Err(E_INVALIDARG.into())
    }

    fn GetCredentialAt(
        &self,
        dwindex: u32,
    ) -> ::windows::core::Result<ICredentialProviderCredential> {
        windbg_print(format!("CredProvider : GetCredentialAt index {} \n", dwindex).as_str());
        if dwindex == 0 {
            return Ok(ICredentialProviderCredential::from(Tile::default()));
        }

        Err(E_INVALIDARG.into())
    }

    fn GetCredentialCount(
        &self,
        pdwcount: *mut u32,
        pdwdefault: *mut u32,
        pbautologonwithdefault: *mut windows::Win32::Foundation::BOOL,
    ) -> ::windows::core::Result<()> {
        windbg_print("CredProvider : GetCredentialCount\n");
        unsafe {
            *pdwcount = 1;
            *pdwdefault = 0;
            *pbautologonwithdefault = false.into();
        }
        Err(S_OK.into())
    }

    fn GetFieldDescriptorCount(&self) -> ::windows::core::Result<u32> {
        windbg_print("CredProvider : GetFieldDescriptorCount\n");
        Ok(4)
    }
}
