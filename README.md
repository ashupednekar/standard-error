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

> Note: Add features `diesel` or `git` to auto-magically handle errors raised by these diesel and git2 crates respectively

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
    detail_hi_IN: "एक पूर्णांक होना चाहिए: [err]"
  - code: ER-0006
    detail_en_US: "Should be an integer - fname: [fname] | lname: [lname]"
    detail_hi_IN: "एक पूर्णांक होना चाहिए - fname: [fname] | lname: [lname]"
  - code: ER-0007
    detail_en_US: "Should be an integer - fname: [fname] | lname: [lname] - [err]"
    detail_hi_IN: "एक पूर्णांक होना चाहिए - fname: [fname] | lname: [lname] - [err]"
  - code: ER-DB-NOTFOUND
    detail_en_US: "Record not found: [err]"
    detail_hi_IN: "रिकॉर्ड नहीं मिला: [err]"
  - code: ER-DB-DATABASE
    detail_en_US: "Database error: [details]"
    detail_hi_IN: "डेटाबेस त्रुटि: [details]"
  - code: ER-DB-QUERYBUILDER
    detail_en_US: "Query builder error: [err]"
    detail_hi_IN: "क्वेरी बिल्डर त्रुटि: [err]"
  - code: ER-DB-DESERIALIZATION
    detail_en_US: "Deserialization error: [err]"
    detail_hi_IN: "डिसेरियलाइजेशन त्रुटि: [err]"
  - code: ER-DB-SERIALIZATION
    detail_en_US: "Serialization error: [err]"
    detail_hi_IN: "सीरियलाइजेशन त्रुटि: [err]"
  - code: ER-DB-ROLLBACK
    detail_en_US: "Transaction was rolled back: [err]"
    detail_hi_IN: "लेनदेन को रोलबैक कर दिया गया: [err]"
  - code: ER-DB-UNKNOWN
    detail_en_US: "An unknown Diesel error occurred: [err]"
    detail_hi_IN: "एक अज्ञात डीजल त्रुटि हुई: [err]"
  - code: ER-IO-NOTFOUND
    detail_en_US: "File not found: [err]"
    detail_hi_IN: "फाइल नहीं मिली: [err]"
  - code: ER-IO-PERMISSION
    detail_en_US: "Permission denied: [err]"
    detail_hi_IN: "अनुमति अस्वीकृत: [err]"
  - code: ER-IO-CONNECTION
    detail_en_US: "Connection refused: [err]"
    detail_hi_IN: "कनेक्शन अस्वीकृत: [err]"
  - code: ER-IO-RESET
    detail_en_US: "Connection reset: [err]"
    detail_hi_IN: "कनेक्शन रीसेट: [err]"
  - code: ER-IO-ABORTED
    detail_en_US: "Connection aborted: [err]"
    detail_hi_IN: "कनेक्शन समाप्त: [err]"
  - code: ER-IO-NOTCONNECTED
    detail_en_US: "Not connected: [err]"
    detail_hi_IN: "कनेक्टेड नहीं: [err]"
  - code: ER-IO-ADDRINUSE
    detail_en_US: "Address in use: [err]"
    detail_hi_IN: "पता उपयोग में: [err]"
  - code: ER-IO-ADDRNOTAVAILABLE
    detail_en_US: "Address not available: [err]"
    detail_hi_IN: "पता उपलब्ध नहीं: [err]"
  - code: ER-IO-BROKENPIPE
    detail_en_US: "Broken pipe: [err]"
    detail_hi_IN: "टूटे हुए पाइप: [err]"
  - code: ER-IO-ALREADYEXISTS
    detail_en_US: "Already exists: [err]"
    detail_hi_IN: "पहले से मौजूद है: [err]"
  - code: ER-IO-WOULDBLOCK
    detail_en_US: "Operation would block: [err]"
    detail_hi_IN: "ऑपरेशन ब्लॉक होगा: [err]"
  - code: ER-IO-TIMEDOUT
    detail_en_US: "Operation timed out: [err]"
    detail_hi_IN: "ऑपरेशन समय समाप्त: [err]"
  - code: ER-IO-INTERRUPTED
    detail_en_US: "Operation interrupted: [err]"
    detail_hi_IN: "ऑपरेशन बाधित: [err]"
  - code: ER-IO-UNEXPECTEDEOF
    detail_en_US: "Unexpected end of file: [err]"
    detail_hi_IN: "अप्रत्याशित फ़ाइल अंत: [err]"
  - code: ER-IO-UNKNOWN
    detail_en_US: "An unknown I/O error occurred: [err]"
    detail_hi_IN: "एक अज्ञात I/O त्रुटि हुई: [err]"
  - code: ER-GIT-NOTFOUND
    detail_en_US: "Not found: [err]"
    detail_hi_IN: "नहीं मिला: [err]"
  - code: ER-GIT-INVALIDSPEC
    detail_en_US: "Invalid specification: [err]"
    detail_hi_IN: "अमान्य विशिष्टता: [err]"
  - code: ER-GIT-AUTHENTICATION
    detail_en_US: "Authentication error: [err]"
    detail_hi_IN: "प्रमाणीकरण त्रुटि: [err]"
  - code: ER-GIT-AUTH
    detail_en_US: "Authorization error: [err]"
    detail_hi_IN: "अनुमति त्रुटि: [err]"
  - code: ER-GIT-CONFIG
    detail_en_US: "Configuration error: [err]"
    detail_hi_IN: "कॉन्फ़िगरेशन त्रुटि: [err]"
  - code: ER-GIT-REFERENCE
    detail_en_US: "Reference error: [err]"
    detail_hi_IN: "संदर्भ त्रुटि: [err]"
  - code: ER-GIT-OBJECT
    detail_en_US: "Object error: [err]"
    detail_hi_IN: "ऑब्जेक्ट त्रुटि: [err]"
  - code: ER-GIT-INDEX
    detail_en_US: "Index error: [err]"
    detail_hi_IN: "इंडेक्स त्रुटि: [err]"
  - code: ER-GIT-WORKTREE
    detail_en_US: "Worktree error: [err]"
    detail_hi_IN: "वर्कट्री त्रुटि: [err]"
  - code: ER-GIT-MERGE
    detail_en_US: "Merge error: [err]"
    detail_hi_IN: "मर्ज त्रुटि: [err]"
  - code: ER-GIT-TREE
    detail_en_US: "Tree error: [err]"
    detail_hi_IN: "ट्री त्रुटि: [err]"
  - code: ER-GIT-INDEXNOTFOUND
    detail_en_US: "Index not found: [err]"
    detail_hi_IN: "इंडेक्स नहीं मिला: [err]"
  - code: ER-GIT-BRANCH
    detail_en_US: "Branch error: [err]"
    detail_hi_IN: "ब्रांच त्रुटि: [err]"
  - code: ER-GIT-TAG
    detail_en_US: "Tag error: [err]"
    detail_hi_IN: "टैग त्रुटि: [err]"
  - code: ER-GIT-COMMIT
    detail_en_US: "Commit error: [err]"
    detail_hi_IN: "कमीट त्रुटि: [err]"
  - code: ER-GIT-CHECKOUT
    detail_en_US: "Checkout error: [err]"
    detail_hi_IN: "चेकआउट त्रुटि: [err]"
  - code: ER-GIT-REPO
    detail_en_US: "Repository error: [err]"
    detail_hi_IN: "रिपोजिटरी त्रुटि: [err]"
  - code: ER-GIT-UNKNOWN
    detail_en_US: "An unknown Git error occurred: [err]"
    detail_hi_IN: "एक अज्ञात Git त्रुटि हुई: [err]"
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




