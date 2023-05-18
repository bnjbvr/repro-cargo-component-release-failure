use wit_log as log;

struct Component;

impl bindings::messaging::Messaging for Component {
    fn init() {
        let _ = log::set_boxed_logger(Box::new(log::WitLog::new()));
        log::set_max_level(log::LevelFilter::Trace);
        log::trace!("Called the init() method \\o/");
    }
}

bindings::export!(Component);
