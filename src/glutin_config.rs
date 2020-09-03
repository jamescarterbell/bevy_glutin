/// A resource for configuring usage of the `rust_glutin` library.
#[derive(Default)]
pub struct GlutinConfig {
    /// Configures the glutin library to return control to the main thread after
    /// the [run](bevy_app::App::run) loop is exited. Glutin strongly recommends
    /// avoiding this when possible. Before using this please read and understand
    /// the [caveats](glutin::platform::desktop::EventLoopExtDesktop::run_return)
    /// in the glutin documentation.
    ///
    /// This feature is only available on desktop `target_os` configurations.
    /// Namely `windows`, `macos`, `linux`, `dragonfly`, `freebsd`, `netbsd`, and
    /// `openbsd`. If set to true on an unsupported platform
    /// [run](bevy_app::App::run) will panic.
    pub return_from_run: bool,
}
