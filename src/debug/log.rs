/** Construct a new logger */
static mut LOGGER: IslandLogger = IslandLogger::new();

/** For passing an info log */
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        crate::debug::log::logger().info(format!($($arg)+));
    };
}

/** For passing an warning log */
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        crate::debug::log::logger().warn(format!($($arg)+));
    };
}

/** For passing an error log */
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        crate::debug::log::logger().error(format!($($arg)+));
    };
}

/** Get the logger */
pub fn logger() -> &'static mut IslandLogger {
    return unsafe { &mut LOGGER };
}

/** The logger for highground */
pub struct IslandLogger {
    log: String,
}

impl IslandLogger {
    /** Construct a new logger */
    pub const fn new() -> Self {
        return Self { log: String::new() };
    }

    /** Get the log */
    pub fn log(&self) -> String {
        return self.log.clone();
    }

    /** Log an info log */
    pub fn info<T>(&mut self, log: T)
    where
        T: Into<String>,
    {
        self.log = format!("{}{}", self.log, format!("[INFO]: {}\n", log.into()));
    }

    /** Log a warning log */
    pub fn warn<T>(&mut self, log: T)
    where
        T: Into<String>,
    {
        self.log = format!("{}{}", self.log, format!("[WARNING]: {}\n", log.into()));
    }

    /** Log a error log */
    pub fn error<T>(&mut self, log: T)
    where
        T: Into<String>,
    {
        self.log = format!("{}{}", self.log, format!("[ERROR]: {}\n", log.into()));
    }
}
