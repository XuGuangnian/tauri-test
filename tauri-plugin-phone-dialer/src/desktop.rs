use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<PhoneDialer<R>> {
    Ok(PhoneDialer(app.clone()))
}

/// Access to the phone-dialer APIs.
pub struct PhoneDialer<R: Runtime>(AppHandle<R>);

impl<R: Runtime> PhoneDialer<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }

    pub fn dial_phone_number(
        &self,
        _payload: DialPhoneRequest,
    ) -> crate::Result<DialPhoneResponse> {
        // Desktop platforms don't support phone dialing
        Ok(DialPhoneResponse {
            success: false,
            message: "拨号功能仅在移动平台上支持".to_string(),
        })
    }

    pub fn request_phone_permission(&self) -> crate::Result<PermissionResponse> {
        // Desktop platforms don't need phone permissions
        Ok(PermissionResponse {
            success: false,
            message: "桌面平台不需要电话权限".to_string(),
        })
    }

    pub fn check_phone_permission(&self) -> crate::Result<PermissionResponse> {
        // Desktop platforms don't have phone permissions
        Ok(PermissionResponse {
            success: false,
            message: "桌面平台不支持电话功能".to_string(),
        })
    }
}
