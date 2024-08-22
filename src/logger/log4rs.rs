use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
      .with_ansi(false)
    //   .with_env_filter(
    //     tracing_subscriber::EnvFilter::from_default_env(),
    //   )
      .init();
    });
    // log4rs::init_file("log/log4rs.yaml", Default::default()).unwrap();
}