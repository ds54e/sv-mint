#[derive(Clone, Copy, Debug)]
pub struct TransportLimits {
    pub stdout_max: usize,
    pub stderr_max: usize,
}

impl Default for TransportLimits {
    fn default() -> Self {
        Self {
            stdout_max: 16 * 1024 * 1024,
            stderr_max: 4 * 1024 * 1024,
        }
    }
}
