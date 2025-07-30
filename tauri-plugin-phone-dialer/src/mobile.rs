use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_phone_dialer);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<PhoneDialer<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.phonedialer", "ExamplePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_phone_dialer)?;
    Ok(PhoneDialer(handle))
}

/// Access to the phone-dialer APIs.
pub struct PhoneDialer<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> PhoneDialer<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        self.0
            .run_mobile_plugin("ping", payload)
            .map_err(Into::into)
    }

    pub fn dial_phone_number(&self, payload: DialPhoneRequest) -> crate::Result<DialPhoneResponse> {
        self.0
            .run_mobile_plugin("dialPhoneNumber", payload)
            .map_err(Into::into)
    }

    pub fn request_phone_permission(&self) -> crate::Result<PermissionResponse> {
        self.0
            .run_mobile_plugin("requestPhonePermission", ())
            .map_err(Into::into)
    }

    pub fn check_phone_permission(&self) -> crate::Result<PermissionResponse> {
        self.0
            .run_mobile_plugin("checkPhonePermission", ())
            .map_err(Into::into)
    }
}
