use once_cell::sync::Lazy;

/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

pub struct ServiceContext {
}

impl ServiceContext {}


impl Default for ServiceContext {
    fn default() -> Self {
        Self { }
    }
}