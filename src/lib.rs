// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-system-logger: system logger component model.
//!
//! Real implementation: loggers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Logger {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemLoggerError {
    EmptyLoggerName,
    DuplicateLogger(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemLoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLoggerName => write!(f, "logger name must not be empty"),
            Self::DuplicateLogger(id) => write!(f, "duplicate logger {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemLoggerError {}

pub struct SystemLogger {
    pub name: String,
    loggers: BTreeMap<String, Logger>,
    events: BTreeMap<String, Event>,
}

impl SystemLogger {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            loggers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_logger(&mut self, item: Logger) -> Result<(), SystemLoggerError> {
        if item.name.trim().is_empty() {
            return Err(SystemLoggerError::EmptyLoggerName);
        }
        let id = item.id.clone();
        if self.loggers.contains_key(&id) {
            return Err(SystemLoggerError::DuplicateLogger(id.clone()));
        }
        self.loggers.insert(id, item);
        Ok(())
    }

    pub fn logger_count(&self) -> usize {
        self.loggers.len()
    }

    pub fn get_logger(&self, id: &str) -> Option<&Logger> {
        self.loggers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemLoggerError> {
        if event.subject.trim().is_empty() {
            return Err(SystemLoggerError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_logger(id: &str, name: &str) -> Logger {
        Logger {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemLogger::new("SystemLogger");
        assert_eq!(m.name, "SystemLogger");
        assert_eq!(m.logger_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_logger() {
        let mut m = SystemLogger::new("SystemLogger");
        m.add_logger(sample_logger("a1", "Alpha")).unwrap();
        assert_eq!(m.logger_count(), 1);
        assert!(m.get_logger("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemLogger::new("SystemLogger");
        assert!(m.add_logger(sample_logger("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemLogger::new("SystemLogger");
        m.add_logger(sample_logger("x", "One")).unwrap();
        assert!(m.add_logger(sample_logger("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemLogger::new("SystemLogger");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
