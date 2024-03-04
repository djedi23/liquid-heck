//! Case conversion (CamelCase, snake_case, ...) filters for
//! [Liquid](https://crates.io/crates/liquid) using
//! [Heck](https://crates.io/crates/heck) library.
//!
//!
//! This library provides case conversion filters for the Liquid template
//! engine, utilizing the Heck library for case conversion.
//!
//!
//! | ... in `filter()` | ... in templates |
//! |-------------------|------------------|
//! | KebabCase         | kebabcase        |
//! | LowerCamelCase    | lowercamelcase   |
//! | ShoutyKebabCase   | shoutykebabcase  |
//! | ShoutySnakeCase   | shoutysnakecase  |
//! | SnakeCase         | snakecase        |
//! | TitleCase         | titlecase        |
//! | TrainCase         | traincase        |
//! | UpperCamelCase    | uppercamelcase   |
//!
//! # Example
//!
//! To use the case conversion filters in your Liquid templates, you first
//! need to register the filters.
//!
//!
//! ```
//! use liquid_heck::{UpperCamelCase,SnakeCase,TrainCase};
//!
//! let template = liquid::ParserBuilder::with_stdlib()
//!     .filter(UpperCamelCase)    // Register the filters
//!     .filter(SnakeCase)
//!     .filter(TrainCase)
//!     .build().unwrap()
//!     .parse("{{text | uppercamelcase}} {{text | snakecase}} {{text | traincase}}").unwrap();
//!
//! let mut globals = liquid::object!({
//!     "text": "Some text to convert"
//! });
//!
//! let output = template.render(&globals).unwrap();
//! assert_eq!(output, "SomeTextToConvert some_text_to_convert Some-Text-To-Convert".to_string());
//! ```
//!
//! # Feature
//!
//! * **tracing** : instruments all the conversion methods using [tracing](https://crates.io/crates/tracing)
use heck::{
  ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
  ToTrainCase, ToUpperCamelCase,
};
use liquid_core::{Filter, Result, Runtime, Value, ValueView};
use liquid_derive::{Display_filter, FilterReflection, ParseFilter};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
  name = "uppercamelcase",
  description = "Convert the string to UpperCamelCase.",
  parsed(UpperCamelCaseFilter)
)]
pub struct UpperCamelCase;
#[derive(Debug, Default, Display_filter)]
#[name = "uppercamelcase"]
struct UpperCamelCaseFilter;
impl Filter for UpperCamelCaseFilter {
  #[cfg_attr(feature = "tracing", tracing::instrument(skip(_runtime)))]
  fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
    let s = input.to_kstr();
    let s = s.as_str();
    Ok(Value::scalar(s.to_upper_camel_case()))
  }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
  name = "lowercamelcase",
  description = "Convert the string to lowerCamelCase.",
  parsed(LowerCamelCaseFilter)
)]
pub struct LowerCamelCase;
#[derive(Debug, Default, Display_filter)]
#[name = "lowercamelcase"]
struct LowerCamelCaseFilter;
impl Filter for LowerCamelCaseFilter {
  #[cfg_attr(feature = "tracing", tracing::instrument(skip(_runtime)))]
  fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
    let s = input.to_kstr();
    let s = s.as_str();
    Ok(Value::scalar(s.to_lower_camel_case()))
  }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
  name = "snakecase",
  description = "Convert the string to snake-case.",
  parsed(SnakeCaseFilter)
)]
pub struct SnakeCase;
#[derive(Debug, Default, Display_filter)]
#[name = "snakecase"]
struct SnakeCaseFilter;
impl Filter for SnakeCaseFilter {
  #[cfg_attr(feature = "tracing", tracing::instrument(skip(_runtime)))]
  fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
    let s = input.to_kstr();
    let s = s.as_str();
    Ok(Value::scalar(s.to_snake_case()))
  }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
  name = "kebabcase",
  description = "Convert the string to kebab-case.",
  parsed(KebabCaseFilter)
)]
pub struct KebabCase;
#[derive(Debug, Default, Display_filter)]
#[name = "kebabcase"]
struct KebabCaseFilter;
impl Filter for KebabCaseFilter {
  #[cfg_attr(feature = "tracing", tracing::instrument(skip(_runtime)))]
  fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
    let s = input.to_kstr();
    let s = s.as_str();
    Ok(Value::scalar(s.to_kebab_case()))
  }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
  name = "shoutysnakecase",
  description = "Convert the string to SHOUTY_SNAKE_CASE.",
  parsed(ShoutySnakeCaseFilter)
)]
pub struct ShoutySnakeCase;
#[derive(Debug, Default, Display_filter)]
#[name = "shoutysnakecase"]
struct ShoutySnakeCaseFilter;
impl Filter for ShoutySnakeCaseFilter {
  #[cfg_attr(feature = "tracing", tracing::instrument(skip(_runtime)))]
  fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
    let s = input.to_kstr();
    let s = s.as_str();
    Ok(Value::scalar(s.to_shouty_snake_case()))
  }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
  name = "titlecase",
  description = "Convert the string to title case.",
  parsed(TitleCaseFilter)
)]
pub struct TitleCase;
#[derive(Debug, Default, Display_filter)]
#[name = "titlecase"]
struct TitleCaseFilter;
impl Filter for TitleCaseFilter {
  #[cfg_attr(feature = "tracing", tracing::instrument(skip(_runtime)))]
  fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
    let s = input.to_kstr();
    let s = s.as_str();
    Ok(Value::scalar(s.to_title_case()))
  }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
  name = "shoutykebabcase",
  description = "Convert the string to SHOUTY-KEBAB-CASE.",
  parsed(ShoutyKebabCaseFilter)
)]
pub struct ShoutyKebabCase;
#[derive(Debug, Default, Display_filter)]
#[name = "shoutykebabcase"]
struct ShoutyKebabCaseFilter;
impl Filter for ShoutyKebabCaseFilter {
  #[cfg_attr(feature = "tracing", tracing::instrument(skip(_runtime)))]
  fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
    let s = input.to_kstr();
    let s = s.as_str();
    Ok(Value::scalar(s.to_shouty_kebab_case()))
  }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
  name = "traincase",
  description = "Convert the string to Train-Case.",
  parsed(TrainCaseFilter)
)]
pub struct TrainCase;

#[derive(Debug, Default, Display_filter)]
#[name = "traincase"]
struct TrainCaseFilter;

impl Filter for TrainCaseFilter {
  #[cfg_attr(feature = "tracing", tracing::instrument(skip(_runtime)))]
  fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
    let s = input.to_kstr();
    let s = s.as_str();
    Ok(Value::scalar(s.to_train_case()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn upper_camel_case() {
    assert_eq!(
      liquid_core::call_filter!(UpperCamelCase, "abc").unwrap(),
      liquid_core::value!("Abc")
    );
    assert_eq!(
      liquid_core::call_filter!(UpperCamelCase, "hello world 21").unwrap(),
      liquid_core::value!("HelloWorld21")
    );
    assert_eq!(
      liquid_core::call_filter!(UpperCamelCase, "hello world 21").unwrap(),
      liquid_core::value!("HelloWorld21")
    );
    assert_eq!(
      liquid_core::call_filter!(UpperCamelCase, "hello_world_21").unwrap(),
      liquid_core::value!("HelloWorld21")
    );
    assert_eq!(
      liquid_core::call_filter!(UpperCamelCase, "HelloWorld21").unwrap(),
      liquid_core::value!("HelloWorld21")
    );
  }

  #[test]
  fn lower_camel_case() {
    assert_eq!(
      liquid_core::call_filter!(LowerCamelCase, "abc").unwrap(),
      liquid_core::value!("abc")
    );
    assert_eq!(
      liquid_core::call_filter!(LowerCamelCase, "hello world 21").unwrap(),
      liquid_core::value!("helloWorld21")
    );
    assert_eq!(
      liquid_core::call_filter!(LowerCamelCase, "hello_world_21").unwrap(),
      liquid_core::value!("helloWorld21")
    );
    assert_eq!(
      liquid_core::call_filter!(LowerCamelCase, "HelloWorld21").unwrap(),
      liquid_core::value!("helloWorld21")
    );
  }

  #[test]
  fn snake_case() {
    assert_eq!(
      liquid_core::call_filter!(SnakeCase, "abc").unwrap(),
      liquid_core::value!("abc")
    );
    assert_eq!(
      liquid_core::call_filter!(SnakeCase, "hello world 21").unwrap(),
      liquid_core::value!("hello_world_21")
    );
    assert_eq!(
      liquid_core::call_filter!(SnakeCase, "hello_world_21").unwrap(),
      liquid_core::value!("hello_world_21")
    );
    assert_eq!(
      liquid_core::call_filter!(SnakeCase, "HelloWorld21").unwrap(),
      liquid_core::value!("hello_world21")
    );
  }

  #[test]
  fn kebab_case() {
    assert_eq!(
      liquid_core::call_filter!(KebabCase, "abc").unwrap(),
      liquid_core::value!("abc")
    );
    assert_eq!(
      liquid_core::call_filter!(KebabCase, "hello world 21").unwrap(),
      liquid_core::value!("hello-world-21")
    );
    assert_eq!(
      liquid_core::call_filter!(KebabCase, "hello_world_21").unwrap(),
      liquid_core::value!("hello-world-21")
    );
    assert_eq!(
      liquid_core::call_filter!(KebabCase, "HelloWorld21").unwrap(),
      liquid_core::value!("hello-world21")
    );
  }

  #[test]
  fn shouty_snake_case() {
    assert_eq!(
      liquid_core::call_filter!(ShoutySnakeCase, "abc").unwrap(),
      liquid_core::value!("ABC")
    );
    assert_eq!(
      liquid_core::call_filter!(ShoutySnakeCase, "hello world 21").unwrap(),
      liquid_core::value!("HELLO_WORLD_21")
    );
    assert_eq!(
      liquid_core::call_filter!(ShoutySnakeCase, "hello_world_21").unwrap(),
      liquid_core::value!("HELLO_WORLD_21")
    );
    assert_eq!(
      liquid_core::call_filter!(ShoutySnakeCase, "HelloWorld21").unwrap(),
      liquid_core::value!("HELLO_WORLD21")
    );
  }

  #[test]
  fn title_case() {
    assert_eq!(
      liquid_core::call_filter!(TitleCase, "abc").unwrap(),
      liquid_core::value!("Abc")
    );
    assert_eq!(
      liquid_core::call_filter!(TitleCase, "hello world 21").unwrap(),
      liquid_core::value!("Hello World 21")
    );
    assert_eq!(
      liquid_core::call_filter!(TitleCase, "hello_world_21").unwrap(),
      liquid_core::value!("Hello World 21")
    );
    assert_eq!(
      liquid_core::call_filter!(TitleCase, "HelloWorld21").unwrap(),
      liquid_core::value!("Hello World21")
    );
  }

  #[test]
  fn shouty_kebab_case() {
    assert_eq!(
      liquid_core::call_filter!(ShoutyKebabCase, "abc").unwrap(),
      liquid_core::value!("ABC")
    );
    assert_eq!(
      liquid_core::call_filter!(ShoutyKebabCase, "hello world 21").unwrap(),
      liquid_core::value!("HELLO-WORLD-21")
    );
    assert_eq!(
      liquid_core::call_filter!(ShoutyKebabCase, "hello_world_21").unwrap(),
      liquid_core::value!("HELLO-WORLD-21")
    );
    assert_eq!(
      liquid_core::call_filter!(ShoutyKebabCase, "HelloWorld21").unwrap(),
      liquid_core::value!("HELLO-WORLD21")
    );
  }

  #[test]
  fn train_case() {
    assert_eq!(
      liquid_core::call_filter!(TrainCase, "abc").unwrap(),
      liquid_core::value!("Abc")
    );
    assert_eq!(
      liquid_core::call_filter!(TrainCase, "hello world 21").unwrap(),
      liquid_core::value!("Hello-World-21")
    );
    assert_eq!(
      liquid_core::call_filter!(TrainCase, "hello_world_21").unwrap(),
      liquid_core::value!("Hello-World-21")
    );
    assert_eq!(
      liquid_core::call_filter!(TrainCase, "HelloWorld21").unwrap(),
      liquid_core::value!("Hello-World21")
    );
  }
}
