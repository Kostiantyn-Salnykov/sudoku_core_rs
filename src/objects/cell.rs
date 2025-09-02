use crate::objects::area::Area;
use crate::objects::line::Line;
use crate::objects::traits::ToVariants;
use crate::traits::Identifiable;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Weak;

#[derive(Debug)]
pub struct Cell {
    id: usize,
    value: Option<u8>,
    variants: Vec<u8>,
    row: Weak<RefCell<Line>>,
    column: Weak<RefCell<Line>>,
    area: Weak<RefCell<Area>>,
}

impl Cell {
    pub fn new(id: usize, value: Option<u8>, variants: Vec<u8>) -> Self {
        Cell {
            id,
            value,
            variants,
            row: Weak::new(),
            column: Weak::new(),
            area: Weak::new(),
        }
    }

    pub fn set_value(&mut self, value: Option<u8>) {
        // Skip setter if the value is already set.
        if self.value.is_some() {
            return;
        }

        self.value = value;
        self.variants = match value {
            Some(val) => vec![val],
            None => (1..=9).collect(),
        };
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

    pub fn remove_from_variants<T>(&mut self, items: T) -> bool
    where
        T: ToVariants,
    {
        let items_to_remove = items.to_variants();
        self.variants.retain(|item| !items_to_remove.contains(item));
        if self.variants.len() == 1 {
            self.value = Some(self.variants[0]);
            true
        } else {
            false
        }
    }

    pub fn variants(&self) -> Vec<u8> {
        self.variants.clone()
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
}

impl PartialEq<Self> for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl Identifiable for Cell {
    fn id(&self) -> usize {
        self.id
    }
}

impl Display for Cell {
    // Implement "{}" formatting for Cell.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "Cell {{id: {}, value: {}}}",
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
        let expected_variants = vec![input];

        let cell = Cell::new(1, Some(input), expected_variants.clone());

        assert_eq!(cell.id, 1);
        assert_eq!(cell.value, Some(input));
        assert_eq!(cell.variants, expected_variants);
        assert!(cell.is_solved());
    }

    #[test]
    fn test_new_without_value() {
        let fake_variants = vec![1, 2, 3];

        let cell = Cell::new(1, None, fake_variants.clone());

        assert_eq!(cell.id, 1);
        assert_eq!(cell.value, None);
        assert_eq!(cell.variants, fake_variants);
        assert!(!cell.is_solved());
    }

    #[test]
    fn test_get_value() {
        let fake_value = 2;

        let cell = Cell::new(1, Some(fake_value), vec![fake_value]);

        let val = cell.get_value();
        assert_eq!(val, Some(fake_value));
    }

    #[rstest]
    #[case::one(1)]
    #[case::nine(9)]
    fn test_set_value(#[case] input: u8) {
        let fake_value = 1;

        let mut cell = Cell::new(1, Some(fake_value), vec![input]);

        cell.set_value(Some(input));
        assert_eq!(cell.get_value(), Some(fake_value));
    }

    #[rstest]
    #[case::one(1)]
    #[case::nine(9)]
    fn test_set_value_none(#[case] input: u8) {
        let mut cell = Cell::new(1, Some(input), vec![input]);

        cell.set_value(None);

        assert_eq!(cell.value, Some(input));
    }
}
