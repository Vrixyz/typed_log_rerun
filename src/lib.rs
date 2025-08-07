pub static RERUN_LOGGER: RerunLogger = RerunLogger;

use std::{any::TypeId, sync::atomic::AtomicU32};

use typed_log::Loggable;

pub fn register_typed_loggers() {
    typed_log::push_log_impl(incr_main_time);
    typed_log::push_log_impl(clear);
}

pub struct RerunLogger;

static RR_CELL: std::sync::OnceLock<rerun::RecordingStream> = std::sync::OnceLock::new();
pub static TIME: AtomicU32 = AtomicU32::new(0);

pub fn init_rr() {
    let _ = RR_CELL.get_or_init(|| {
        rerun::RecordingStreamBuilder::new("rerun_logger")
            .spawn()
            .unwrap()
    });
    get_rr().set_duration_secs("main", 0);
}

/// Retrieves the rerun logger.
pub fn get_rr() -> &'static rerun::RecordingStream {
    RR_CELL.get().unwrap()
}

pub struct IncrementTime(pub u32);
impl Loggable for IncrementTime {
    fn type_id(&self) -> std::any::TypeId {
        TypeId::of::<Self>()
    }
}

pub struct Clear(pub String);
impl Loggable for Clear {
    fn type_id(&self) -> std::any::TypeId {
        TypeId::of::<Self>()
    }
}

/// Increases the time of "main" rerun timeline
pub fn incr_main_time(increment: &IncrementTime) {
    let time = TIME.fetch_add(increment.0, core::sync::atomic::Ordering::Relaxed);
    get_rr().set_duration_secs("main", time + increment.0);
}

/// Clears recursively elements
pub fn clear(clear: &Clear) {
    get_rr()
        .log(clear.0.as_str(), &rerun::Clear::recursive())
        .unwrap();
}
