pub enum BackendChoice {
    Winit,
    Drm,
}

pub fn select_backend() -> BackendChoice {
    // Default to winit for development until DRM is wired for production.
    BackendChoice::Winit
}
