//! SigmaOS-api-testing: API testing model.
//!
//! Real implementation: tests, assertions, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Test {
    pub id: String,
    pub name: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assertion {
    pub id: String,
    pub test_id: String,
    pub expected: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiTestingError {
    EmptyTestName,
    DuplicateTest(String),
}

impl fmt::Display for ApiTestingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTestName => write!(f, "test name must not be empty"),
            Self::DuplicateTest(id) => write!(f, "duplicate test {:?}", id),
        }
    }
}

impl std::error::Error for ApiTestingError {}

pub struct ApiTesting {
    pub name: String,
    tests: BTreeMap<String, Test>,
    assertions: BTreeMap<String, Assertion>,
}

impl ApiTesting {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), tests: BTreeMap::new(), assertions: BTreeMap::new() }
    }

    pub fn add_test(&mut self, t: Test) -> Result<(), ApiTestingError> {
        if t.name.trim().is_empty() { return Err(ApiTestingError::EmptyTestName); }
        let id = t.id.clone();
        if self.tests.contains_key(&id) {
            return Err(ApiTestingError::DuplicateTest(id.clone()));
        }
        self.tests.insert(id, t);
        Ok(())
    }

    pub fn test_count(&self) -> usize { self.tests.len() }

    pub fn get_test(&self, id: &str) -> Option<&Test> { self.tests.get(id) }

    pub fn add_assertion(&mut self, a: Assertion) {
        self.assertions.insert(a.id.clone(), a);
    }

    pub fn get_assertion(&self, id: &str) -> Option<&Assertion> { self.assertions.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = ApiTesting::new("ApiTesting");
        assert_eq!(a.name, "ApiTesting");
        assert_eq!(a.test_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = ApiTesting::new("ApiTesting");
        let test = Test { id: "t1".into(), name: "get-users".into(), endpoint: "/api/users".into() };
        a.add_test(test).unwrap();
        assert_eq!(a.test_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = ApiTesting::new("ApiTesting");
        let test = Test { id: "x".into(), name: "".into(), endpoint: "x".into() };
        assert!(a.add_test(test).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = ApiTesting::new("ApiTesting");
        let test = Test { id: "x".into(), name: "X".into(), endpoint: "x".into() };
        a.add_test(test).unwrap();
        let test2 = Test { id: "x".into(), name: "X".into(), endpoint: "x".into() };
        assert!(a.add_test(test2).is_err());
    }
}
