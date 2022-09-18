use std::ops::{Deref, DerefMut};

use druid::{Data, Lens};

#[derive(Debug, Clone)]
pub struct TimerTokenData(pub druid::TimerToken);

impl Data for TimerTokenData {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Deref for TimerTokenData {
    type Target = druid::TimerToken;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TimerTokenData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for TimerTokenData {
    fn default() -> Self {
        Self(druid::TimerToken::INVALID)
    }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {
    pub disabled: bool,
    pub token: String,
    pub start_button: &'static str,
    pub ip: String,
    pub run_time: usize,
    pub warning: String,
    pub use_tun: bool,
    pub auto_restart: bool,
    pub debug_mode: bool,
    pub kill_hiper_when_start: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            disabled: false,
            token: "".into(),
            start_button: "启动",
            ip: "".into(),
            warning: "".into(),
            run_time: 0,
            use_tun: true,
            auto_restart: true,
            debug_mode: false,
            kill_hiper_when_start: true,
        }
    }
}