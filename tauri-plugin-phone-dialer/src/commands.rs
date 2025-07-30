use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::PhoneDialerExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.phone_dialer().ping(payload)
}
