use log::Log;

use crate::safe::{println, PrintType};

pub fn setup_logger() -> Result<(), log::SetLoggerError> {
    log::set_max_level(log::LevelFilter::Debug);
    log::set_logger(&Logger)
}

struct Logger;
impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        // Only allow logs from our own module
        _metadata.target().starts_with("crate::")
    }

    fn log(&self, record: &log::Record) {
        let print_type = match record.level() {
            log::Level::Error => PrintType::Error,
            log::Level::Warn => PrintType::Warning,
            log::Level::Info => PrintType::Normal,
            log::Level::Debug => PrintType::Info,
            log::Level::Trace => PrintType::Info,
        };

        // Filter out null bytes
        let message = record.args().to_string().replace('\0', "");

        // Ignore errors
        let _ = println(print_type, message);
    }

    // There is nothing to flush
    fn flush(&self) { }
}

pub mod prelude {
    pub use log::{info, warn, error, debug, trace};

    pub trait RebasePointerDisplay {
        fn rebase_display(&self) -> String;
    }

    impl RebasePointerDisplay for usize {
        fn rebase_display(&self) -> String {
            let base = *crate::rbx::BASE;

            match *self {
                0x0 => "nullptr".to_string(),
                x if x < base => format!("nullptr ({:#x})", x),
                x if x == base => "baseptr".to_string(),
                x => format!("{:#x}", x - base),
            }
        }
    }
    impl<T> RebasePointerDisplay for *const T {
        fn rebase_display(&self) -> String {
            (*self as usize).rebase_display()
        }
    }
}