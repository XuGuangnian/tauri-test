use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::PhoneDialerExt;
use crate::Result;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.phone_dialer().ping(payload)
}

#[command]
pub(crate) async fn dial_phone_number<R: Runtime>(
    app: AppHandle<R>,
    payload: DialPhoneRequest,
) -> Result<DialPhoneResponse> {
    app.phone_dialer().dial_phone_number(payload)
}

#[command]
pub(crate) async fn request_phone_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<PermissionResponse> {
    app.phone_dialer().request_phone_permission()
}

#[command]
pub(crate) async fn check_phone_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<PermissionResponse> {
    app.phone_dialer().check_phone_permission()
}
