use crate::objects::area::Area;
use crate::objects::line::Line;
use crate::objects::traits::Candidate;
use crate::traits::{HasSlots, Identifiable};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Slot {
    id: usize,
    value: Option<u8>,
    possible_variants: HashSet<u8>,
    restricted_variants: HashSet<u8>,
    row: Weak<RefCell<Line>>,
    column: Weak<RefCell<Line>>,
    area: Weak<RefCell<Area>>,
}

impl Slot {
    pub fn new(id: usize, value: Option<u8>) -> Self {
        let mut slot = Slot {
            id,
            value: None,
            possible_variants: HashSet::new(),
            restricted_variants: HashSet::new(),
            row: Weak::new(),
            column: Weak::new(),
            area: Weak::new(),
        };
        slot.set_value(value);
        slot
    }

    pub fn set_value(&mut self, value: Option<u8>) -> bool {
        // Skip setter if the value is already set.
        if self.value.is_some() {
            return false;
        }

        self.value = value;
        self.possible_variants = match value {
            Some(val) => {
                let mut set = HashSet::new();
                set.insert(val);
                set
            }
            None => (1..=9).collect(),
        };
        self.restricted_variants.clear();
        true
    }

    pub fn set_row(&mut self, row: Weak<RefCell<Line>>) {
        self.row = row;
    }

    pub fn set_column(&mut self, column: Weak<RefCell<Line>>) {
        self.column = column;
    }

    pub fn set_area(&mut self, area: Weak<RefCell<Area>>) {
        self.area = area;
    }

    pub fn get_value(&self) -> Option<u8> {
        self.value
    }

    pub fn is_solved(&self) -> bool {
        self.value.is_some()
    }

    pub fn exclude_values<T: Candidate>(&mut self, values: T) -> bool {
        if self.is_solved() {
            return false;
        }

        let items = values.to_candidates();
        let before = self.possible_variants.len();
        for val in items {
            self.restricted_variants.insert(val);
            self.possible_variants.remove(&val);
        }

        assert!(
            !self.possible_variants.is_empty(),
            "Slot {} has no candidates after exclude_values",
            self.id
        );

        if self.possible_variants.len() == 1 {
            let last_value = *self.possible_variants.iter().next().unwrap();
            return self.set_value(Some(last_value));
        }

        self.possible_variants.len() != before
    }

    pub fn exclude_value(&mut self, value: u8) -> bool {
        if self.is_solved() {
            return false;
        }

        let before = self.possible_variants.len();
        self.restricted_variants.insert(value);

        if self.possible_variants.remove(&value) && self.possible_variants.len() == 1 {
            let last_val = *self.possible_variants.iter().next().unwrap();
            return self.set_value(Some(last_val));
        }

        self.possible_variants.len() != before
    }

    pub fn variants(&self) -> HashSet<u8> {
        if let Some(val) = self.value {
            return HashSet::from([val]);
        }
        self.possible_variants.clone()
    }

    pub fn row(&self) -> Weak<RefCell<Line>> {
        self.row.clone()
    }

    pub fn column(&self) -> Weak<RefCell<Line>> {
        self.column.clone()
    }

    pub fn area(&self) -> Weak<RefCell<Area>> {
        self.area.clone()
    }

    pub fn get_peers(&self) -> Vec<Rc<RefCell<Slot>>> {
        let mut peer_ids = HashSet::new();
        let mut peers = Vec::new();

        let mut add_peers = |slots: &[Rc<RefCell<Slot>>]| {
            for slot in slots {
                let slot_id = slot.borrow().id();
                if slot_id != self.id && peer_ids.insert(slot_id) {
                    peers.push(Rc::clone(slot));
                }
            }
        };

        if let Some(row) = self.row.upgrade() {
            add_peers(row.borrow().slots());
        }

        if let Some(col) = self.column.upgrade() {
            add_peers(col.borrow().slots());
        }

        if let Some(area) = self.area.upgrade() {
            add_peers(area.borrow().slots());
        }

        peers
    }

    pub fn get_solved_peers_values(&self) -> HashSet<u8> {
        let mut values = HashSet::new();

        let mut collect_values = |slots: &[Rc<RefCell<Slot>>]| {
            for slot in slots {
                let slot_borrow = slot.borrow();
                if slot_borrow.id() != self.id
                    && let Some(val) = slot_borrow.get_value()
                {
                    values.insert(val);
                }
            }
        };

        if let Some(row) = self.row.upgrade() {
            collect_values(row.borrow().slots());
        }

        if let Some(col) = self.column.upgrade() {
            collect_values(col.borrow().slots());
        }

        if let Some(area) = self.area.upgrade() {
            collect_values(area.borrow().slots());
        }

        values
    }

    pub fn has_candidate(&self, value: u8) -> bool {
        self.possible_variants.contains(&value)
    }

    pub fn candidate_count(&self) -> usize {
        self.possible_variants.len()
    }
}

impl PartialEq<Self> for Slot {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl Identifiable for Slot {
    fn id(&self) -> usize {
        self.id
    }
}

impl Display for Slot {
    // Implement "{}" formatting for Slot.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "Slot {{id: {}, value: {}}}",
                self.id,
                self.get_value().unwrap()
            )
        } else {
            write!(f, "{}", self.get_value().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::one(1)]
    #[case::two(2)]
    #[case::three(81)]
    fn test_new_with_value(#[case] input: u8) {
        let slot = Slot::new(1, Some(input));

        assert_eq!(slot.id, 1);
        assert_eq!(slot.value, Some(input));
        assert!(slot.is_solved());
    }

    #[test]
    fn test_new_without_value() {
        let slot = Slot::new(1, None);

        assert_eq!(slot.id, 1);
        assert_eq!(slot.value, None);
        assert!(!slot.is_solved());
    }

    #[test]
    fn test_get_value() {
        let fake_value = 2;

        let slot = Slot::new(1, Some(fake_value));

        let val = slot.get_value();
        assert_eq!(val, Some(fake_value));
    }
}
