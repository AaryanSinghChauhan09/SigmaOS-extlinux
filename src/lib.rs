// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-extlinux: extlinux component model.
//!
//! Real implementation: extlinux_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtlinuxItem {
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
pub enum ExtlinuxError {
    EmptyExtlinuxItemName,
    DuplicateExtlinuxItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ExtlinuxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExtlinuxItemName => write!(f, "extlinux_item name must not be empty"),
            Self::DuplicateExtlinuxItem(id) => write!(f, "duplicate extlinux_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ExtlinuxError {}

pub struct Extlinux {
    pub name: String,
    extlinux_items: BTreeMap<String, ExtlinuxItem>,
    events: BTreeMap<String, Event>,
}

impl Extlinux {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            extlinux_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_extlinux_item(&mut self, item: ExtlinuxItem) -> Result<(), ExtlinuxError> {
        if item.name.trim().is_empty() {
            return Err(ExtlinuxError::EmptyExtlinuxItemName);
        }
        let id = item.id.clone();
        if self.extlinux_items.contains_key(&id) {
            return Err(ExtlinuxError::DuplicateExtlinuxItem(id.clone()));
        }
        self.extlinux_items.insert(id, item);
        Ok(())
    }

    pub fn extlinux_item_count(&self) -> usize {
        self.extlinux_items.len()
    }

    pub fn get_extlinux_item(&self, id: &str) -> Option<&ExtlinuxItem> {
        self.extlinux_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ExtlinuxError> {
        if event.subject.trim().is_empty() {
            return Err(ExtlinuxError::EmptyEventSubject);
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

    fn sample_extlinux_item(id: &str, name: &str) -> ExtlinuxItem {
        ExtlinuxItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Extlinux::new("Extlinux");
        assert_eq!(m.name, "Extlinux");
        assert_eq!(m.extlinux_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_extlinux_item() {
        let mut m = Extlinux::new("Extlinux");
        m.add_extlinux_item(sample_extlinux_item("a1", "Alpha")).unwrap();
        assert_eq!(m.extlinux_item_count(), 1);
        assert!(m.get_extlinux_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Extlinux::new("Extlinux");
        assert!(m.add_extlinux_item(sample_extlinux_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Extlinux::new("Extlinux");
        m.add_extlinux_item(sample_extlinux_item("x", "One")).unwrap();
        assert!(m.add_extlinux_item(sample_extlinux_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Extlinux::new("Extlinux");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
