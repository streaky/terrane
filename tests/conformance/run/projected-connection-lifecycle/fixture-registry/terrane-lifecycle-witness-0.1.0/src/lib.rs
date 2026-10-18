use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Debug)]
pub enum LifecycleError {
    MissingPath,
    AlreadyOpen,
    AlreadyClosed,
}

impl std::fmt::Display for LifecycleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::MissingPath => "path does not exist",
            Self::AlreadyOpen => "connection is already open",
            Self::AlreadyClosed => "connection is already closed",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for LifecycleError {}

static PATHS: LazyLock<Mutex<HashMap<String, bool>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn paths() -> &'static Mutex<HashMap<String, bool>> {
    &PATHS
}

pub struct ConnectionOptions {
    path: String,
    create_if_missing: bool,
}

impl ConnectionOptions {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            create_if_missing: true,
        }
    }

    pub fn filename(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn create_if_missing(mut self, create: bool) -> Self {
        self.create_if_missing = create;
        self
    }

    pub async fn connect(&self) -> Result<Connection, LifecycleError> {
        std::future::ready(()).await;
        let mut paths = paths().lock().expect("path registry poisoned");
        if !paths.contains_key(&self.path) && !self.create_if_missing {
            return Err(LifecycleError::MissingPath);
        }
        let is_open = paths.entry(self.path.clone()).or_insert(false);
        if *is_open {
            return Err(LifecycleError::AlreadyOpen);
        }
        *is_open = true;
        Ok(Connection {
            path: self.path.clone(),
        })
    }
}

pub struct Connection {
    path: String,
}

impl Connection {
    pub async fn close(self) -> Result<(), LifecycleError> {
        std::future::ready(()).await;
        let mut paths = paths().lock().expect("path registry poisoned");
        let is_open = paths
            .get_mut(&self.path)
            .ok_or(LifecycleError::AlreadyClosed)?;
        if !*is_open {
            return Err(LifecycleError::AlreadyClosed);
        }
        *is_open = false;
        Ok(())
    }
}
