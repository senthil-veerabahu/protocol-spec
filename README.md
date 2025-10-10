## protocol-spec

This crate helps developers create protocol parsers by using a declarative, DSL-style approach.
For e.g, developer can create custom protocol for imaginary example of sending \`hello world\`` to server upon connection
using the below code

```rust
use protocol_spec::common::*;
let mut spec_builder = ProtoSpecBuilderData::<BuildFromScratch>::new();
let spec = spec_builder
.inline_value_follows(SpecName::NoName, true)
.expect_string(SpecName::Name("greeting".to_string()), false).delimited_by_space()
.inline_value_follows(SpecName::NoName, true)
.expect_string(SpecName::Name("who".to_string()), false).delimited_by_space().build();
```

Text protocol can be thought of as list of data holders. Data here refers to `hello` and `world` separated by space.
`hello` represents the greeting type and `world` represents the receiver of the greeting.
Data can be thought of as key and value. The value represents the data and key identifies it with a name.
There are two ways to represent data in the parser 1)InlineValue 2)KeyValue.

#### InlineKeyValue

Inline Value specifies that the key is the SpecName and value is available in the protocol payload
In the above example, Key is `greeting`(from spec name) and value is `hello`

```rust
use protocol_spec::common::*;
let mut spec_builder = ProtoSpecBuilderData::<BuildFromScratch>::new();
let spec = spec_builder
.inline_value_follows(SpecName::NoName, true)
.expect_string(SpecName::Name("greeting".to_string()), false).delimited_by_space();
```

`delimited_by_space` specifies that the string `hello` ends with space.
It is also possible to specify data in other data types e.g u32.
In that case, the spec becomes as below. The boolean in `inline_value_follows` and `expect_u32` specifies whether the data is optional.

```rust
use protocol_spec::common::*;
let mut spec_builder = ProtoSpecBuilderData::<BuildFromScratch>::new();
let spec = spec_builder
.inline_value_follows(SpecName::NoName, true)
.expect_u32(SpecName::Name("somedata".to_string()), false);
```

The protocol can be thought of tree of individual data items and each individual data items can be represented using the spec builder.
For e.g in http request,

```rust
PUT /vote HTTP/1.1
Content-Type: application/json
Content-Length: 21
{option:1, id:"a1234"}
```

Http request can be thought of as request line followed by one or more key-value pairs followed by new line and payload data.
Each data here is represented as Spec. Spec contains metadata including a name(SpecName) and flag representing optionality of the spec
Each Spec can be serialized and deserialized.

In http request example, PUT can be represented as InlineKeyWithValue spec which contains DelimitedString,
Each header item can be represented as KeyValueSpec and Payload can be represented as InlineKeyWithValue spec containing bytes

Each header can be represented as below

```rust
use protocol_spec::common::*;
use protocol_spec::common::SpecName::*;
let mut header_placeholder_builder = new_mandatory_spec_builder(Transient("header".to_string()));    
let header_place_holder = header_placeholder_builder
.key_follows(Name("header_name".to_string()), true)
.expect_string( NoName, false)
.delimited_by(": ".to_string())
.value_follows(Name("header_value".to_owned()), false)
.expect_string(NoName, false)
.delimited_by_newline()
.build();
```

### KeyValueSpec

To specify both key and value from the protocol itself, use key_follows and value_follows function as in the above example.
Key is expected to be a string and value can be number, string or bytes

### RepeatMany spec

http headers can be repeated many times and it ends with a extra newline character. This can be represented as below using repeat_many function

```rust
use protocol_spec::common::*;
use protocol_spec::common::SpecName::*;
let mut spec_builder = ProtoSpecBuilderData::<BuildFromScratch>::new();        
let mut header_placeholder_builder = new_mandatory_spec_builder(Transient("header".to_string()));    
let header_place_holder = header_placeholder_builder
.key_follows(Name("header_name".to_string()), true)
.expect_string( NoName, false)
.delimited_by(": ".to_string())
.value_follows(Name("header_value".to_owned()), false)
.expect_string(NoName, false)
.delimited_by_newline()
.build();
let spec_builder = spec_builder.repeat_many(Name("headers".to_owned()), true, 
Separator::Delimiter("\r\n".to_owned()),header_place_holder);
```

Entire http request can be represented as spec

```rust
use protocol_spec::common::*;
use protocol_spec::http::BodySpec;
use protocol_spec::common::SpecName::*;
pub fn build_http_request_protocol() -> ListSpec {
   
   let space = " ";
   let newline = "\r\n";
   let mut spec_builder = ProtoSpecBuilderData::<BuildFromScratch>::new();        
   let request_line_placeholder= ProtoSpecBuilderData::<BuildFromScratch>::new_with(Transient("request_line".to_string()), false);
   //let request_line_placeholder = ;

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

   let mut header_placeholder_builder = new_mandatory_spec_builder(Transient("header".to_string()));
   //let mut header_placeholder_builder = header_placeholder_builder.delimited_by_newline();

   let header_place_holder = header_placeholder_builder
       .key_follows(Name("header_name".to_string()), true)
       .expect_string( NoName, false)
       .delimited_by(": ".to_string())
       
       .value_follows(Name("header_value".to_owned()), false)
       .expect_string(NoName, false)
       .delimited_by_newline()
       .build();

   let spec_builder = spec_builder.expect_composite(request_line_placeholder)
   .repeat_many(Name("headers".to_owned()), true, Separator::Delimiter("\r\n".to_owned()),header_place_holder)
   
   .use_spec(Box::new(BodySpec::new(Name("request_body".to_owned()), true)));

   spec_builder.build()
}
```
