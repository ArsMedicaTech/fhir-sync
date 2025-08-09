# Rust Unit Testing Guide

This guide will help you understand how to add unit tests to your Rust project, using examples from your FHIR sync project.

## Table of Contents

1. [Basic Test Structure](#basic-test-structure)
2. [Testing Functions](#testing-functions)
3. [Testing Structs and Enums](#testing-structs-and-enums)
4. [Async Testing](#async-testing)
5. [Integration Testing](#integration-testing)
6. [Test Organization](#test-organization)
7. [Running Tests](#running-tests)
8. [Best Practices](#best-practices)
9. [Examples from Your Project](#examples-from-your-project)

## Basic Test Structure

In Rust, tests are written using the `#[test]` attribute. Tests are typically placed in a `tests` module within the same file as the code being tested.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code here
        assert_eq!(2 + 2, 4);
    }
}
```

## Testing Functions

### Simple Function Tests

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
}
```

### Testing Functions with Results

```rust
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        let result = divide(10.0, 2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }
}
```

## Testing Structs and Enums

### Testing Struct Serialization/Deserialization

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Patient {
    pub id: String,
    pub name: Option<String>,
    pub age: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_patient_serialization() {
        let patient = Patient {
            id: "12345".to_string(),
            name: Some("John Doe".to_string()),
            age: Some(30),
        };

        // Test serialization
        let json = serde_json::to_string(&patient).unwrap();
        assert!(json.contains("12345"));
        assert!(json.contains("John Doe"));

        // Test deserialization
        let deserialized: Patient = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, patient.id);
        assert_eq!(deserialized.name, patient.name);
        assert_eq!(deserialized.age, patient.age);
    }

    #[test]
    fn test_patient_with_optional_fields() {
        let patient = Patient {
            id: "67890".to_string(),
            name: None,
            age: None,
        };

        let json = serde_json::to_string(&patient).unwrap();
        let deserialized: Patient = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.id, "67890");
        assert_eq!(deserialized.name, None);
        assert_eq!(deserialized.age, None);
    }
}
```

### Testing Enum Variants

```rust
#[derive(Debug, PartialEq)]
pub enum Event {
    PatientCreated(String),
    PatientUpdated(String),
    PatientDeleted(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_variants() {
        let created = Event::PatientCreated("12345".to_string());
        let updated = Event::PatientUpdated("12345".to_string());
        let deleted = Event::PatientDeleted("12345".to_string());

        assert_ne!(created, updated);
        assert_ne!(updated, deleted);
        assert_ne!(created, deleted);
    }
}
```

## Async Testing

For testing async functions, use the `#[tokio::test]` attribute:

```rust
use tokio;

pub async fn fetch_patient(id: &str) -> Result<String, String> {
    // Simulate async operation
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    
    if id.is_empty() {
        Err("Invalid ID".to_string())
    } else {
        Ok(format!("Patient data for {}", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_patient_success() {
        let result = fetch_patient("12345").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Patient data for 12345");
    }

    #[tokio::test]
    async fn test_fetch_patient_empty_id() {
        let result = fetch_patient("").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid ID");
    }
}
```

## Integration Testing

Integration tests go in the `tests/` directory and test your crate as an external user would.

```rust
// tests/integration_test.rs
use your_crate_name::{Patient, validate_patient};

#[test]
fn test_integration_patient_validation() {
    let patient = Patient {
        id: "12345".to_string(),
        name: Some("John Doe".to_string()),
        age: Some(30),
    };

    let result = validate_patient(&patient);
    assert!(result.is_ok());
}
```

## Test Organization

### Helper Functions

Create helper functions to reduce code duplication:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test data
    fn create_test_patient() -> Patient {
        Patient {
            id: "12345".to_string(),
            name: Some("John Doe".to_string()),
            age: Some(30),
        }
    }

    #[test]
    fn test_patient_creation() {
        let patient = create_test_patient();
        assert_eq!(patient.id, "12345");
    }

    #[test]
    fn test_patient_modification() {
        let mut patient = create_test_patient();
        patient.name = Some("Jane Doe".to_string());
        assert_eq!(patient.name, Some("Jane Doe".to_string()));
    }
}
```

### Parameterized Tests

Test multiple scenarios with a single test:

```rust
#[test]
fn test_email_validation() {
    let test_cases = vec![
        ("valid@email.com", true),
        ("invalid-email", false),
        ("another@test.org", true),
        ("", true), // Empty email is valid (optional field)
    ];

    for (email, should_be_valid) in test_cases {
        let result = validate_email(email);
        if should_be_valid {
            assert!(result.is_ok(), "Email '{}' should be valid", email);
        } else {
            assert!(result.is_err(), "Email '{}' should be invalid", email);
        }
    }
}
```

## Running Tests

### Basic Commands

```bash
# Run all tests
cargo test

# Run tests with output (even for passing tests)
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests matching a pattern
cargo test test_patient

# Run only unit tests (exclude integration tests)
cargo test --lib

# Run only integration tests
cargo test --test integration_test
```

### Test Output

```
running 15 tests
test tests::test_add ... ok
test tests::test_divide_by_zero ... ok
test tests::test_divide_success ... ok
test tests::test_patient_creation ... ok
test tests::test_patient_modification ... ok
test tests::test_patient_serialization ... ok
test tests::test_patient_with_optional_fields ... ok
test tests::test_email_validation ... ok
test tests::test_date_validation ... ok
test tests::test_validate_patient_empty_demographic ... ok
test tests::test_validate_patient_invalid_date ... ok
test tests::test_validate_patient_invalid_email ... ok
test tests::test_validate_patient_minimal_valid ... ok
test tests::test_validate_patient_success ... ok
test tests::test_handle_patient_webhook_invalid_patient ... ok
test tests::test_handle_patient_webhook_success ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Best Practices

### 1. Test Naming

Use descriptive test names that explain what is being tested:

```rust
#[test]
fn test_validate_patient_with_empty_demographic_number_returns_error() {
    // Test implementation
}

#[test]
fn test_serialize_patient_with_all_fields_populated() {
    // Test implementation
}
```

### 2. Arrange-Act-Assert Pattern

Structure your tests clearly:

```rust
#[test]
fn test_divide_by_zero_returns_error() {
    // Arrange
    let a = 10.0;
    let b = 0.0;

    // Act
    let result = divide(a, b);

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Division by zero");
}
```

### 3. Test Edge Cases

Always test edge cases and error conditions:

```rust
#[test]
fn test_validate_patient_edge_cases() {
    // Test empty string
    let patient = Patient { id: "".to_string(), name: None, age: None };
    assert!(validate_patient(&patient).is_err());

    // Test very long string
    let patient = Patient { 
        id: "a".repeat(1000), 
        name: None, 
        age: None 
    };
    assert!(validate_patient(&patient).is_ok());

    // Test special characters
    let patient = Patient { 
        id: "12345".to_string(), 
        name: Some("José María".to_string()), 
        age: None 
    };
    assert!(validate_patient(&patient).is_ok());
}
```

### 4. Use Meaningful Assertions

```rust
// Good
assert_eq!(result, expected_value, "Expected {} but got {}", expected_value, result);

// Better - use assert! with custom messages
assert!(result.is_ok(), "Operation should succeed but failed with: {:?}", result);

// Best - use specific assertion methods
assert_matches!(result, Ok(value) if value > 0);
```

### 5. Test Documentation

Document complex tests:

```rust
#[test]
fn test_patient_serialization_roundtrip() {
    // This test verifies that a patient can be serialized to JSON
    // and then deserialized back to the same struct without data loss.
    
    let original = create_test_patient();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Patient = serde_json::from_str(&json).unwrap();
    
    assert_eq!(original, deserialized);
}
```

## Examples from Your Project

### Testing DomainPatient

```rust
// In src/domain/patient.rs
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_patient_deserialization() {
        let json = r#"{
            "demographic_no": "12345",
            "first_name": "John",
            "last_name": "Doe",
            "date_of_birth": "1990-01-01",
            "location": ["Toronto", "ON", "Canada", "M5V1A1"],
            "sex": "male",
            "phone": "+1-555-123-4567",
            "email": "john.doe@example.com"
        }"#;

        let patient: DomainPatient = serde_json::from_str(json).unwrap();
        
        assert_eq!(patient.demographic_no, "12345");
        assert_eq!(patient.first_name, Some("John".to_string()));
        assert_eq!(patient.last_name, Some("Doe".to_string()));
    }
}
```

### Testing Webhook Handler

```rust
// In src/webhook.rs
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use axum::body::Body;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_handle_upsert_success() {
        // Create a test channel
        let (tx, _rx) = tokio::sync::mpsc::channel::<Event>(1);
        
        // Create test patient
        let test_patient = create_test_patient();
        
        // Create the request
        let app = Router::new()
            .route("/patient", post(handle_upsert))
            .layer(Extension(tx));
        
        let request_body = serde_json::to_string(&test_patient).unwrap();
        let request = Request::builder()
            .method("POST")
            .uri("/patient")
            .header("content-type", "application/json")
            .body(Body::from(request_body))
            .unwrap();
        
        // Send the request
        let response = app.oneshot(request).await.unwrap();
        
        // Assert the response
        assert_eq!(response.status(), StatusCode::OK);
    }
}
```

## Next Steps

1. **Start Small**: Begin by adding tests to your existing functions
2. **Focus on Critical Paths**: Test the most important business logic first
3. **Add Tests as You Go**: Write tests for new features as you develop them
4. **Use Test Coverage**: Consider using tools like `tarpaulin` to measure test coverage
5. **Continuous Integration**: Set up CI/CD to run tests automatically

## Additional Resources

- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust by Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html) 