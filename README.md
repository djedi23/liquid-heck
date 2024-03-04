# liquid-heck

Case conversion (CamelCase, snake_case, ...) filters for
[Liquid](https://crates.io/crates/liquid) using
[Heck](https://crates.io/crates/heck) library.


This library provides case conversion filters for the Liquid template
engine, utilizing the Heck library for case conversion.


| ... in `filter()` | ... in templates |
|-------------------|------------------|
| KebabCase         | kebabcase        |
| LowerCamelCase    | lowercamelcase   |
| ShoutyKebabCase   | shoutykebabcase  |
| ShoutySnakeCase   | shoutysnakecase  |
| SnakeCase         | snakecase        |
| TitleCase         | titlecase        |
| TrainCase         | traincase        |
| UpperCamelCase    | uppercamelcase   |

## Example

To use the case conversion filters in your Liquid templates, you first
need to register the filters.


```rust
use liquid_heck::{UpperCamelCase,SnakeCase,TrainCase};

let template = liquid::ParserBuilder::with_stdlib()
    .filter(UpperCamelCase)	// Register the filters
    .filter(SnakeCase)
    .filter(TrainCase)
    .build().unwrap()
    .parse("{{text | uppercamelcase}} {{text | snakecase}} {{text | traincase}}").unwrap();

let mut globals = liquid::object!({
    "text": "Some text to convert"
});

let output = template.render(&globals).unwrap();
assert_eq!(output, "SomeTextToConvert some_text_to_convert Some-Text-To-Convert".to_string());
```
