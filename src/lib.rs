// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-startup-manager: startup manager component model.
//!
//! Real implementation: managers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manager {
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
pub enum StartupManagerError {
    EmptyManagerName,
    DuplicateManager(String),
    EmptyEventSubject,
}

impl fmt::Display for StartupManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyManagerName => write!(f, "manager name must not be empty"),
            Self::DuplicateManager(id) => write!(f, "duplicate manager {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for StartupManagerError {}

pub struct StartupManager {
    pub name: String,
    managers: BTreeMap<String, Manager>,
    events: BTreeMap<String, Event>,
}

impl StartupManager {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            managers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_manager(&mut self, item: Manager) -> Result<(), StartupManagerError> {
        if item.name.trim().is_empty() {
            return Err(StartupManagerError::EmptyManagerName);
        }
        let id = item.id.clone();
        if self.managers.contains_key(&id) {
            return Err(StartupManagerError::DuplicateManager(id.clone()));
        }
        self.managers.insert(id, item);
        Ok(())
    }

    pub fn manager_count(&self) -> usize {
        self.managers.len()
    }

    pub fn get_manager(&self, id: &str) -> Option<&Manager> {
        self.managers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), StartupManagerError> {
        if event.subject.trim().is_empty() {
            return Err(StartupManagerError::EmptyEventSubject);
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

    fn sample_manager(id: &str, name: &str) -> Manager {
        Manager {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = StartupManager::new("StartupManager");
        assert_eq!(m.name, "StartupManager");
        assert_eq!(m.manager_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_manager() {
        let mut m = StartupManager::new("StartupManager");
        m.add_manager(sample_manager("a1", "Alpha")).unwrap();
        assert_eq!(m.manager_count(), 1);
        assert!(m.get_manager("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = StartupManager::new("StartupManager");
        assert!(m.add_manager(sample_manager("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = StartupManager::new("StartupManager");
        m.add_manager(sample_manager("x", "One")).unwrap();
        assert!(m.add_manager(sample_manager("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = StartupManager::new("StartupManager");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
