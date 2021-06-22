use rocket::{Build, Config, Rocket, log::LogLevel};

pub trait AppConfigExt {
    fn impl_app_config(self) -> Self;
}

impl AppConfigExt for Rocket<Build> {
    fn impl_app_config(self) -> Self {
        let mut server_config = Config::default();
        server_config.log_level = LogLevel::Off;

        self.configure(server_config)
    }
}