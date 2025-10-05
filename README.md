# protocol-spec
protocol-spec is a library to create arbitrary text and binary protocols. The protocol-spec  is currently implemented based on understanding of constructs required for parsing text based protocol e.g HTTP, FTP. 

Below is the code to build http protocol

```rust
let request_line_placeholder= ProtoSpecBuilderData::<BuildFromScratch>::new_with(Transient("request_line".to_string()), false);
let request_line_placeholder = 
request_line_placeholder.inline_value_follows(Name("request_method".to_owned()), false)
.expect_one_of_string(
    NoName,
    false,
    vec![
        "GET".to_string(),
        "POST".to_string(),
        "DELETE".to_string(),
        "PUT".to_string(),
        "OPTIONS".to_string(),
    ],
)
.delimited_by_space()

.inline_value_follows(Name("request_uri".to_owned()), false)
.expect_string(
    NoName,
    false,
    
)
.delimited_by_space()

.inline_value_follows(Name("protocol_version".to_owned()), false)
.expect_string(NoName,false)
.delimited_by_newline()
.build();

```

The above code will allow the protocol-spec parser to read the first line of typical http request
```
GET /index.html HTTP/1.1
```
