use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.login1.Session",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1/session/auto",
    gen_blocking = true,
    gen_async = false,
    blocking_name = "SessionProxy"
)]
pub trait Session {
    fn set_brightness(&self, subsystem: &str, name: &str, brightness: u32) -> zbus::Result<()>;
}
