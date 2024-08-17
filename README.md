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
            return Err(StandardError::from("ER-0037", StatusCode::BAD_REQUEST));
        }
    };
    Ok(Json(json!({"message": "success"})))
}
```

## Explanation

- **Error Handling**: In the example, StandardError is used to handle cases where a required query parameter is missing. If the group parameter is not provided, the function returns a StandardError with the appropriate error code and HTTP status.
- **Error Code**: The error code "ER-0037" is used to look up the relevant error message based on the locale.


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
```

> Keep this yaml file (`errors.yaml`) at the root of your directory, outside `src`.
 Or you can keep it wherever you please and set the `ERROR_YAML_FILE_PATH` environment variable.

> As for the locale configuration, by default, the crate picks up the default value from the `DEFAULT_LOCALE` env, which is set to `en_US` by default.
> - You can change this env to any value you like, provided the corresponding keys are present in yout errors yaml file.

> If you wish to dynamically change the locale programmatically at any given point, you can call the `standard_error::set_current_locale` function like so

```rust
use standard_error::set_current_locale;


def my_business_logic(){
    //...
    set_current_locale("hi_IN");
    //...
}

This sets a thread local refcell variable that'll persist throught the thread. Since it's a `RefCell` value, and not something like `Arc`, you don't have to worry about it leaking into your other threads/requests.



```


