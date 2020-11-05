pub trait LoggerRepository {
    fn print(&self, message: &str);
}


#[derive(Debug)]
pub struct LoggerInteractor<T>
where
    T: LoggerRepository,
{
    logger_repository: T,
}


impl<T: LoggerRepository> LoggerInteractor<T> {
    pub fn new(logger: T) -> LoggerInteractor<T> {
        LoggerInteractor { logger_repository: logger }
    }

    pub fn print(&self, message: &str) {
        self.logger_repository.print(message);
    }
}

