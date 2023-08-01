use std::{cell::RefCell, mem::transmute};

use windows::{
    core::{implement, PCWSTR, PWSTR},
    Win32::{
        Foundation::{self, BOOL, E_INVALIDARG, E_NOTIMPL, S_OK},
        Graphics::Gdi::HBITMAP,
        Security::{
            Authentication::Identity::{
                LsaConnectUntrusted, LsaDeregisterLogonProcess, LsaLookupAuthenticationPackage,
                MSV1_0_PACKAGE_NAME,
            },
            Credentials::CRED_PACK_PROTECTED_CREDENTIALS,
        },
        System::{Com::CoTaskMemAlloc, Kernel::STRING},
        UI::Shell::{
            ICredentialProviderCredential, ICredentialProviderCredential_Impl, CPFIS_FOCUSED,
            CPFS_DISPLAY_IN_BOTH, CPGSR_RETURN_CREDENTIAL_FINISHED, CPSI_NONE,
            CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION,
            CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE, CREDENTIAL_PROVIDER_FIELD_STATE,
            CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE, CREDENTIAL_PROVIDER_STATUS_ICON,
        },
    },
};

use crate::{create_wide_str_ptr, windbg_print, WCP_CLSID};

#[implement(ICredentialProviderCredential)]
///The winlogon queries IcredentialProviderCredential Interface to get informatin about what needs to be displayed on the Screen
pub struct Tile {
    username: RefCell<String>,
    password: RefCell<String>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            username: String::from("username").into(),
            password: String::from("password").into(),
        }
    }
}

// impl ICredentialProviderCredential2_Impl for Tile {
//     fn GetUserSid(&self) ->  ::windows::core::Result<::windows::core::PWSTR> {
//         windbg_print("CredProviderCred : GetUserSid \n");
//         Err(S_FALSE.into())
//     }
// }

impl ICredentialProviderCredential_Impl for Tile {
    fn Advise(
        &self,
        _pcpce: std::option::Option<
            &windows::Win32::UI::Shell::ICredentialProviderCredentialEvents,
        >,
    ) -> ::windows::core::Result<()> {
        windbg_print("CredProviderCred : Advise \n");
        Err(E_NOTIMPL.into())
    }

    fn UnAdvise(&self) -> ::windows::core::Result<()> {
        windbg_print("CredProviderCred : UnAdvise \n");
        Err(E_NOTIMPL.into())
    }

    fn SetSelected(&self) -> ::windows::core::Result<BOOL> {
        windbg_print("CredProviderCred : SetSelected \n");
        Err(E_NOTIMPL.into())
    }

    fn SetDeselected(&self) -> ::windows::core::Result<()> {
        windbg_print("CredProviderCred : SetDeSelected \n");
        Err(E_NOTIMPL.into())
    }

    fn GetFieldState(
        &self,
        dwfieldid: u32,
        pcpfs: *mut CREDENTIAL_PROVIDER_FIELD_STATE,
        pcpfis: *mut CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE,
    ) -> ::windows::core::Result<()> {
        windbg_print(
            format!(
                "CredProviderCred : Get field state index = {} \n",
                dwfieldid
            )
            .as_str(),
        );
        unsafe { *pcpfis = CPFIS_FOCUSED };
        unsafe { *pcpfs = CPFS_DISPLAY_IN_BOTH };
        Ok(())
    }

    fn GetStringValue(&self, dwfieldid: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        windbg_print(
            format!("CredProviderCred : GetStringValue index = {} \n", dwfieldid).as_str(),
        );
        match dwfieldid {
            0 => Ok(PWSTR::from_raw(create_wide_str_ptr(
                "Windows Credential Provider in Rust",
            )?)),
            2 => Ok(PWSTR::from_raw(create_wide_str_ptr("Username")?)),
            3 => Ok(PWSTR::from_raw(create_wide_str_ptr("Password")?)),
            _ => Err(E_INVALIDARG.into()),
        }
    }

    fn GetBitmapValue(&self, _dwfieldid: u32) -> ::windows::core::Result<HBITMAP> {
        windbg_print(format!(" {} \n", std::line!()).as_str());
        todo!()
    }

    fn GetCheckboxValue(
        &self,
        _dwfieldid: u32,
        _pbchecked: *mut BOOL,
        _ppszlabel: *mut ::windows::core::PWSTR,
    ) -> ::windows::core::Result<()> {
        windbg_print(format!(" {} \n", std::line!()).as_str());
        todo!()
    }

    fn GetSubmitButtonValue(&self, dwfieldid: u32) -> ::windows::core::Result<u32> {
        windbg_print(
            format!(
                "CredProviderCred : Get SUbmit button value = {} \n",
                dwfieldid
            )
            .as_str(),
        );
        Ok(3)
    }

    fn GetComboBoxValueCount(
        &self,
        _dwfieldid: u32,
        _pcitems: *mut u32,
        _pdwselecteditem: *mut u32,
    ) -> ::windows::core::Result<()> {
        windbg_print(format!(" {} \n", std::line!()).as_str());
        todo!()
    }

    fn GetComboBoxValueAt(
        &self,
        _dwfieldid: u32,
        _dwitem: u32,
    ) -> ::windows::core::Result<::windows::core::PWSTR> {
        windbg_print(format!(" {} \n", std::line!()).as_str());
        todo!()
    }

    fn SetStringValue(
        &self,
        dwfieldid: u32,
        psz: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<()> {
        match dwfieldid {
            2 => {
                *self.username.borrow_mut() = unsafe { psz.to_string().unwrap() };
            }
            3 => {
                *self.password.borrow_mut() = unsafe { psz.to_string().unwrap() };
            }
            _ => {
                return Err(E_INVALIDARG.into());
            }
        }
        Ok(())
    }

    fn SetCheckboxValue(&self, _dwfieldid: u32, _bchecked: BOOL) -> ::windows::core::Result<()> {
        windbg_print(format!(" {} \n", std::line!()).as_str());
        todo!()
    }

    fn SetComboBoxSelectedValue(
        &self,
        _dwfieldid: u32,
        _dwselecteditem: u32,
    ) -> ::windows::core::Result<()> {
        windbg_print(format!(" {} \n", std::line!()).as_str());
        todo!()
    }

    fn CommandLinkClicked(&self, _dwfieldid: u32) -> ::windows::core::Result<()> {
        windbg_print(format!(" {} \n", std::line!()).as_str());
        todo!()
    }

    fn GetSerialization(
        &self,
        pcpgsr: *mut CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE,
        pcpcs: *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION,
        ppszoptionalstatustext: *mut ::windows::core::PWSTR,
        pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON,
    ) -> ::windows::core::Result<()> {
        windbg_print("CredProviderCred : GetSerialization \n");
        unsafe {
            *pcpsioptionalstatusicon = CPSI_NONE;
        }

        unsafe {
            *ppszoptionalstatustext = PWSTR::null();
        }

        unsafe {
            *pcpgsr = CPGSR_RETURN_CREDENTIAL_FINISHED;
        }
        let size: *mut u32 = unsafe { transmute(&u32::default()) };
        let serialized_creds = unsafe { CoTaskMemAlloc(1000) as *mut u8 };
        let status = unsafe {
            windows::Win32::Security::Credentials::CredPackAuthenticationBufferW(
                CRED_PACK_PROTECTED_CREDENTIALS,
                PCWSTR::from_raw(create_wide_str_ptr(self.username.borrow().as_str())?),
                PCWSTR::from_raw(create_wide_str_ptr(self.password.borrow().as_str())?),
                Some(serialized_creds),
                size,
            )
        };

        windbg_print(
            format!(
                "CredProviderCred : CredPackAuthenticationBufferA status = {:?}, size = {:?}\n",
                status.as_bool(),
                unsafe { *size }
            )
            .as_str(),
        );

        unsafe {
            (*pcpcs).ulAuthenticationPackage = RetrieveNegotiateAuthPackage().unwrap();
            (*pcpcs).cbSerialization = *size;
            (*pcpcs).clsidCredentialProvider = WCP_CLSID;
            (*pcpcs).rgbSerialization = serialized_creds;
        }

        Err(S_OK.into())
    }

    fn ReportResult(
        &self,
        ntsstatus: Foundation::NTSTATUS,
        ntssubstatus: Foundation::NTSTATUS,
        _ppszoptionalstatustext: *mut ::windows::core::PWSTR,
        _pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON,
    ) -> ::windows::core::Result<()> {
        windbg_print(
            format!(
                "CredProviderCred : ReportResult status = {:?}, substatus = {:?} \n",
                ntsstatus, ntssubstatus
            )
            .as_str(),
        );
        Err(S_OK.into())
    }
}

fn RetrieveNegotiateAuthPackage() -> ::windows::core::Result<u32> {
    let mut lsahandle = Foundation::HANDLE::default();
    let _ = unsafe { LsaConnectUntrusted(std::ptr::addr_of_mut!(lsahandle)) };
    let package = unsafe { MSV1_0_PACKAGE_NAME.to_string().unwrap() };
    let auth_package: STRING = STRING {
        Length: package.len() as u16,
        MaximumLength: package.len() as u16,
        Buffer: unsafe { transmute(package.as_ptr()) },
    };
    let auth_package_ptr: *const STRING = unsafe { transmute(&auth_package) };
    let auth_id: *mut u32 = unsafe { transmute(&u32::default()) };
    let status = unsafe { LsaLookupAuthenticationPackage(lsahandle, auth_package_ptr, auth_id) };
    windbg_print(
        format!(
            "CredProviderCred : LsaLookupAuthenticationPackage status = {:?}, package = {:?}\n",
            status,
            unsafe { *auth_id }
        )
        .as_str(),
    );

    let _ = unsafe { LsaDeregisterLogonProcess(lsahandle) };
    Ok(unsafe { *auth_id })
}
