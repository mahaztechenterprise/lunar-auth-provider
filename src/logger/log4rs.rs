use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
      .with_ansi(false)
      .init();
    });
}