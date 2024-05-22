use std::time::Duration;

use anyhow::Context;
use rustbus::{
    connection::{ll_conn::force_finish_on_error, Timeout},
    MessageBuilder, RpcConn,
};

pub fn set_brightness(subsystem: &str, name: &str, brightness: u32) -> anyhow::Result<()> {
    let mut conn = RpcConn::system_conn(Timeout::Duration(Duration::from_secs(5)))
        .context("Could not open system bus")?;

    // Build the D-Bus message
    let mut call = MessageBuilder::new()
        .call("SetBrightness")
        .with_interface("org.freedesktop.login1.Session")
        .on("/org/freedesktop/login1/session/auto")
        .at("org.freedesktop.login1")
        .build();
    call.body
        .push_param3(subsystem, name, brightness)
        .context("Failed to push D-Bus call parameters")?;

    // Send the D-Bus message and keep its ID to wait for a response
    let id = conn
        .send_message(&mut call)
        .context("Failed to send D-Bus message")?
        .write_all()
        .map_err(force_finish_on_error)
        .context("Failed to write D-Bus message")?;

    // Retrieve the response
    let message = conn
        .wait_response(id, Timeout::Duration(Duration::from_secs(5)))
        .context("Failed to wait for D-Bus response")?;

    // Check if the response is an error
    if message.typ == rustbus::MessageType::Error {
        let message = message
            .body
            .parser()
            .get::<String>()
            .context("Failed to unmarshall D-Bus error")?;
        anyhow::bail!("D-Bus error: {message}");
    }

    Ok(())
}
