# standard-error

`standard-error` is a Rust crate designed to simplify the handling of error messages for various locales. It reads error messages from YAML and other sources, providing a structured and efficient way to manage errors in your applications.

## Features

- **Locale-Specific Error Messages**: Automatically fetches error messages for the specified locale.
- **Flexible Data Sources**: Reads error messages from YAML files and other formats.
- **Easy Integration**: Designed to work seamlessly with Rust web frameworks like `Axum`.

## Usage


Below is an example of how to use `standard-error` in a web application using the `Axum` framework:

```rust
use axum::{
    extract::Query,
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use standard_error::StandardError;

pub async fn my_handler(Query(params): Query<HashMap<String, String>>) -> Result<Json<Value>, StandardError> {
    let group = match params.get("name") {
        Some(g) => g,
        None => {
            return Err(StandardError::new("ER-0037").code(StatusCode::BAD_REQUEST));
        }
    };
    Ok(Json(json!({"message": "success"})))
}
```

## Explanation

- **Error Handling**: In the example, StandardError is used to handle cases where a required query parameter is missing. If the group parameter is not provided, the function returns a StandardError with the appropriate error code and HTTP status.
- **Error Code**: The error code "ER-0037" is used to look up the relevant error message based on the locale.

## Scenarios

### Basic Error Handling with Default Status Code

If a parsing error occurs (e.g., trying to convert a string to an integer), a StandardError can be returned with a default status code (`500 INTERNAL_SERVER_ERROR`):

```rust
async fn parse_int(a: &str) -> Result<i32, StandardError> {
    a.parse().map_err(|_| StandardError::new("ER-0004"))
}

// Example usage
let res = parse_int("abc").await;
// This will return an error with code "ER-0004" and status code 500.
```

### Setting a Custom Status Code

You can customize the status code to something other than the default. For example, returning `400 BAD_REQUEST`:

```rust
async fn parse_int_custom(a: &str) -> Result<i32, StandardError> {
    a.parse().map_err(|_| StandardError::new("ER-0004").code(StatusCode::BAD_REQUEST))
}

// Example usage
let res = parse_int_custom("abc").await;
// This will return an error with code "ER-0004" and status code 400.
```

### Interpolating Error Details

`StandardError` supports error message interpolation. For example, you can include the specific error details when returning the error:

```rust
async fn parse_with_error_interpolation(a: &str) -> Result<i32, StandardError> {
    a.parse().map_err(|e| StandardError::new("ER-0005").interpolate_err(e.to_string()))
}

// Example usage
let res = parse_with_error_interpolation("abc").await;
// This will return an error with code "ER-0005" and message: "Should be an integer: invalid digit found in string".
```

### Interpolating Values

You can interpolate additional values into the error message, such as user-specific data:

```rust
async fn parse_with_value_interpolation(a: &str) -> Result<i32, StandardError> {
    a.parse().map_err(|_| {
        let mut values = HashMap::new();
        values.insert("fname".to_string(), "ashu".to_string());
        values.insert("lname".to_string(), "pednekar".to_string());
        StandardError::new("ER-0006").interpolate_values(values)
    })
}

// Example usage
let res = parse_with_value_interpolation("abc").await;
// This will return an error with code "ER-0006" and message: "Should be an integer - fname: ashu | lname: pednekar".
```

### Chaining Status Code and Interpolations

You can chain multiple methods on `StandardError`, like setting a custom status code and interpolating both error details and values:

```rust

async fn parse_with_chained_errors(a: &str) -> Result<i32, StandardError> {
    a.parse().map_err(|e| {
        let mut values = HashMap::new();
        values.insert("fname".to_string(), "ashu".to_string());
        values.insert("lname".to_string(), "pednekar".to_string());
        StandardError::new("ER-0007")
            .code(StatusCode::IM_A_TEAPOT)
            .interpolate_values(values)
            .interpolate_err(e.to_string())
    })
}

// Example usage
let res = parse_with_chained_errors("abc").await;
// This will return an error with code "ER-0007", status code 418, and message:
// "Should be an integer - fname: ashu | lname: pednekar - invalid digit found in string".
```

## Installation

Add standard-error to your Cargo.toml:

```toml
[dependencies]
standard-error = "0.1"
```
or with cargo

```bash
cargo add standard-error
```

## Configuration

To configure standard-error, you can provide YAML files containing error messages for different locales. The crate will automatically load the correct message based on the locale specified in your application.

Example YAML structure:

```yaml
errors:
  - code: ER-0001
    detail_en_US: "Missing required 'group' parameter."
    detail_hi_IN: "'group' पैरामीटर आवश्यक है।"
  - code: ER-0002
    detail_en_US: "Permission Denied"
    detail_hi_IN: "अनुमति अस्वीकृत"
  - code: ER-0003
    detail_en_US: "Not Found"
    detail_hi_IN: "नहीं मिला"
  - code: ER-0004
    detail_en_US: "Should be an integer"
    detail_hi_IN: "एक पूर्णांक होना चाहिए"
  - code: ER-0005
    detail_en_US: "Should be an integer: [err]"
    detail_hi_IN: "एक पूर्णांक होना चाहिए"
  - code: ER-0006
    detail_en_US: "Should be an integer - fname: [fname] | lname: [lname]"
    detail_hi_IN: "एक पूर्णांक होना चाहिए"
  - code: ER-0007
    detail_en_US: "Should be an integer - fname: [fname] | lname: [lname] - [err]"
    detail_hi_IN: "एक पूर्णांक होना चाहिए"
```

> Keep this yaml file (`errors.yaml`) at the root of your directory, outside `src`.
 Or you can keep it wherever you please and set the `ERROR_YAML_FILE_PATH` environment variable.

> As for the locale configuration, by default, the crate picks up the default value from the `DEFAULT_LOCALE` env, which is set to `en_US` by default.
> - You can change this env to any value you like, provided the corresponding keys are present in yout errors yaml file.

> If you wish to dynamically change the locale programmatically at any given point, you can call the `standard_error::set_current_locale` function like so

```rust
use standard_error::set_current_locale;


fn my_business_logic(){
    //...
    set_current_locale("hi_IN");
    //...
}
```

> This sets a thread local refcell variable that'll persist throught the thread. Since it's a `RefCell` value, and not something like `Arc`, you don't have to worry about it leaking into your other threads/requests.




