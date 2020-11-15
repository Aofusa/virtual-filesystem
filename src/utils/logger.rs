pub trait LoggerRepository {
    fn print(&self, message: &str);
}


#[derive(Debug)]
pub struct LoggerInteractor<T>
where
    T: LoggerRepository + Clone,
{
    logger_repository: T,
}


impl<T: LoggerRepository + Clone> LoggerInteractor<T> {
    pub fn new(logger: T) -> LoggerInteractor<T> {
        LoggerInteractor { logger_repository: logger }
    }

    pub fn get(&self) -> T {
        self.logger_repository.clone()
    }

    pub fn print(&self, message: &str) {
        self.logger_repository.print(message);
    }
}


#[derive(Clone)]
pub struct DefaultLoggerRepository {}
impl LoggerRepository for DefaultLoggerRepository {
    fn print(&self, _message: &str) {}
}

