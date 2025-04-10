# protocol-spec
protocol-spec is a library to create arbitrary text and binary protocols. The protocol-spec  is currently implemented based on understanding of constructs required for parsing text based protocol e.g HTTP, FTP. 

Below is the code to build http protocol

```rust

let root_placeholder =
            Placeholder::new(Name("root".to_string()), None, PlaceHolderType::Composite);
let mut spec_builder = SpecBuilder(root_placeholder);

let request_line_placeholder = SpecBuilder::new_composite("request_line".to_string())
    .expect_one_of_string(
        vec![
            "GET".to_string(),
            "POST".to_string(),
            "DELETE".to_string(),
            "PUT".to_string(),
            "OPTIONS".to_string(),
        ],
        InlineKeyWithValue("request_method".to_string()),
    )
    .expect_space()
    .expect_string(InlineKeyWithValue("request_uri".to_string()))
    .expect_space()
    .expect_string(InlineKeyWithValue("protocol_version".to_string()))
    .expect_newline()
    .build();
```

The above code will allow the protocol-spec parser to read the first line of typical http protocol
```
GET /index.html HTTP/1.1

```
