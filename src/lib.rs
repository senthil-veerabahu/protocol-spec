//! # protocol-spec 
//! This crate helps developers create protocol parsers by using a declarative, DSL-style approach.
//! For e.g, developer can create custom protocol for imaginary example of sending `hello world`` to server upon connection 
//! using the below code
//! ```ignore
//! let mut spec_builder = ProtoSpecBuilderData::<BuildFromScratch>::new();
//! let spec = spec_builder
//! .inline_value_follows(SpecName::NoName, true)
//! .expect_string(SpecName::Name("greeting".to_string()), false).delimited_by_space()
//! .inline_value_follows(SpecName::NoName, true)
//! .expect_string(SpecName::Name("who".to_string()), false).delimited_by_space().build();
//! ```
//! 
//! Text protocol can be thought of as list of data holders. Data here refers to `hello` and `world` separated by space.
//! `hello` represents the greeting type and `world` represents the receiver of the greeting.
//! Data can be thought of as key and value. The value represents the data and key identifies it with a name.
//! There are two ways to represent data in the parser 1)InlineValue 2)KeyValue.
//! ### InlineKeyValue 
//! Inline Value specifies that the key is the SpecName and value is available in the protocol payload
//! In the above example, Key is `greeting`(from spec name) and value is `hello`
//! 
//! ```ignore
//! .inline_value_follows(SpecName::NoName, true)
//! .expect_string(SpecName::Name("greeting".to_string()), false).delimited_by_space()
//! ```
//! delimited_by_space specifies that the string `hello` ends with space. 
//! It is also possible to specify data in other data types e.g u32.
//! In that case, the spec becomes as below. The boolean in inline_value_follows and expect_u32 specifies whether the data is optional.
//! 
//! .inline_value_follows(SpecName::NoName, true)
//! .expect_u32(SpecName::Name("somedata"), false)
//! 
//! The protocol can be thought of tree of individual data items and each individual data items can be represented using the spec builder.
//! For e.g in http request,
//! ```ignore
//! PUT /vote HTTP/1.1
//! Content-Type: application/json
//! Content-Length: 21
//! 
//! {option:1, id:"a1234"}
//! ```
//! Http request can be thought of request line followed by one or more Key value pairs followed by new line and payload data.
//! Each data here is represented as Spec. Spec contains metadata including a name(SpecName) and flag representing optionality of the spec
//! Each Spec can be serialized and deserialized.
//! 
//! In http request example, PUT can be represented as InlineKeyWithValue spec which contains DelimitedString,
//! Each header item can be represented as KeyValueSpec and Payload can be represented as InlineKeyWithValue spec containing bytes
//! 
//! Each header can be represented as below
//! 
//! ```ignore
//! let mut header_placeholder_builder = new_mandatory_spec_builder(Transient("header".to_string()));    
//! let header_place_holder = header_placeholder_builder
//! .key_follows(Name("header_name".to_string()), true)
//! .expect_string( NoName, false)
//! .delimited_by(": ".to_string())
//! .value_follows(Name("header_value".to_owned()), false)
//! .expect_string(NoName, false)
//! .delimited_by_newline()
//! .build();
//! ```
//! 
//! ## KeyValueSpec
//! To specify both key and value from the protocol itself, use key_follows and value_follows function as in the above example.
//! Key is expected to be a string and value can be number, string or bytes
//! 
//! ## RepeatMany spec
//! 
//! http headers can be repeated many times and it ends with a extra newline character. This can be represented as below using repeat_many function
//! 
//! ```ignore
//! let spec_builder = spec_builder.repeat_many(Name("headers".to_owned()), true, 
//! Separator::Delimiter("\r\n".to_owned()),header_place_holder)
//! ```
//! 
//! Entire http request can be represented as spec
//! 
//! 
//! ```ignore
//!  
//! pub fn build_http_request_protocol() -> ListSpec {
//!    
//!    let space = " ";
//!    let newline = "\r\n";
//!    let mut spec_builder = ProtoSpecBuilderData::<BuildFromScratch>::new();        
//!    let request_line_placeholder= ProtoSpecBuilderData::<BuildFromScratch>::new_with(Transient("request_line".to_string()), false);
//!    //let request_line_placeholder = ;
//!
//!        let request_line_placeholder = 
//!        request_line_placeholder.inline_value_follows(Name("request_method".to_owned()), false)
//!        .expect_one_of_string(
//!            NoName,
//!            false,
//!            vec![
//!                "GET".to_string(),
//!                "POST".to_string(),
//!                "DELETE".to_string(),
//!                "PUT".to_string(),
//!                "OPTIONS".to_string(),
//!            ],
//!        )
//!        .delimited_by_space()
//!
//!        .inline_value_follows(Name("request_uri".to_owned()), false)
//!        .expect_string(
//!            NoName,
//!            false,
//!            
//!        )
//!        .delimited_by_space()
//!
//!        .inline_value_follows(Name("protocol_version".to_owned()), false)
//!        .expect_string(NoName,false)
//!        .delimited_by_newline()
//!        .build();
//!
//!    let mut header_placeholder_builder = new_mandatory_spec_builder(Transient("header".to_string()));
//!    //let mut header_placeholder_builder = header_placeholder_builder.delimited_by_newline();
//!
//!    let header_place_holder = header_placeholder_builder
//!        .key_follows(Name("header_name".to_string()), true)
//!        .expect_string( NoName, false)
//!        .delimited_by(": ".to_string())
//!        
//!        .value_follows(Name("header_value".to_owned()), false)
//!        .expect_string(NoName, false)
//!        .delimited_by_newline()
//!        .build();
//!
//!    let spec_builder = spec_builder.expect_composite(request_line_placeholder)
//!    .repeat_many(Name("headers".to_owned()), true, Separator::Delimiter("\r\n".to_owned()),header_place_holder)
//!    
//!    .use_spec(Box::new(BodySpec::new(Name("request_body".to_owned()), true)));
//!
//!    spec_builder.build()
//!}
//! 
//! 
 
/// common module exposes all the public items of the spec required to build custom protocol
pub mod common{

    pub use crate::mapping_extractor::{SpecTraverse, traverse_spec, ToSpecType, DefaultMapper};

    pub use crate::core::{SpecMapper, Spec, SpecMetaData, MapperContext, ProtocolSpec, AllBytesSpec, 
        DelimitedSpec, DelimitedStringSpec, OneOfSpec, NumberI16Spec, NumberI64Spec,
        NumberU16Spec, NumberU32Spec, NumberU64Spec, ListSpec, SimpleValueSpec,RepeatManySpec, NBytesSpec, 
        SpecRead, SpecWrite, Value, InfoProvider,
         Mapper, RequestInfo, ResponseInfo, ParserError, 
         RequestHandler, ResponseHandler, RequestFactory, ResponseFactory, RequestErrorHandler, ResponseErrorHandler, RequestSerializer, ResponseSerializer, DefaultSerializer,
        ProtocolConfig, ProtoSpecBuilderData, BuildFromScratch,
        InlineValueBuilder, KeySpecBuilder, RepeatBuilder, DelimitedStringSpecBuilder, 
        NumberSpecBuilder, DelimiterBuilder, ProtoSpecBuilder, ValueBuilder, CompositeBuilder, CustomSpecBuilder, 
        new_mandatory_spec_builder, Separator,
        SpecName, ValueType,  ValueExtractor, SpecSerialize, SpecDeserialize };
}

/// mapping_extractor specifies how to traverse tree of spec to build metadata required for parsing and querying
mod mapping_extractor{
    use std::collections::HashMap;
    use tracing::debug;
    use crate::core::{extract_name_and_spec_path, InlineKeyWithValue, Key, KeyValueSpec, ListSpec, MappableSpec, Mapper, MapperContext, ParserError, ProtocolSpec, RepeatManySpec, RepeaterContext, SimpleValueSpec, Spec, SpecMapper, SpecType, Value, ValueSpec};

    pub trait SpecTraverse{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError>;
    }

    #[derive(Clone, Default, Debug)]
    pub struct DefaultMapper{
        protocol_to_spec_field_map: HashMap<String, String>,
        protocol_to_spec_template_map: HashMap<String, String>,
        spec_data_map: HashMap<String, Value>,
        mapper_context: MapperContext,
        repeater_context_map: HashMap<String, RepeaterContext>,
        
    }

   impl Default for MapperContext{
    fn default() -> Self {
        MapperContext::new()
    }
   }

    impl DefaultMapper{
        pub fn new() ->Self{
            Self{
                protocol_to_spec_field_map: HashMap::new(),
                protocol_to_spec_template_map: HashMap::new(),
                spec_data_map: HashMap::new(),
                mapper_context: MapperContext::new(),
                repeater_context_map: HashMap::new(),
            }
        }
    }

   impl Mapper for DefaultMapper{

    fn get_mapping_data_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.protocol_to_spec_field_map
    }
   
    fn get_mapping_data_template_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.protocol_to_spec_template_map
    }

    fn get_mapping_data_template(&self) -> &HashMap<String, String> {
        &self.protocol_to_spec_template_map
    }
   
    fn get_mapping_data(&self) -> &HashMap<String, String> {
        &self.protocol_to_spec_field_map
    }
   
    fn get_spec_data_mut(&mut self) -> &mut HashMap<String, crate::core::Value> {
        &mut self.spec_data_map
    }

    fn get_spec_data(&self) -> &HashMap<String, crate::core::Value> {
        &self.spec_data_map
    }

    fn get_repeater_context_map_mut(&mut self) -> &mut HashMap<String, RepeaterContext>{
        &mut self.repeater_context_map
    }
   
    
   
    fn get_mapper_context_mut(&mut self) -> &mut crate::core::MapperContext {
        &mut self.mapper_context
    }
    
    fn get_mapper_context(&self) -> &MapperContext {
        &self.mapper_context
    }    
   }

   pub trait ToSpecType: Spec{
        fn to_spec_type(&self) ->SpecType {
            let spec_name = self.get_meta_data().get_name().clone();
            SpecType::Simple(spec_name)
        }
    }

    impl ToSpecType for dyn Spec {
    }

    /* impl ToSpecType for dyn StringSpec {
    } */

    impl ToSpecType for ListSpec {
    }

    impl ToSpecType for Key {
    }

    impl ToSpecType for ValueSpec {
    }

    impl ToSpecType for KeyValueSpec {
    }

    impl ToSpecType for InlineKeyWithValue {
    }

   impl ToSpecType for RepeatManySpec{
        fn to_spec_type(&self) ->SpecType{
            let spec_name = self.get_meta_data().get_name();
            SpecType::RepeatMany(spec_name.clone(), self.repeat_count.clone(), 0)
        }
    }

    impl  SpecTraverse for Key{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError> {
            traverse_spec(self, mapper)
        }
    }

    impl <S> SpecTraverse for S where S:SimpleValueSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>)  -> Result<(), ParserError> {
            traverse_spec(self, mapper)
        }
    }

    impl  SpecTraverse for ValueSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError> {
            traverse_spec(self, mapper)
        }
    }

    impl  SpecTraverse for KeyValueSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError> {
            traverse_spec(self, mapper)
        }
    }

    impl SpecTraverse for RepeatManySpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError> {
            traverse_spec(self, mapper)
        }
    }

    impl SpecTraverse for InlineKeyWithValue{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError> {
            traverse_spec(self, mapper)
        }
    }

    //TODO change the return value to Result instead of unit
    pub fn traverse_spec<S>(spec: &S, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError> where S:MappableSpec + ?Sized{
        mapper.get_mapper_context_mut().start_spec_type(spec.to_spec_type());    
        spec.add_mapping_template(mapper)?;
        return mapper.get_mapper_context_mut().end_spec(spec);
        
    }

    impl SpecTraverse for ListSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError> {
            traverse_spec(self, mapper)
        }
    }

    impl SpecMapper for InlineKeyWithValue{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>)->Result<(), ParserError>  {
            self.0.traverse(mapper)
        }
    }
   
    impl SpecMapper for RepeatManySpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) ->Result<(), ParserError>  {
            self.constituents.traverse(mapper)
        }
    }

    
    impl <T> SpecMapper for T where T:SimpleValueSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>)->Result<(), ParserError>  {
            debug!("delimited string spec {}", self.get_meta_data().get_name().to_string());

            if let Some(key_name) = mapper.get_mapper_context().get_last_available_spec_name() {
                let path = mapper.get_mapper_context_mut().get_current_spec_path_template();
                mapper.add_mapping_template(key_name, path);
            }
            Ok(())
        }
    }

    impl SpecMapper for ValueSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError> {
            self.0.traverse(mapper)
        }
    }

    impl SpecMapper for KeyValueSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) ->Result<(), ParserError> {

            debug!("keyvalue name {}, key name {}, inner keyspec name {}", self.get_meta_data().get_name(), self.key.get_meta_data().get_name(), self.key.0.get_meta_data().get_name());
            let path_finder =  |mapper:  &Box<dyn Mapper>| {
                mapper.get_mapper_context().get_current_spec_path_template()
            };
            //mapper.get_mapper_context_mut().start_spec(&self.key);
            let ( key_name,  key_spec_path,) = extract_name_and_spec_path(path_finder, mapper, &self.key, &self.key.0)?;
            match (&key_name, &key_spec_path){                
                (Some(name), Some(path)) => {
                    mapper.add_mapping_template(name.clone(), path.clone());
                }
                (_,_) =>{}
            }

            let ( value_name, value_spec_path,) = extract_name_and_spec_path(path_finder, mapper,&self.value, &self.value.0)?;
            match ( &value_name,  &value_spec_path){                
                (Some(name), Some(path)) => {
                    mapper.add_mapping_template(name.clone(), path.clone());
                }
                (_,_) =>{}
            }

            match (&key_spec_path, &value_spec_path){        
            (Some(key_path), Some(value_path)) => {
                mapper.add_mapping_template(key_path.clone(), value_path.clone());
            }
            (_,_) =>{}
            }
            Ok(())
        }
    }

    impl SpecMapper for Key{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) ->Result<(), ParserError>  {
            self.0.traverse(mapper)
        }   
    }

    

    impl SpecMapper for ListSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) ->Result<(), ParserError> {
            return self.constituents
            .iter()
            .fold(Ok(()), |result: Result<(), ParserError>, element: &Box<dyn ProtocolSpec>|
             result.and_then(|_| {
                element.traverse(mapper)
            }));
            
        }
    }
}


/// Core module contains the basic framework for building protocol specification.
pub mod core {
    use crate::core::protocol_reader::ReadBytesSize;
    use crate::core::protocol_writer::PlaceHolderWrite;
    use crate::core::protocol_reader::PlaceHolderRead;
    use crate::mapping_extractor::{DefaultMapper, SpecTraverse, ToSpecType};    
    use async_trait::async_trait;
    use derive_builder::Builder;
    use protocol_reader::ProtocolBuffReader;
    use protocol_reader::{ MarkAndRead};

    use protocol_writer::ProtocolBuffWriter;
    use tracing::{debug, info, warn};
    
    
    
    
    use std::collections::HashMap;
    use std::marker::PhantomData;
    use std::{
        fmt::{Debug, Display, Formatter}, mem::{self}, str::Utf8Error
    };
    use tokio::{
        io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
        net::{TcpListener, TcpStream},
    };

    //Currently not used. But later when we support udp and binary protocols

    #[allow(dead_code)]
    pub trait ProtocolInfo {
        fn get_name() -> String;
        fn get_version() -> String;
        fn get_transport_type() -> Transport;
        fn get_format() -> ProtocolFormat;
    }

    /// Enum for all common error generated from the framework
    #[allow(unused)]
    #[derive(Debug)]
    pub enum ParserError {
        /// Particular token expected is not found
        TokenExpected {
            line_index: usize,
            char_index: usize,
            message: String,
        },

        /// Error to denote that key is missing in KeyValueSpec
        MissingKey(String),

        /// Data is missing when try to serialize spec
        MissingData(String),

        /// Value is missing when try to deserialize KeyValueSpec
        MissingValue(String),

        /// denotes error from serde crate
        SerdeError(String),

        /// denotes error when parsing utf8 string
        Utf8Error(Utf8Error),

        /// denotes end of stream error
        EndOfStream,

        /// No  constituent of a composite spec can be serialized/deserialized
        NoValidListConstituents(String),

        /// Invalid marker        
        InvalidMarker {
            line_index: usize,
            char_index: usize,
            message: String,
        },

        /// wraps std::io::error
        IOError {
            error: std::io::Error,
        },
    }

    impl ParserError{

        /// check if the error is EndOfStream
        fn is_eof(&self) -> bool{
            match self{
                ParserError::EndOfStream => true,
                _ => false,
            }
        }
    }


    /// Implements Display for each item of ParserError enum
    impl Display for ParserError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                ParserError::TokenExpected {
                                            line_index: line_pos,
                                            char_index: position,
                                            message,
                                        } => {
                                            write!(
                                                f,
                                                "Token expected at line {} char_pos {} {}",
                                                line_pos, position, message
                                            )
                                        }
                ParserError::IOError { error } => {
                                            write!(f, "IO Error {}", error)
                                        }
                ParserError::MissingKey(msg) => write!(f, "{}", msg),                
                ParserError::MissingData(key) => {
                                            write!(f, "Expected data for key {} but found none whle writing to writer", key)
                                        }
                ParserError::MissingValue(key) => write!(
                                            f,
                                            "Expected value for key {} but found none whle writing to writer",
                                            key
                                        ),
                ParserError::Utf8Error(_key) => write!(
                                            f,
                                            "Expected value is not a valid  utf-8 data",                    
                                        ),
                ParserError::SerdeError(_key) => write!(
                                            f,
                                            "Expected value is not a valid  utf-8 data",                    
                                        ),
                ParserError::EndOfStream => write!(
                                f,
                                "End of stream reached while parsing data. Expected more data to be present.",                    
                            ),

                ParserError::NoValidListConstituents(name) => write!(
                                f,
                                "No consituent of the list spec {} has valid value", name                    
                            ),                            
                ParserError::InvalidMarker { line_index, char_index, message } => write!(
                    f,
                    "Invalid Marker provided during mark/reset operation at line {} char_pos {}: {}", line_index, char_index, message                       
                ),                
            }
        }
    }

    #[allow(unused)]

    ///Extractor to retrieve underlying data from Value
    pub trait ValueExtractor<'a> {

        /// Gets underlying string value wrapped in Result
        fn get_string_value_unchecked(&self) -> Result<String, ParserError>;

        /// Gets underlying i64 value  wrapped in Result
        fn get_signed_num_64_value_unchecked(&self) -> Result<i64, ParserError>;

        /// Gets underlying u64 value  wrapped in Result
        fn get_unsigned_num_64_value_unchecked(&self) -> Result<u64, ParserError>;

        /// Gets underlying u32 value  wrapped in Result
        fn get_unsigned_num_32_value_unchecked(&self) -> Result<u32, ParserError>;

        /// Gets underlying i16 value  wrapped in Result
        fn get_signed_num_16_value_unchecked(&self) -> Result<i16, ParserError>;

        /// Gets underlying u16 value  wrapped in Result
        fn get_unsigned_num_16_value_unchecked(&self) -> Result<u16, ParserError>;

        /// Gets underlying u8 value  wrapped in Result
        fn get_u8_vec_unchecked(&self) -> Result<&Vec<u8>, ParserError>;

        /// Gets underlying string value as Option
        fn get_string_value(&self) -> Option<String>;

        /// Gets underlying i64 value as Option
        fn get_signed_num_64_value(&self) -> Option<i64>;

        /// Gets underlying u64 value as Option
        fn get_unsigned_num_64_value(&self) -> Option<u64>;

        /// Gets underlying u32 value as Option
        fn get_unsigned_num_32_value(&self) -> Option<u32>;

        /// Gets underlying i16 value as Option
        fn get_signed_num_16_value(&self) -> Option<i16>;

        /// Gets underlying u16 value as Option
        fn get_unsigned_num_16_value(&self) -> Option<u16>;

        /// Gets underlying data Vec<u8> Option
        fn get_u8_vec(&self) -> Option<&Vec<u8>>;
    }

    impl<'a> ValueExtractor<'a> for Value {
        fn get_string_value(&self) -> Option<String> {
            return match &self {
                Value::String(ref data) => Some(data.clone()),
                Value::UnSignedNumber16(ref data) => Some(data.to_string()),
                Value::UnSignedNumber32(ref data) => Some(data.to_string()),
                Value::UnSignedNumber64(ref data) => Some(data.to_string()),
                Value::SignedNumber16(ref data) => Some(data.to_string()),
                Value::SignedNumber64(ref data) => Some(data.to_string()),

                _ => {
                    return None;
                }
            };
        }

        fn get_signed_num_64_value(&self) -> Option<i64> {
            return match self {
                Value::SignedNumber64(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        fn get_unsigned_num_64_value(&self) -> Option<u64> {
            return match self {
                Value::UnSignedNumber64(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        fn get_unsigned_num_32_value(&self) -> Option<u32> {
            return match self {
                Value::UnSignedNumber32(data) => Some(*data),
                Value::String(data) => Some(data.parse::<u32>().unwrap()),

                _ => {
                    return None;
                }
            };
        }

        fn get_signed_num_16_value(&self) -> Option<i16> {
            return match self {
                Value::SignedNumber16(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        fn get_unsigned_num_16_value(&self) -> Option<u16> {
            return match self {
                Value::UnSignedNumber16(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        fn get_u8_vec(&self) -> Option<&Vec<u8>> {
            return match self {
                Value::U8Vec(data) => Some(data),

                _ => {
                    return None;
                }
            };
        }
        
        fn get_string_value_unchecked(&self) -> Result<String, ParserError> {
            match self.get_string_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get String value from {:?}", self)))
            }
        }
        
        fn get_signed_num_64_value_unchecked(&self) -> Result<i64, ParserError> {
            match self.get_signed_num_64_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get signed 64 value from {:?}", self)))
            }
        }
        
        fn get_unsigned_num_64_value_unchecked(&self) -> Result<u64, ParserError> {
            match self.get_unsigned_num_64_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get unsigned 64 value from {:?}", self)))
            }
        }

        fn get_unsigned_num_32_value_unchecked(&self) -> Result<u32, ParserError> {
            match self.get_unsigned_num_32_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get unsigned 32 value from {:?}", self)))
            }
        }

        fn get_signed_num_16_value_unchecked(&self) -> Result<i16, ParserError> {
            match self.get_signed_num_16_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get signed 64 value from {:?}", self)))
            }
        }
        
        fn get_unsigned_num_16_value_unchecked(&self) -> Result<u16, ParserError> {
            match self.get_unsigned_num_16_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get unsigned 64 value from {:?}", self)))
            }
        }
        
        fn get_u8_vec_unchecked(&self) -> Result<&Vec<u8>, ParserError> {
            match self.get_u8_vec(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get vec of bytes value from {:?}", self)))
            }
        }
    }
    

    /// Value wraps underlying data. It is wrapper for string, number and bytes 
    #[allow(unused)]
    #[derive(Debug, Clone)]
    pub enum Value {
        String(String),
        SignedNumber64(i64),
        UnSignedNumber64(u64),
        UnSignedNumber32(u32),
        SignedNumber16(i16),
        UnSignedNumber16(u16),
        U8Vec(Vec<u8>),
        
        None,
    }

    /// Value Type enum list various types used to represent underlying protocol data 
    #[derive(PartialEq)]
    pub enum ValueType {
        String,
        SignedNumber64,
        UnSignedNumber64,
        UnSignedNumber32,
        SignedNumber16,
        UnSignedNumber16,
        U8Vec,
        None,
        CompositeMap,
        CompositeList
    }

    impl ValueType{

        /// Converts slice into Value based on value type
        pub fn parse(value_type: &ValueType, value: &[u8]) -> Value {                

            match value_type {
                ValueType::String => {
                                            Value::String(String::from_utf8(value.to_vec()).unwrap())
                                        }
                
                ValueType::SignedNumber64 => {
                                            Value::SignedNumber64(i64::from_be_bytes(value.try_into().unwrap()))
                                        }

                ValueType::UnSignedNumber64 => {
                                            Value::UnSignedNumber64(u64::from_be_bytes(value.try_into().unwrap()))
                                        }   
                ValueType::UnSignedNumber32 => {
                    Value::UnSignedNumber32(u32::from_be_bytes(value.try_into().unwrap()))
                    }
                ValueType::SignedNumber16 => {
                    Value::SignedNumber16(i16::from_be_bytes(value.try_into().unwrap()))
                }
                ValueType::UnSignedNumber16 => {
                    Value::UnSignedNumber16(u16::from_be_bytes(value.try_into().unwrap()))
                },
                ValueType::U8Vec => {
                                            Value::U8Vec(value.to_vec())
                                        }
                _ => {
                    Value::None
                }
            }
        }
    }

    /// Serializes `Value` to  Writer
    #[allow(unused)]
    async fn write<W: AsyncWrite + Unpin>(value: &Value, mut writer: W) -> Result<(), ParserError> {
        match value {
            Value::String(s) => {
                            writer.write(s.as_bytes()).await?;
                        }
            Value::SignedNumber64(num) => {
                            writer.write_i64(*num).await?;
                        }
            Value::UnSignedNumber64(num) => {
                            writer.write_u64(*num).await?;
                        }
            Value::UnSignedNumber32(num) => {
                writer.write_u32(*num).await?;
            }
            Value::U8Vec(data) => {
                            writer.write_all(&data[..]).await?;
                        }
            Value::SignedNumber16(num) => {
                writer.write_i16(*num).await?;
            }
            Value::UnSignedNumber16(num) => {
                writer.write_u16(*num).await?;
            },
            _ => todo!(),
        }
        Ok(())
    }

    impl Value {

        /// Serializes Value to a AsyncWrite
        #[allow(unused)]
        async fn write<W: AsyncWrite + Unpin>(& self, mut writer: W) -> Result<(), ParserError> {
            match self {
                Value::String(s) => {
                                            writer.write(s.as_bytes()).await?;
                                        }
                Value::SignedNumber64(num) => {
                                            writer.write_i64(*num).await?;
                                        }
                Value::UnSignedNumber64(num) => {
                                            writer.write_u64(*num).await?;
                                        }
                Value::UnSignedNumber32(num) => {
                                writer.write_u32(*num).await?;
                            }
                Value::U8Vec(data) => {
                                            writer.write_all(&data[..]).await?;
                                        }
                Value::SignedNumber16(num) => {
                                writer.write_i16(*num).await?;
                            }
                Value::UnSignedNumber16(num) => {
                                writer.write_u16(*num).await?;
                            },
                Value::None => todo!(),
            }
            Ok(())
        }
    }


    /// The trait provides methods to get deserialized protocol data.    
    pub trait InfoProvider:  Send + Sync{

        /// Gets the data using key as lookup        
        fn get_info(&self, key: &String) -> Option<&Value>{
            self.get_mapper().get_value_by_key(key)
        }

        /// Gets the data using spec path as lookup        
        fn get_info_by_spec_path(&self, spec_path: &String) -> Option<&Value>{
            self.get_mapper().get_spec_data().get(spec_path)
        }

        /// Gets the data using key and spec name. This is used when querying Keyvalue pair that is generated using RepeatMany Spec
        /// For Http headers example, key could be Content-Type and spec_name is `header_name`(header_name is specified in Spec builder)
        fn get_key_value_info_by_spec_name(&self, key: String, spec_name: &String) -> Option<&Value>{
            let path = self.get_mapper().get_mapping_data_template().get(spec_name);
            if let Some(path) = path{
                let full_path = format!("{}.{}", path, key);
                let value_path = self.get_mapper().get_mapping_data_template().get(&full_path);
                if let Some(value_path) = value_path{
                    return self.get_mapper().get_spec_data().get(value_path);
                }else {
                    return None;
                }
            }else{
                return None;
            }
            
        }

        /* #[allow(unused)]
        fn get_info_mut(&mut self, key: &String) -> Option<&mut Value>;

        #[allow(unused)]
        fn get_keys_by_group_name(&self, name: String) -> Option<Vec<& String>>; */

        /// Adds simple key and a value
        fn add_info(&mut self, key: String, value: Value) -> Result<(), ParserError>{
            self.get_mapper_mut().add_simple_data(key, value)
        }

        /// Adds info by using key, key spec name, value and value spec_name
        /// For http headers example 
        /// key -> Content-Type,
        /// value -> application/json,
        /// key_spec_name -> header_name
        /// value_spec_name -> header_value
        fn add_info_by_spec_path(&mut self, key: String, key_spec_name: String, value: Value , value_spec_name: String) {
            self.get_mapper_mut().add_to_key_value_list(key, value, key_spec_name, value_spec_name);
        }

        //fn add_transient_info(&mut self, key: String, value: Value);

        //fn has_all_data(&self) -> bool;

        /// Gets mutable mapper reference
        fn get_mapper_mut(&mut self) ->&mut Box<dyn Mapper>;

        /// Gets shared mutable mapper reference
        fn get_mapper(&self) ->&Box<dyn Mapper>;

        /// Gets mapper context        
        fn get_mapper_context(&mut self) ->&mut MapperContext{
            self.get_mapper_mut().get_mapper_context_mut()
        }
    }

    /// Represents Contextual data of RepeatManySpec e.g holds current count
    #[derive(Clone, Debug)]
    pub struct RepeaterContext{
        count: u32
    }

    impl RepeaterContext{

        fn new()->Self{
            Self { count: 0 }
        }

        fn get_count(&self) -> u32{
            return self.count;
        }

        fn next(&mut self) -> u32{
            self.count+=1;
            return self.count;
        }
    }



    /// trait to represent protocol request information
    pub trait RequestInfo: InfoProvider {
    }

    /// trait to represent protocol response information
    pub trait ResponseInfo: InfoProvider {
        fn add_defaults(&mut self) -> Result<(), ParserError>;

    }

    /// RequestFactory contains methods to generate Request related objects e.g RequestInfo, RequestSerializer,
    /// RequestDeserializer, ReqiestErrorHandler, Request Spec
    #[allow(unused)]
    pub trait RequestFactory<REQI, REQSER, REQH, REQERRH, RESI> : Send + Sync
    where
        REQI: RequestInfo,
        REQSER: RequestSerializer<REQI>,
        REQH: RequestHandler<REQI, RESI>,
        REQERRH: RequestErrorHandler<REQI, RESI>,
        RESI: ResponseInfo,
    {
        ///Gets the request specification
        fn get_request_spec(&self) -> &Box<dyn ProtocolSpec>;

        ///Creates request info object
        fn create_request_info(&self) -> REQI;        
        
        ///Creates request serializer
        fn create_request_serializer(&self) -> REQSER;

        ///Creates  request handler
        fn create_request_handler(&self) -> REQH;

        ///Creates request error handler
        fn create_error_request_handler(&self) -> REQERRH;
    }

    /// ResponseFactory contains methods to generate Response related objects e.g ResponseInfo, ResponseSerializer,
    /// ResponseDeserializer, ResponseErrorHandler, Response Spec
    pub trait ResponseFactory<RESI, RESS, RESH, RESERRH>: Send + Sync
    where
        RESI: ResponseInfo,
        RESS: ResponseSerializer<RESI>,
        RESH: ResponseHandler<RESI>,
        RESERRH: ResponseErrorHandler<RESI>,
    {
        ///Gets response spec created using the spec builder
        fn get_response_spec(&self) -> &Box<dyn ProtocolSpec>;

        /// Creates ResponseInfo object
        fn create_response_info(&self) -> Result<RESI, ParserError>;

        /// Creates Response Serializer object
        fn create_response_serializer(&self) -> RESS;

        /// Creates Response handler object
        fn create_response_handler(&self) -> RESH;

        /// Creates Response error handler object
        fn create_error_response_handler(&self) -> RESERRH;
    }

    /// Request Handler trait
    #[async_trait]
    pub trait RequestHandler<REQI, RESI> : Send + Sync
    where
        REQI: RequestInfo,
        RESI: ResponseInfo,
    {
        /// handles the request
        /// * `request` - RequestInfo object containing deserialized request data
        /// * `response` - Response infomation to be popuated by this method
        /// * returns - Result of ResponseInfo
        async fn handle_request(&self, request: &REQI, response: &mut RESI) -> Result<RESI, ParserError>;
    }

    pub trait ResponseHandler<RESI> : Send + Sync
    where
        RESI: ResponseInfo,
    {
        /// handles the response
        /// * `response` - ResponseInfo object containing deserialized response data. This is mainly used by protocol client to handle the response generated by the server                        
        fn handle_response(&self, response: &RESI) -> Result<(), ParserError>;
    }

    /// Response Error Handler trait
    pub trait ResponseErrorHandler<RESI>  : Send + Sync
    where
        RESI: ResponseInfo,
    {
        /// handles the error response
        /// * `response` - ResponseInfo object containing deserialized response data. This is mainly used by protocol client to handle the response generated by the server                        
        /// * `error` - Error data
        #[allow(unused)]
        fn handle_response_error<E>(
            &self,
            response_info: &RESI,
            error: E,
        ) -> Result<(), ParserError>;
    }

    
    /// handles the error request
    /// * `response` - RequestInfo object containing deserialized request data. 
    /// * `error` - Error data
    pub trait RequestErrorHandler<REQI, RESI>: Send + Sync
    where
        REQI: RequestInfo,
        RESI: ResponseInfo,
    {
        /// handles the error request
        /// * `request` - RequestInfo object containing deserialized request data. 
        /// * `error` - Error data
        #[allow(unused)]
        fn handle_request_error<E>(&self, request: &REQI, error: E) -> Result<RESI, ParserError>;
    }


    /// Serializer for request
    #[async_trait]
    pub trait RequestSerializer<
        REQI: RequestInfo> : Send + Sync
    {

        /// Serializes request to writer
        /// * req - Request Info
        /// * writer - AsyncWrite implementation
        /// * spec - Request Spec
        #[allow(unused)]
        async fn serialize_to<W>(
            &self,
            req: &mut REQI,
            writer: W,
            spec: Box<dyn ProtocolSpec>,
        ) -> Result<(), ParserError>
        where W: AsyncWrite + Unpin + Send + Sync;

        /// DeSerializes request from reader stream
        /// * req - Request Info
        /// * reader - AsyncRead implementation
        /// * spec - Request Spec
        async fn deserialize_from<'a, B>(
            &self,
            request_info: &'a mut REQI,
            reader: B,
            spec: &dyn SpecDeserialize,
        ) -> Result<&'a mut REQI, ParserError> where B:AsyncRead + Unpin + Send + Sync;        
    }


    /// Serializer for Response
    #[async_trait]
    pub trait ResponseSerializer<RSI>: Send + Sync 
    where RSI: ResponseInfo ,
        
    {
        /// Serializes response info to writer
        /// * res - Response Info
        /// * writer - AsyncWrite implementation
        /// * spec - Response Spec
        async fn serialize_to<W>(
            &self,
            res: RSI,
            writer: W,
            spec: &Box<dyn ProtocolSpec>,
        ) -> Result<(), ParserError>
        where W: AsyncWrite + Unpin + Send + Sync;

        /// DeSerializes response from reader stream
        /// * response_info - Response Info
        /// * reader - AsyncRead implementation
        /// * spec - Response Spec
        #[allow(unused)]
        async fn deserialize_from<'a, R>(&self,  
            response_info: &'a mut RSI,
            reader: &mut BufReader<R>,
            spec: &dyn SpecDeserialize) -> Result<&'a mut RSI, ParserError>
        where R:SpecRead;
    }

    // Default Serializer struct
    #[allow(unused)]
    pub struct DefaultSerializer;

    #[async_trait]
    impl<REQI>
        RequestSerializer<REQI> for DefaultSerializer
        where 
            REQI: RequestInfo + 'static,
    {
        async fn serialize_to<W>(
            &self,
            request_info: &mut REQI,
            writer: W,
            spec: Box<dyn ProtocolSpec>,
        ) -> Result<(), ParserError> 
        where W: AsyncWrite + Unpin + Send + Sync {
            let mut mapper_context = MapperContext::new();
            let mut protocol_writer = ProtocolBuffWriter::new(writer);

            serialize(&spec, request_info,  &mut protocol_writer, &mut mapper_context).await?;
            Ok(())
        }

        async fn deserialize_from<'a, B>(
            &self,
            request_info:  &'a mut REQI,
            reader: B,
            spec: &dyn SpecDeserialize,
        )  -> Result<&'a mut REQI, ParserError> 
        where B:AsyncRead + Unpin + Send + Sync  {
            let mut protocol_reader = ProtocolBuffReader::new( BufReader::new(reader), 1024);
            spec.deserialize(request_info,&mut  protocol_reader, true).await?;
            /* if let Err(parser_error) = parse_result{
                if let ParserError::EndOfStream = parser_error  {
                    if request_info.has_all_data() {
                        return Ok(request_info);
                    }
                    return Err(ParserError::EndOfStream);
                } else {
                    return Err(parser_error);
                }
            } */
           //todo!("do we need the above error handling");
            Ok(request_info)
        }        
    }

    

    

    #[async_trait]
    impl<RESI>
        ResponseSerializer<RESI> for DefaultSerializer
        where 
            RESI: ResponseInfo + 'static,
    {
        async fn serialize_to<W>(
            &self,
            response_info: RESI,
            writer: W,
            spec: &Box<dyn ProtocolSpec>,
        ) -> Result<(), ParserError> where W: AsyncWrite + Unpin + Send + Sync {
            let mut protocol_writer = ProtocolBuffWriter::new(writer);
            let mut mapper_context= MapperContext::new();
            serialize(spec, &response_info, &mut protocol_writer, &mut mapper_context).await?;
            Ok(())
        }

        //(&self, mut response_info: RSI,reader: R, spec: &Placeholder)
        //async fn deserialize_from(&self,  response_info: &mut RSI,reader: &mut BufReader<&mut R>, spec: &Placeholder) -> Result<RSI, ParserError>;

        async fn deserialize_from<'a, R>(
            &self,
            response_info:&'a mut RESI,
            reader: &mut BufReader< R>,
            spec: &dyn SpecDeserialize,
        ) -> Result<&'a mut RESI, ParserError> 
        where R:SpecRead {
            let mut protocol_reader = ProtocolBuffReader::new(reader, 1024);
            spec.deserialize(response_info,&mut  protocol_reader, true).await?;
           //todo handle the above
            Ok(response_info)
        }        
    }

    /// For future use to support binary and UDP protocol
    #[allow(unused)]
    pub struct Protocol {
        name: ProtocolVersion,
        transport: Transport,
        format: ProtocolFormat,
        request_place_holder: ListSpec, //Placeholder,
        response_place_holder: ListSpec,
    }

    /// For future use to support UDP protocol
    #[allow(unused)]
    pub enum Transport {
        UDP,
        TCP,
    }

    /// For future use to support Text/Binary protocol
    #[allow(unused)]
    pub enum ProtocolFormat {
        Text,
        Binary,
    }

    /// For future use 
    #[allow(unused)]
    pub struct ProtocolBuilder<RQI, RSI>
    where
        RQI: RequestInfo,
        RSI: ResponseInfo,
    {
        name: Option<String>,
        version: Option<String>,
        transport: Option<Transport>,
        format: Option<ProtocolFormat>,
        request_info: Option<RQI>,
        response_info: Option<RSI>,
    }

    /// Error enum to represent server/transport related errors
    #[derive(Debug,)]    
    pub enum ServerError {
        StartError(String),
        StopError,
        RequestError(ParserError),
        ResponseError(ParserError),
        IOError(std::io::Error),
    }

    #[async_trait]
    pub trait Server {
        #[allow(unused)]
        async fn start(&'static mut self) -> Result<(), ServerError>;

        #[allow(unused)]
        async fn stop(&mut self) -> Result<(), ServerError>;
    }

    impl From<std::io::Error> for ServerError {
        fn from(error: std::io::Error) -> Self {
            ServerError::IOError(error)
        }
    }

    pub trait ProtocolConfig: Send + Sync
    {
        type REQF: RequestFactory<Self::REQI, Self::REQSER, Self::REQH, Self::REQERRH, Self::RESI>;
        type RESF: ResponseFactory<Self::RESI, Self::RESSER, Self::RESH, Self::RESERRH>;
        type REQI: RequestInfo;
        type RESI: ResponseInfo;
        type REQSER: RequestSerializer<Self::REQI>;
        type RESSER: ResponseSerializer<Self::RESI>;

        type REQH: RequestHandler<Self::REQI, Self::RESI>;
        type RESH: ResponseHandler<Self::RESI>;
        type REQERRH: RequestErrorHandler<Self::REQI, Self::RESI>;
        type RESERRH: ResponseErrorHandler<Self::RESI>;
    }

    struct MapperAwareRequestFactory<T> where T:ProtocolConfig{
        inner: T::REQF,
        mapper: Box<dyn Mapper>,
    }

    struct MapperAwareResponseFactory<T> where T:ProtocolConfig{
        inner: T::RESF,
        mapper: Box<dyn Mapper>,
    }
    /*
     REQI: RequestInfo,
        REQSER: RequestSerializer<REQI>,
        REQH: RequestHandler<REQI, RESI>,
        REQERRH: RequestErrorHandler<REQI, RESI>,
        RESI: ResponseInfo,
     */

    impl <T> MapperAwareRequestFactory<T> where T: ProtocolConfig{
        fn new(inner: T::REQF) -> Self{
            
            let mut mapper: Box<dyn Mapper> = Box::new(DefaultMapper::new());
            let result = inner.get_request_spec().traverse(&mut mapper);
            if result.is_err(){
                panic!("unexpected error while parsing request spec {}", result.unwrap_err());
            }
            Self { inner, mapper }
        }
    }

    impl <T> RequestFactory<T::REQI, T::REQSER, T::REQH, T::REQERRH, T::RESI> for MapperAwareRequestFactory<T> where T: ProtocolConfig{
        fn get_request_spec(&self) -> &Box<dyn ProtocolSpec> {
            self.inner.get_request_spec()
        }
    
        fn create_request_info(&self) -> T::REQI {
            let mut request_info = self.inner.create_request_info();
            self.mapper.get_mapping_data_template().clone_into(request_info.get_mapper_mut().get_mapping_data_template_mut());
            request_info            

        }
    
        fn create_request_serializer(&self) -> T::REQSER {
            self.inner.create_request_serializer()
        }
    
        fn create_request_handler(&self) -> T::REQH {
            self.inner.create_request_handler()
        }
    
        fn create_error_request_handler(&self) -> T::REQERRH {
            self.inner.create_error_request_handler()
        }
    }

    impl <T> MapperAwareResponseFactory<T> where T: ProtocolConfig{
        fn new(inner: T::RESF) -> Self{
            
            let mut mapper: Box<dyn Mapper> = Box::new(DefaultMapper::new());
            let result = inner.get_response_spec().traverse(&mut mapper);
            if result.is_err(){
                panic!("unexpected error while parsing response spec {}", result.unwrap_err());
            }
            Self { inner, mapper }
        }
    }

    impl <T> ResponseFactory<T::RESI, T::RESSER, T::RESH, T::RESERRH, > for MapperAwareResponseFactory<T> where T: ProtocolConfig{
        fn get_response_spec(&self) -> &Box<dyn ProtocolSpec> {
            self.inner.get_response_spec()
        }
    
        fn create_response_info(&self) -> Result<T::RESI, ParserError> {
            let mut response_info = self.inner.create_response_info()?;
            self.mapper.get_mapping_data_template().clone_into(response_info.get_mapper_mut().get_mapping_data_template_mut());
            response_info.add_defaults()?;
            Ok(response_info)           

        }
    
        fn create_response_serializer(&self) -> T::RESSER {
            self.inner.create_response_serializer()
        }
    
        fn create_response_handler(&self) -> T::RESH {
            self.inner.create_response_handler()
        }
    
        fn create_error_response_handler(&self) -> T::RESERRH {
            self.inner.create_error_response_handler()
        }
    }
    
    #[derive(Builder)]
    #[builder(pattern = "owned")]
    pub struct ServerInstance<CFG> 
    where CFG: ProtocolConfig{
        hosts: Vec<String>,
        
        #[builder(setter(custom))]
        request_factory: MapperAwareRequestFactory<CFG>,
        
        #[builder(setter(custom))]        
        response_factory: MapperAwareResponseFactory<CFG>,

        #[builder(setter(skip))]
        listeners: Vec<TcpListener>,
    }

    impl <CFG:ProtocolConfig> ServerInstanceBuilder<CFG>    {
        pub fn request_factory( mut self, value: CFG::REQF) ->  Self{
            self.request_factory = Some(MapperAwareRequestFactory::new(value));
            self
        }

        pub fn response_factory(mut self, value: CFG::RESF) -> Self{
            self.response_factory = Some(MapperAwareResponseFactory::new(value));
            self
        }
    }

    
    impl <CFG> ServerInstance<CFG> 
    where CFG: ProtocolConfig,
                 
    {
        #[allow(unused)]
        async fn create_listeners(&mut self) -> Result<(), ServerError> {
            for host in &self.hosts {
                let option = host.split_once(":");
                match option {
                    Some((host, port)) => {
                        let listener = TcpListener::bind(format!("{}:{}", host, port))
                            .await
                            .unwrap();
                        self.listeners.push(listener);
                    }
                    None => {
                        return Err(ServerError::StartError(
                            "Invalid host:port format".to_owned(),
                        ));
                    }
                }
            }
            Ok(())
        }

        async fn accept_connections(&'static self, tcp_listener: &'static TcpListener) {
            tokio::spawn(async move {
                loop {
                    let (socket, addr) = tcp_listener.accept().await.unwrap();
                    info!("Accepted connection from {}", addr);

                    let _handle = tokio::spawn(async move {
                        let result = self.handle_connection(socket).await;
                        if result.is_err(){
                            warn!("error handing request from addr {}, {}", addr.ip(), result.unwrap_err());
                        }
                    });
                }
            });
        }

        async fn handle_connection(&'static self, mut socket: TcpStream) -> Result<(), ParserError> {
            let mut req_info = self.request_factory.create_request_info();
            let serializer = self.request_factory.create_request_serializer();            
            let mut res_info = self.response_factory.create_response_info()?;
            
            
            
            let mut buf_reader  = BufReader::new(&mut socket);  
             let request_info = 
             serializer
                .deserialize_from(
                    &mut req_info,
                    &mut buf_reader,
                    self.request_factory.get_request_spec(),
                )
                .await?; 
            let result = CFG::REQH::handle_request(
                &self.request_factory.create_request_handler(),
                &request_info,
                &mut res_info
            ).await;
            match result {
                Ok(response_info) => {
                    let serializer = self.response_factory.create_response_serializer();
                    let spec= self.response_factory.get_response_spec();
                    return serializer
                        .serialize_to(
                            response_info,
                            socket,
                            spec
                        )
                        .await;
                }
                Err(e) => {
                    warn!("Error handling request: {:?}", e);
                    return Err(e);
                }
            } 
        }
    }

    #[async_trait]
    impl<CFG> Server for ServerInstance<CFG> 
    where CFG: ProtocolConfig{
        async fn start(&'static mut self) -> Result<(), ServerError> {
            self.create_listeners().await?;
            
            for listener in &self.listeners {
                let _result = self.accept_connections(listener).await;
                info!("hh{:?}", listener);
            }

            Ok(())
        }

        async fn stop(&mut self) -> Result<(), ServerError> {
            todo!()
        }

        
        
       /*  async fn configure_mappers(&mut self) -> Result<(), ServerError>{
            //configure request mapper templates
            

            //configure response mapper templates
            let mapper = self.response_factory.get_default_mapper_mut();
            let spec = self.response_factory.get_response_spec();
            spec.traverse(mapper);
            Ok(())
        } */
    }

    #[allow(unused)]
    pub trait Processor {
        fn process(_req: Request, _res: Response) {
        }
    }

    #[allow(unused)]
    pub struct ProtocolVersion {
        name: String,
        version: Option<String>,
    }

    pub struct Request {}

    pub struct Response {}

    pub enum HeaderValue{
        String,
        
    }

    #[allow(unused)]
    pub enum PlaceHolderIdentifier {
        #[allow(unused)]
        Name(String),
        #[allow(unused)]
        Key,
        InlineKeyWithValue(String),
        Value,
    }

    impl Default for PlaceHolderIdentifier {
        fn default() -> Self {
            PlaceHolderIdentifier::Name(String::new())
        }
    }

    impl Debug for PlaceHolderIdentifier{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name(arg0) => f.debug_tuple("Name").field(arg0).finish(),
            Self::Key => write!(f, "Key"),
            Self::InlineKeyWithValue(arg0) => f.debug_tuple("InlineKeyWithValue").field(arg0).finish(),
            Self::Value => write!(f, "Value"),
        }
    }
    }

    #[allow(dead_code)]
    trait TokenParser {
        async fn read_string(until_delimiter: String) -> String;
    }
    pub trait SpecRead: PlaceHolderRead + MarkAndRead + AsyncRead + Unpin + Send + Sync {
    }

    pub trait SpecWrite: PlaceHolderWrite + AsyncWrite + Unpin + Send + Sync {
    }

    #[async_trait]
    pub trait SpecDeserialize: Send + Sync {
        async fn deserialize (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead), update_info: bool,
        ) -> Result<Value, ParserError>;
    }

    struct SpecDeserializer<'a, S> where S: SerializableSpec{
        inner: &'a S
    }

    struct SpecSerializer<'a, S> where S: ProtocolSpec + ?Sized{
        inner: &'a S
    }

    
    fn begin<S>(spec:&S, mapper_context:&mut MapperContext) where S: SerializableSpec{
            let spec_type = spec.to_spec_type();
            mapper_context.start_spec_type(spec_type);
    }

    fn end_current_context(mapper_context: &mut MapperContext){
        mapper_context.end_current_spec();
    }

    async fn serialize<S>(spec: &S, info_provider: & ( dyn InfoProvider + Send + Sync ), 
        writer: &mut (dyn SpecWrite), 
        mapper_context: &mut MapperContext) -> Result<(), ParserError>
        where S: ProtocolSpec + ?Sized{
            //SpecDeserialize
        let serialier = SpecSerializer{
            inner: spec
        };
        return serialier.serialize(info_provider, mapper_context, writer).await;
    }

    #[async_trait]
    impl<'a, S> SpecSerialize for SpecSerializer<'a, S> where S:ProtocolSpec + ?Sized{

        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>{
            mapper_context.start_spec(self.inner);
            let result = self.inner.serialize(info_provider, mapper_context, writer).await;
            let end_spec_result = mapper_context.end_spec(self.inner);
            if result.is_err(){
                return result;
            }
            return end_spec_result;
        }
    }

    #[async_trait]
    impl <'a, S> SpecDeserialize for SpecDeserializer<'a, S> where S:SerializableSpec{
        async fn deserialize (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead), update_info: bool,
        ) -> Result<Value, ParserError>{            
            begin(self.inner, info_provider.get_mapper_mut().get_mapper_context_mut());
            let value_result = self.inner.deserialize(info_provider, reader, update_info).await;
            end_current_context(info_provider.get_mapper_mut().get_mapper_context_mut());
            return value_result;
            
            /* if let Ok(value) = value_result{
                if let SpecName::Name(_) = self.inner.get_meta_data().get_name() {
                    let context: &mut MapperContext = info_provider.get_mapper_mut().get_mapper_context_mut();
                    let spec_name = context.get_last_available_spec_name();
                    if update_info{
                        info_provider.get_mapper_mut().get_spec_data_mut().insert(spec_name, value );
                    }
                    end_current_context(info_provider.get_mapper_mut().get_mapper_context_mut());
                    return Ok(Value::None);
                }else{
                    end_current_context(info_provider.get_mapper_mut().get_mapper_context_mut());
                    return Ok(value);
                } 
                
            }else {
                end_current_context(info_provider.get_mapper_mut().get_mapper_context_mut());
                return value_result;
            }*/
            
            // should we clone the value instead of sending None?
            
        }
    }

    #[derive(Clone, Debug, )]
    pub enum SpecType{
        Composite(SpecName),
        RepeatMany(SpecName, RepeatCount, u16),
        
        Key(SpecName),
        Value(SpecName),
        Simple(SpecName),
    }

    impl PartialEq for SpecType{
        fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Composite(l0), Self::Composite(r0)) => l0 == r0,
            (Self::RepeatMany(l0, _l1, _l2), Self::RepeatMany(r0, _r1, _r2)) => l0 == r0,
            (Self::Key(l0), Self::Key(r0)) => l0 == r0,
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::Simple(l0), Self::Simple(r0)) => l0 == r0,
            _ => false,
        }
    }
    }

    impl SpecType{
        fn to_path_template_string(&self) ->String{

            match self{
                
                SpecType::RepeatMany(spec_name, _, _) => format!("{}.{{}}", spec_name.to_path_string()),
                SpecType::Composite(spec_name) | 
                SpecType::Key(spec_name) | 
                SpecType::Value(spec_name) |
                SpecType::Simple(spec_name) => spec_name.to_path_string(),
            }
        }

        fn to_path_string(&self) ->String{
            match self{
                SpecType::RepeatMany(name, _, current_index) => format!("{}.{}", name.to_name_string(), current_index),
                
                SpecType::Composite(spec_name) | 
                SpecType::Key(spec_name) | 
                SpecType::Value(spec_name) |
                SpecType::Simple(spec_name) => spec_name.to_path_string(),
            }
        }
    }

    
    #[async_trait]
        pub trait  SpecSerialize: Send + Sync/* :Spec */{
        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>;
        
    }

    #[derive(PartialEq, Clone, Debug)]
    pub enum SpecName{
        NoName,
        Name(String),
        Transient(String),
        Delimiter
    }

    impl Display for SpecName{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self{
                SpecName::NoName => write!(f, "{}", "noName"),
                SpecName::Name(name) => write!(f, "{}", name),
                SpecName::Transient(name) =>  write!(f, "transient {}", name),
                SpecName::Delimiter =>  write!(f, "{}", "delimiter"),
            }
        }
    }

    impl SpecName{
        fn to_path_string(&self) -> String{
            match self{
              
                SpecName::Name(name) => name.to_owned(),
                SpecName::Transient(name) => name.to_owned(),
                SpecName::NoName => "NoName".to_owned(),
                SpecName::Delimiter => "Delimiter".to_owned(),
            }
        }

        fn is_delimiter(&self) -> bool{
            match self{
                SpecName::Delimiter => true,
                _ => false
            }
        }
    }

    impl Into<String> for &SpecName{
        fn into(self) -> String {
            if let SpecName::Name(name) = self{
                return name.to_owned();
            }

            if let SpecName::Transient(name) = self{
                return name.to_owned();
            }

            "default".to_owned()
        }
    }
 
    #[async_trait]
    impl SpecSerialize for InlineKeyWithValue{
        
        async fn serialize (
            &self,
            info_provider: &( dyn InfoProvider + Send + Sync ), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>{
            let result = serialize(&self.0, info_provider, writer, mapper_context).await;
            if !self.1.optional & result.is_err(){
                return result;
            }
            return Ok(())
        }
    }

    /* struct UndoableDeserializer<'a, S> where S:SpecDeserialize {
        inner:  &'a S
    } */

   struct UndoableDeserializer<'a, S> where S: SerializableSpec{
        inner:  SpecDeserializer<'a, S>,
    }

    async fn peek_undoable_deserialize<S>(spec: &S,  info_provider: &mut ( dyn InfoProvider + Send + Sync ), reader: &mut (dyn SpecRead), update_info: bool,) -> Result<Value, ParserError>
        where S: SerializableSpec {           

            let marker = reader.mark();
            let result = spec.deserialize(info_provider, reader,update_info).await;            
            match result {
                Ok(value_type) => {
                    reader.reset(&marker)?;
                    return Ok(value_type);                    
                }
                Err(e) => {
                    reader.reset(&marker)?;
                    return Err(e);
                }
            }        
    }
        
    

    async fn undoable_deserialize<S>(spec: &S, info_provider: &mut ( dyn InfoProvider + Send + Sync ), reader: &mut (dyn SpecRead), update_info: bool,) -> Result<Value, ParserError>
        where S: SerializableSpec {
            //SpecDeserialize
        let serialier = SpecDeserializer{
            inner: spec
        };
        let undoable_serializer = UndoableDeserializer{
                inner: serialier,
        };

        
        //test11(undoable_serializer, info_provider, reader);
        undoable_serializer.deserialize(info_provider, reader, update_info).await
        
    }

     /* fn test11<S>(s:UndoableDeserializer<S>, info_provider: &mut ( dyn InfoProvider + Send + Sync ), reader: &mut (dyn SpecRead)) 
     where S:SpecDeserialize{
       //s.deserialize(info_provider, reader)
    }  */

    #[async_trait]
    impl <'a, T> SpecDeserialize for UndoableDeserializer<'a, T> where T:SerializableSpec{        
        async fn deserialize (
            &self,  
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead), update_info: bool,
        ) -> Result<Value, ParserError>{
            let marker = reader.mark();
            let result = self.inner.deserialize(info_provider, reader,update_info).await;            
            match result {
                Ok(value_type) => {
                    reader.unmark(&marker)?;
                    return Ok(value_type);                    
                }
                Err(e) => {
                    match e{
                        
                        ParserError::EndOfStream => {
                            let optional = self.inner.inner.get_meta_data().is_optional();
                            if optional{
                                warn!("EOS reached when trying to parse optional spec {}", self.inner.inner.get_meta_data().get_name().to_path_string());
                                return Ok(Value::None);
                            }
                            reader.reset(&marker)?;
                            return Err(e);
                        }
                        _ => {
                            reader.reset(&marker)?;
                            return Err(e);
                        }
                    }
                    
                }
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Separator{
        Delimiter(String),
        NBytes(u32),
        EndOfStream,
    }

    impl Default for Separator {
        fn default() -> Self {
            Separator::EndOfStream
        }
    }   

    #[derive( PartialEq)]
    pub struct SpecMetaData{
        name: SpecName,
        value_type: ValueType,
        optional: bool,
    }

    trait ToName {
        fn to_name_string(&self) ->String;
    }

    impl ToName for SpecName{
        fn to_name_string(&self) ->String {
            self.to_path_string()
            /* match self{
                SpecName::Name(name)  => name.to_owned(),
                _ => "Default".to_owned(),
                
            } */
        }
    }

    impl Default for SpecMetaData{
        fn default() -> Self {
            SpecMetaData {
                name: SpecName::NoName,
                value_type: ValueType::None,
                optional: false,
            }
        }
    }

    impl  SpecMetaData{
        pub fn new(name: SpecName, value_type: ValueType, optional: bool) -> Self {
            SpecMetaData {
                name,
                value_type,
                optional,
            }
        }

        pub fn get_name(&self) -> &SpecName {
            &self.name
        }
        pub fn get_value_type(&self) -> &ValueType {
            &self.value_type
        }
        pub fn is_optional(&self) -> bool {
            self.optional
        }
    }

    pub trait SimpleValueSpec: Spec + SpecSerialize + SpecDeserialize + MappableSpec + SpecTraverse + ToSpecType {}

    pub trait DelimitedSpec: SimpleValueSpec + Default{
        fn set_delimiter(&mut self, delimiter: Separator) ;
        fn get_delimiter(& self) -> &Separator;
    }

    pub trait StringSpec: SimpleValueSpec + Send + Sync{}

    impl <T> SimpleValueSpec for T where T:StringSpec{}

    #[derive(Default)]
    pub struct DelimitedStringSpec{
        spec_meta_data: SpecMetaData,
        until: Separator,
    }

    impl Spec for DelimitedStringSpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    impl  StringSpec for DelimitedStringSpec{}

    impl  DelimitedSpec for DelimitedStringSpec{
        fn set_delimiter(&mut self, delimiter: Separator)  {
            self.until = delimiter;
        }
        
        fn get_delimiter(&self) -> &Separator {
            &self.until
        }

        
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum RepeatCount{
        Fixed(u32),
        Delimited(Separator),
    }

    impl RepeatCount{
        async fn serialize (
            &self,
            _info_provider: & ( dyn InfoProvider + Send + Sync ), _mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>{
            match self {
                
                RepeatCount::Delimited(separator) => {
                    match separator{
                        Separator::Delimiter(delimiter) => {
                            writer.write_string(delimiter.to_owned()).await?;
                            return Ok(());
                        },
                        Separator::NBytes(num) => {
                            writer.write_data_u32(*num).await?;
                            return Ok(());
                        },
                        Separator::EndOfStream => {
                            Ok(())
                        },
                    }
                },

                _ =>{
                    Ok(())
                }
            }
        }
    }

    impl Default for RepeatCount{
        fn default() -> Self {
            RepeatCount::Fixed(2)
        }
    }

    #[derive(Default)]
    pub struct RepeatManySpec{
        spec_meta_data: SpecMetaData,        
        pub(crate) repeat_count: RepeatCount,
        pub(crate) constituents: ListSpec,
    }

    impl Spec for RepeatManySpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    

    #[async_trait]
    impl SpecDeserialize for RepeatManySpec{
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut dyn SpecRead, update_info: bool,
        ) -> Result<Value, ParserError> {
            // Implementation for parsing repeat many spec
            let mut repeat_count = 0;
            loop{
                info_provider.get_mapper_context().increment_current_repeat_spec();
                let result = self.constituents.deserialize(info_provider, reader, update_info).await;
                if result.is_ok() {
                    repeat_count += 1;
                }

                match &self.repeat_count{
                    RepeatCount::Fixed(count) => {
                        if result.is_err() && repeat_count < *count {
                            return result;
                        }
                        if repeat_count >= *count {
                            break;
                        }
                    },
                    RepeatCount::Delimited(ref delimiter) => {
                        match delimiter{
                            Separator::Delimiter(delimiter) => {
                                let spec: Box<dyn ProtocolSpec> = Box::new(ExactStringSpec::new(SpecName::Delimiter, delimiter.clone(), false));
                                let delimiter_result = undoable_deserialize(&spec, info_provider, reader, false).await;
                                if delimiter_result.is_ok(){
                                    break;
                                }else{
                                    if result.is_err() {
                                        return result;
                                    }
                                }
                            },

                            Separator::NBytes(n) => {
                                let spec: Box<dyn ProtocolSpec> = Box::new(NumberU32Spec(SpecMetaData::new(SpecName::Delimiter, ValueType::UnSignedNumber32, false)));
                                let number_read_result = undoable_deserialize(&spec, info_provider, reader, false).await;
                                if number_read_result.is_ok() {
                                    let value =  number_read_result.unwrap();
                                    if value.get_unsigned_num_32_value().unwrap() == *n {
                                        break;
                                    }
                                }else if result.is_err(){
                                    //return the error
                                    return result;
                                }
                            },

                            Separator::EndOfStream => {
                                //try to read a single byte 
                                let spec: Box<dyn ProtocolSpec> = Box::new(NBytesSpec::new(SpecName::Delimiter, 1, false));
                                let peek_result = peek_undoable_deserialize(
                                    &spec, info_provider, reader, update_info).await;
                                if peek_result.is_err(){
                                    //check for end of stream
                                    if peek_result.unwrap_err().is_eof(){
                                        break;
                                    }
                                }else if result.is_err(){
                                    //Not end of stream, but content of repeater result has error  
                                    return result;
                                }
                            }
                        };
                    },
                };
            }
            return Ok(Value::None);
            
            //// Return appropriate value based on parsing
        
        }
    }

    #[async_trait]
    impl SpecSerialize for RepeatManySpec
    {
        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>
        {                        
            let mut has_one_success = false;
            loop{
                
                let result = serialize(&self.constituents, info_provider, writer, mapper_context).await;
                has_one_success = has_one_success | result.is_ok();
                if result.is_ok(){                    
                    mapper_context.increment_current_repeat_spec();
                    continue;
                }else if !has_one_success && !self.get_meta_data().is_optional()  {
                    
                    //mapper_context.end_spec(self)?;
                    self.repeat_count.serialize(info_provider, mapper_context, writer).await?;
                    return result;
                }else if !has_one_success && self.get_meta_data().is_optional(){
                    //mapper_context.end_spec(self)?;
                    self.repeat_count.serialize(info_provider, mapper_context, writer).await?;
                    return Ok(());
                }else if has_one_success {
                    //mapper_context.end_spec(self)?;
                    self.repeat_count.serialize(info_provider, mapper_context, writer).await?;
                    return Ok(());
                }
            }
        }
    }


    pub trait Spec: Send + Sync  {
        fn get_meta_data(&self) -> &SpecMetaData;
    }

    impl Spec for Box<dyn ProtocolSpec>{
        fn get_meta_data(&self) -> &SpecMetaData {
            (**self).get_meta_data()
        }
    }

    pub trait SpecMapper{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError>;
    }

    impl SpecMapper for Box<dyn ProtocolSpec>{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) ->Result<(), ParserError>  {
            (**self).add_mapping_template(mapper)?;
            Ok(())
        }
    }


    pub trait SerializableSpec: Spec + SpecSerialize + SpecDeserialize + ToSpecType{}

    #[async_trait]
    impl SpecSerialize for Box<dyn ProtocolSpec>{
        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>{
            (**self).serialize(info_provider, mapper_context, writer).await
        }
    }

    #[async_trait]
    impl SpecDeserialize for Box<dyn ProtocolSpec>{
        async fn deserialize (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead), update_info: bool,
        ) -> Result<Value, ParserError>{
            (**self).deserialize(info_provider, reader, update_info).await
        }
    }

    impl ToSpecType for Box<dyn ProtocolSpec>{
        fn to_spec_type(&self) ->SpecType {
            (**self).to_spec_type()
        }
    }

    impl SpecTraverse for Box<dyn ProtocolSpec>{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError> {
            (**self).traverse(mapper)
        }
    }
   

    impl <T> SerializableSpec for T where T: Spec + SpecSerialize + SpecDeserialize + ToSpecType{}

    pub trait MappableSpec: Spec + SpecTraverse + SpecMapper + ToSpecType{}

    impl <T> MappableSpec for T where T: Spec + SpecTraverse + SpecMapper + ToSpecType{}

    

    pub trait ProtocolSpec: SerializableSpec + MappableSpec{        
    }

    impl <T> ProtocolSpec for T where T: SerializableSpec + MappableSpec{}

    #[allow(dead_code)]
    pub(crate) trait Anyway{
        fn end_current_spec(self, mapper_context: &mut MapperContext) -> Self;

        fn end_spec<S>(self, mapper_context: &mut MapperContext,  spec: &S) -> Self where S: ToSpecType + ?Sized;
    }

    impl <R> Anyway for Result<R, ParserError> 
    {
        fn end_current_spec(self, mapper_context: &mut MapperContext,  ) -> Self {
            mapper_context.end_current_spec();
            self
        }

        fn end_spec<S>(self, mapper_context: &mut MapperContext,  spec: &S) -> Self where S: ToSpecType + ?Sized{
            mapper_context.end_spec(spec)?;
            self
        }
    }

    #[derive(Clone, Debug)]
    pub struct MapperContext{
        types: Vec<SpecType>,
    }    

    impl MapperContext{
        pub fn new() -> MapperContext{
            Self { types: vec!() }
        }

        pub fn start_spec<S>(&mut self, spec: &S) where S: ToSpecType + ?Sized{
            debug!("starting spec {}", spec.to_spec_type().to_path_string());
            self.start_spec_type(spec.to_spec_type());
        }

        pub fn start_spec_type(&mut self, spec_type:SpecType){
            /* let x: Box<dyn ProtocolSpec> = Box::new(OneOfSpec::new(SpecName::NoName, false, vec!()));
            test_m1(x);  */
            self.types.push(spec_type);
        }

        pub fn end_current_spec(&mut self){
            self.types.pop();
        } 

        pub fn end_spec<S>(&mut self, in_spec: &S) -> Result<(), ParserError> where 
        S: ToSpecType + ?Sized{
            debug!("");
            debug!("trying to close spec {:?}, the total spec {:?}", in_spec.to_spec_type(), self.types);
            debug!("");
            if let Some(spec_type) = self.types.last(){
                if &in_spec.to_spec_type() == spec_type {
                    self.types.pop();
                    return Ok(());
                }else {
                    return Err(ParserError::InvalidMarker { 
                        line_index: 0, 
                        char_index: 0, 
                        message: format!("expected spec type {:?} to be removed, but found {:?}" , in_spec.to_spec_type(), spec_type),
                    });
                }
            }
            Ok(())
        }

        pub fn increment_current_repeat_spec(&mut self){
            let last = self.types.last_mut();
            if let Some( repeater) = last{
                match repeater{
                    
                    SpecType::RepeatMany(_, _repeat_count, current_index) => {
                        *current_index += 1;
                    },
                    _ =>{}
                }
            }
        }

       
        pub fn get_current_spec_path_template(&self) -> String{
            let mut spec_template = "$".to_owned();
            self.types.iter().for_each(|spec_type|{
                spec_template = format!("{}.{}", spec_template,spec_type.to_path_template_string());
                /* spec_template = match spec_type{
                    SpecType::Composite(name) => format!("{}.{}", spec_template,name.to_path_string()),
                    SpecType::RepeatMany(name,_repeat_count, _current_index) => format!("{}.{}.{{}}", spec_template,name.to_path_string()),                    
                    SpecType::Key(name) => format!("{}.{}", spec_template,name.to_path_string()),
                    SpecType::Value(name) => format!("{}.{}", spec_template,name.to_path_string()),
                    SpecType::Simple(name) => format!("{}.{}", spec_template,name.to_path_string()),
                } */
            });
            spec_template
        }

        pub fn get_current_spec_path(&self) -> String{
            let mut spec_path = "$".to_string();
            self.types.iter().for_each(|spec_type|{
                spec_path = format!("{}.{}", spec_path, spec_type.to_path_string())
            });
            spec_path
        }

        pub fn get_last_available_spec_name(&self) -> Option<String>{
            
            for spec_type in self.types.iter().rev(){
                match spec_type{
                    SpecType::Composite(name) 
                        | SpecType::Key(name) 
                        | SpecType::Value(name)
                        | SpecType::Simple(name)
                    => {
                        match name {
                            SpecName::Name(name) => {
                                return Some(name.to_owned());
                            },
                            _  =>  continue,
                        }
                    }
                    SpecType::RepeatMany(name, _, _current_index) => {
                        match name {
                            SpecName::Name(name) => {
                                return Some(name.to_owned());
                            },
                            _  =>  continue,
                        }
                    }
                }
            }
            None
            //panic!("at least one SpecName::Name is expected in the list");
        }
    }

    fn normalize_repeater(spec_name: &String, repeater_context: &RepeaterContext,) -> String{
        normalize_repeater_with_count(spec_name, repeater_context.get_count())        
    }

    fn normalize_repeater_with_count(spec_name: &String, count: u32) -> String{
        spec_name.replace("{}", count.to_string().as_str())
    }

    fn get_context_from_qualified_name(qualified_name:&str, lookup_name: &str)->String{
        qualified_name.replace(format!(".{}",  lookup_name).as_str(), "")
    }

    pub trait Mapper:  Send + Sync + Debug{

        fn get_value_by_key(&self, spec_name: &str) -> Option<&Value>{

            
            let value_path = self.get_mapping_data_template().get(spec_name);
            
            if let Some(value_path) = value_path{
                debug!("getting value for key {} -> {}", spec_name, value_path);
                self.get_spec_data().get(value_path)
            }else{
                None
            }
        }

        fn get_value_from_key_value_list(&self,key: String, spec_name: &str) -> Option<&Value>{
            let spec_path = self.get_mapping_data_template().get(spec_name);
            
            if let Some(spec_path) = spec_path{
                debug!("getting value for key {} -> {}", spec_name, spec_path);
                let key_quick_lookup_name = format!("{}.{}", spec_path, key);
                let value_path = self.get_mapping_data_template().get(&key_quick_lookup_name);
                if let Some(value_path) = value_path{
                    self.get_spec_data().get(value_path)
                }else{
                    return None;
                }
            }else{
                None
            }
        }

        fn get_key_value_map(&self, key_lookup_name: &String)-> Result<HashMap<String, &Value>, ParserError>{
            //let context = self.get_context_from_lookup_name(&key_lookup_name);
            //let qualified_name = self.get_mapping_data_template().get(key_lookup_name);
            let qualified_name = self.get_qualified_name(&key_lookup_name)?;
            let mut index = 0u32;
            let mut values = HashMap::<String, &Value>::new();
            loop{
                let key_spec_name = normalize_repeater_with_count(&qualified_name, index);
                //let value_spec_name =  self.get_mapping_data().get(&key_spec_name);

                let key =  self.get_spec_data().get(&key_spec_name);
                if key.is_none() {
                    return Ok(values);
                }

                let key = key.unwrap();
                    //.ok_or(ParserError::MissingData(format!("spec key data missing for {}", &key_spec_name)))?;
                
                let value_spec_name = self.get_mapping_data().get(&key_spec_name)
                    .ok_or(ParserError::MissingData(format!("value spec mssing for spec key {}", &key_spec_name)))?;
                let value = self.get_spec_data().get(value_spec_name)
                    .ok_or(ParserError::MissingData(format!("value spec data mssing for spec key {}", &value_spec_name)))?;
                values.insert(key.get_string_value_unchecked().unwrap(), value);
                index+=1;
            }
            
        }

        fn get_context_from_lookup_name(&self, lookup_name: &str)-> Result<String, ParserError>{
            let qualified_name = self.get_qualified_name(lookup_name)?;
            Ok(get_context_from_qualified_name(qualified_name.as_str(),  lookup_name))
            
        }

        fn get_qualified_name(&self, lookup_name: &str)-> Result<String, ParserError>{
            let qualified_name = self.get_mapping_data_template().get(lookup_name)
                .ok_or(ParserError::MissingData(format!("qualified lookup name missing in spec template for {lookup_name}")))?;
            Ok(qualified_name.clone())
            
        }

        fn add_simple_data(&mut self, key: String, value: Value) -> Result<(), ParserError>{
            
            if let Some(template) = self.get_mapping_data_template().get(&key).map(|element| element.to_owned()) {
                debug!("adding value for key {} -> {}", key, template);
                self.get_spec_data_mut().insert(template, value);
                return Ok(())
            }else{
                return Err(ParserError::MissingKey(format!("template lookup failed for key {}", key)));
            } 
        }

        fn add_to_key_value_list(&mut self, key: String, value: Value, key_lookup_name: String, value_lookup_name: String) -> Result<(), ParserError>{
            
            let key_spec_name = self.get_qualified_name(&key_lookup_name)?;
            let value_spec_name = self.get_qualified_name(&value_lookup_name)?;
            
            let context_name= get_context_from_qualified_name(key_spec_name.as_str(),  key_lookup_name.as_str());
            
            let repeater_context = self.get_repeater_context_mut(context_name.to_owned());
            let normalized_key_spec_name = normalize_repeater(&key_spec_name, repeater_context);
            let value_spec_name = normalize_repeater(&value_spec_name, repeater_context);                        
            repeater_context.next();

            // Map key spec name to value spec name e.g headers.0.HeaderName -> headers.0.HeaderValue
            self.get_mapping_data_mut().insert(normalized_key_spec_name.clone(), value_spec_name.clone());

            // Map key spec name to the actual key string headers.0.HeaderName -> Content-Type
            self.get_spec_data_mut().insert(normalized_key_spec_name.clone(), Value::String(key.clone()));

            // Map value spec name to the actual  value headers.0.HeaderValue -> application/json
            self.get_spec_data_mut().insert(value_spec_name.clone(), value);

            let key_quick_lookup_name = format!("{}.{}", key_spec_name, key);


            debug!("adding value for  {} -> {}", key_quick_lookup_name, value_spec_name);

            // Map key to value spec name for quick lookup of value e.g. headers.0.HeaderName.Content-Type -> headers.0.HeaderValue
            self.get_mapping_data_template_mut().insert(key_quick_lookup_name, value_spec_name);
            
            

            Ok(())
        }

        


        fn add_mapping_data(&mut self, proto_name: String, spec_name: String) {
            self.get_mapping_data_mut().insert(proto_name, spec_name);

        }

        fn add_mapping_template(&mut self, proto_name: String, spec_name: String) {
            debug!("adding template for name {} -> {}", proto_name, spec_name);
            self.get_mapping_data_template_mut().insert(proto_name, spec_name.clone());
        }

        fn setup_mapping_data(&mut self){
        }

        

        fn get_mapping_data_mut(&mut self) -> &mut HashMap<String, String>;

        fn get_mapping_data_template_mut(&mut self) -> &mut HashMap<String, String>;

        fn get_mapping_data_template(&self) -> & HashMap<String, String>;

        fn get_mapping_data(&self) -> &HashMap<String, String>;

        fn get_spec_data_mut(&mut self) -> &mut HashMap<String, Value>;

        fn get_spec_data(&self) -> &HashMap<String, Value>;

        fn get_repeater_context_mut(&mut self, context_name: String) -> &mut RepeaterContext{
            let context_map = self.get_repeater_context_map_mut();
            context_map.entry(context_name).or_insert(RepeaterContext::new())
        }

        fn get_repeater_context_map_mut(&mut self) -> &mut HashMap<String, RepeaterContext>;

        fn get_mapper_context_mut(&mut self) -> &mut MapperContext;

        fn get_mapper_context(&self) -> &MapperContext;
    }

    impl DelimitedStringSpec{
        /* fn new(name: SpecName, delimiter: Separator,  optional: bool) -> Self {
            DelimitedStringSpec {                
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional),
                until: delimiter,
            }
        } */
    }

    async fn parse_delimited_string_spec<D:DelimitedSpec>(spec: &D, reader: &mut dyn SpecRead,) -> Result<Value, ParserError>{
        let value = match spec.get_delimiter() {
                Separator::Delimiter(ref delimiter) => {
                    reader.read_placeholder_until(delimiter.to_owned()).await?
                }
                Separator::NBytes(size) => {
                    reader.read_bytes( ReadBytesSize::Fixed(*size)).await?
                }
                Separator::EndOfStream => {
                    reader.read_bytes(ReadBytesSize::Full).await?
                }
            };

            if let Some(value) = value {
                return Ok(ValueType::parse(&spec.get_meta_data().value_type, &value));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read value for placeholder: {:?}",
                    spec.get_meta_data().name.to_name_string()
                )))
            }
    }

    #[async_trait]
    impl SpecDeserialize for DelimitedStringSpec
    {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut dyn SpecRead, update_info: bool,
        ) -> Result<Value, ParserError>      
        {
            //let mut buf = vec![];
             let value = parse_delimited_string_spec(self, reader).await?;
             if update_info{
                if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name() {
                    info_provider.add_info(spec_name, value.clone())?;
                }
                return Ok(Value::None);
             }
             Ok(value)
        }
    }

    #[async_trait]
    impl SpecSerialize for DelimitedStringSpec
    {
        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>
        {
            //mapper_context.start_spec(self);
            let name = self.get_meta_data().get_name();            
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name.to_name_string(), value, self.get_meta_data().is_optional(), writer).await?;
            if let Separator::Delimiter(delimiter) = &self.until{
                writer.write(delimiter.as_bytes()).await?;
            }

            if let Separator::NBytes(delimiter) = &self.until{
                writer.write_data_u32(*delimiter).await?;
            }
            Ok(())
        }
    }

    pub struct ExactStringSpec{
        pub input: String,
        pub spec_meta_data: SpecMetaData,
    }

    //impl SimpleValueSpec for ExactStringSpec{}

    impl StringSpec for ExactStringSpec {}

    impl StringSpec for OneOfSpec {}

    impl ExactStringSpec{
        fn new(name: SpecName, input: String, optional: bool) -> Self {
            ExactStringSpec {
                input,
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional),
            }
        }
    }

    impl  Spec for ExactStringSpec {
        
        fn get_meta_data(&self)-> &SpecMetaData{
            &self.spec_meta_data
        }
    }

    #[async_trait]
    impl SpecDeserialize for ExactStringSpec{
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut dyn SpecRead, update_info: bool,
        ) -> Result<Value, ParserError>
        {
            let value = reader.read_placeholder_as_string(self.input.clone()).await?;
            if let Some(value) = value {
                if update_info && !self.spec_meta_data.get_name().is_delimiter() {

                    if let Some(name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                       info_provider.add_info(name, ValueType::parse(&self.get_meta_data().value_type, &value))?;
                    }
                }
                return Ok(ValueType::parse(&self.get_meta_data().value_type, &value));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read exact string for placeholder: {:?}",
                    self.get_meta_data().get_name().to_name_string()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecSerialize for ExactStringSpec
    {
        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>
        {   
            //mapper_context.start_spec(self);

            let name = self.get_meta_data().get_name().to_name_string();
            if let SpecName::Delimiter = self.get_meta_data().get_name() {
                write_data(name, Some(&Value::String(self.input.to_owned())), 
                    self.get_meta_data().is_optional(), 
                    writer).await?;
            }else{
                let value = info_provider.get_info(&mapper_context.get_current_spec_path());
                write_data(name, value, self.get_meta_data().is_optional(), writer).await?;
            }
            
            
            

            Ok(())
        }
    }

    pub(crate) struct KeyValueSpec{
        pub spec_metadata: SpecMetaData,
        pub key: Key,
        pub value: ValueSpec,
    }
    
    impl KeyValueSpec{
        pub fn new(key: Key, value: ValueSpec, spec_metadata: SpecMetaData) -> Self {
            KeyValueSpec {
                spec_metadata,
                key,
                value,
            }
        }
    }

    pub(crate) fn extract_name_and_spec_path<F, S> (
        spec_path_finder: F,
        mapper: &mut Box<dyn Mapper>, spec: &S, inner_spec: &Box<dyn ProtocolSpec> ) -> Result<(Option<String>, Option<String>), ParserError>
        where F: Fn(&Box<dyn Mapper>) -> String,
        S: ProtocolSpec,
        {
        mapper.get_mapper_context_mut().start_spec_type(spec.to_spec_type());
        mapper.get_mapper_context_mut().start_spec_type(inner_spec.to_spec_type());
        let spec_name = mapper.get_mapper_context().get_last_available_spec_name();
        let spec_path = Some(
                spec_path_finder(mapper)                
            );
        mapper.get_mapper_context_mut().end_spec(inner_spec)?;
        mapper.get_mapper_context_mut().end_spec(spec)?;
        return Ok((spec_name, spec_path))
    }


    impl Spec for KeyValueSpec{
        fn get_meta_data(&self) -> &SpecMetaData{
            &self.spec_metadata
        }
    }



    #[async_trait]
    impl SpecSerialize for KeyValueSpec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            //mapper_context.start_spec(self);
            serialize(&self.key, info_provider, writer, mapper_context).await?;
            //self.key.serialize(info_provider, mapper_context, writer).await?;

            serialize(&self.value, info_provider, writer, mapper_context).await?;
            //mapper_context.start_spec(self);
            //self.value.serialize(info_provider, mapper_context, writer).await?;


            //self.key.serialize(info_provider, writer).await?;
            Ok(())
        }
    }

    #[async_trait]
    impl SpecDeserialize for KeyValueSpec{
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool,
            
        ) -> Result<Value, ParserError>
        {
            
            
            let path_finder =  |mapper:  &Box<dyn Mapper>| {mapper.get_mapper_context().get_current_spec_path()};
            
            
            let ( key_spec_name,  key_spec_path,) = extract_name_and_spec_path(path_finder, info_provider.get_mapper_mut(), &self.key, &self.key.0)?;
            

            let key_name = undoable_deserialize(&self.key, info_provider, reader, false).await?;

            
            let ( value_spec_name,  value_spec_path,) = extract_name_and_spec_path(path_finder,info_provider.get_mapper_mut(), &self.value, &self.value.0)?;
            

            let value = undoable_deserialize(&self.value, info_provider, reader, false).await?;
            
            match (key_spec_path, value_spec_path){
                (None, None) => {},
                (None, Some(ref value_spec_path)) => {
                    info_provider.get_mapper_mut().get_spec_data_mut().insert(value_spec_path.clone(), value);
                },
                (Some(ref key_spec_path), None) => {
                    info_provider.get_mapper_mut().get_spec_data_mut().insert(key_spec_path.clone(), key_name);
                },
                (Some(ref _key_spec_path), Some(ref _value_spec_path)) => {
                    if update_info{
                        info_provider.get_mapper_mut().add_to_key_value_list(key_name.get_string_value_unchecked().unwrap(),
                            value, key_spec_name.unwrap(), value_spec_name.unwrap())?;
                    }
                },
            }
            return Ok(Value::None);            
        }
    }

    pub struct NBytesSpec{
        spec_meta_data: SpecMetaData,
        size: u32,
    }

    impl  NBytesSpec{
        pub fn new(name: SpecName, size: u32, optional: bool) -> Self {
            NBytesSpec {
                spec_meta_data: SpecMetaData::new(name, ValueType::U8Vec, optional),
                size,
            }
        }
    }

    impl SimpleValueSpec for NBytesSpec{}

    impl Spec for NBytesSpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    #[async_trait]
    impl SpecDeserialize for NBytesSpec{
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool,
        ) -> Result<Value, ParserError>
        {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(self.size)).await?;
            if let Some(bytes) = bytes {
                if update_info {
                    if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                        info_provider.add_info(spec_name, Value::U8Vec(bytes.clone()))?;
                    }
                }
                return Ok(ValueType::parse(&self.get_meta_data().value_type, &bytes));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read {} bytes for placeholder: {:?}",
                    self.size, self.get_meta_data().get_name().to_name_string()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecSerialize for NBytesSpec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            
            //mapper_context.start_spec(self);
            let name = self.get_meta_data().get_name().to_name_string();
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name, value, self.get_meta_data().is_optional(), writer).await?;
            Ok(())
        }
    }

    

    pub struct AllBytesSpec{     
        spec_meta_data: SpecMetaData,           
    }

    impl Spec for AllBytesSpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    impl SimpleValueSpec for AllBytesSpec{}

    #[async_trait]
    impl SpecDeserialize for AllBytesSpec{    
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool,   
        ) -> Result<Value, ParserError>
        {
            let bytes = reader.read_bytes(ReadBytesSize::Full).await?;
            if let Some(bytes) = bytes {
                if update_info{
                    if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                        info_provider.add_info(spec_name, Value::U8Vec(bytes.clone()))?;
                    }
                    return Ok(Value::None)
                }
                return Ok(ValueType::parse(&self.get_meta_data().get_value_type(), &bytes));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read {} bytes for placeholder: {:?}",
                    "remaining ", self.get_meta_data().name.to_name_string()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecSerialize for AllBytesSpec{
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.get_meta_data().get_name().to_name_string();                        
            //mapper_context.start_spec(self);
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name, value, self.get_meta_data().is_optional(), writer).await?;
            Ok(())
        }
    }

    #[derive(Default)]
    pub struct OneOfSpec{
        spec_meta_data: SpecMetaData,
        values: Vec<String>,        
        until: Separator,
    }

    

    impl DelimitedSpec for OneOfSpec{
        fn set_delimiter(&mut self, delimiter: Separator)  {
            self.until = delimiter;
        }
        
        fn get_delimiter(& self) -> &Separator {
            &self.until
        }
    }

    impl Spec for OneOfSpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    impl OneOfSpec{
        pub fn new(name: SpecName, optional: bool, values: Vec<String>) -> Self {
            OneOfSpec {
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional),
                values,
                until: Separator::EndOfStream,
            }
        }

        pub fn add_value(&mut self, value: String) {
            self.values.push(value);
        }

        pub fn set_delimiter(&mut self, delimiter: Separator) {
            self.until = delimiter;
        }

        pub fn get_values(&self) -> &Vec<String> {
            &self.values
        }
    }

    #[async_trait]
    impl SpecDeserialize for OneOfSpec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool,
        ) -> Result<Value, ParserError>
        {
            let result = parse_delimited_string_spec(self, reader).await?;
            
                //.undoable_parse(info_provider, reader).await?;
            if let Some(value) = &result.get_string_value() {
                if self.values.contains(value) {
                    if update_info{
                        if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                            info_provider.add_info(spec_name, Value::String(value.clone()))?;
                        }
                        return Ok(Value::None);
                    }
                    
                return Ok(result);
                } else {
                    return Err(ParserError::MissingValue(format!(
                        "Expected one of {:?}, but got: {:?}",
                        self.values, value
                    )));
                }
            } else {
                Err(ParserError::MissingValue(format!(
                    "Expected one of {:?}, but got: {:?}",
                    self.values, result
                )))
            }
        }
    }


    #[async_trait]
    impl SpecSerialize for OneOfSpec{
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.get_meta_data().get_name().to_name_string();                        
            //mapper_context.start_spec(self);
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name, value, self.get_meta_data().is_optional(), writer).await?;
            if let Separator::Delimiter(delimiter) = &self.until{
                writer.write(delimiter.as_bytes()).await?;
            }
            Ok(())
        }
    }

    #[derive(Default)]
    pub struct ListSpec{            
        spec_meta_data: SpecMetaData,
        pub constituents: Vec<Box< (dyn ProtocolSpec)>>,
    }

    

    #[async_trait]
    impl SpecDeserialize for ListSpec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool,            
        ) -> Result<Value, ParserError>
        {
            let mut has_one_success = false;
            for constituent in &self.constituents {   
                let result = undoable_deserialize(constituent, info_provider, reader, update_info).await;
                debug!("deserializing {}", constituent.get_meta_data().get_name());
                match result{
                    Ok(_) => {
                        has_one_success = true;
                        continue;
                    },
                    Err(ref e) => {
                        debug!("{} is optional? {}, {}", constituent.get_meta_data().get_name(), constituent.get_meta_data().is_optional(),e);
                        has_one_success = has_one_success | false;
                        if constituent.get_meta_data().is_optional() {
                            continue;
                        }else{
                            return result;
                        }
                    },
                }
            }

            if !has_one_success {
                return Err(ParserError::NoValidListConstituents(self.get_meta_data().get_name().to_path_string()));
            }
            Ok(Value::None) // or some other appropriate return value
        }
    }
   
    #[async_trait]
    impl SpecSerialize for ListSpec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            //mapper_context.start_spec(self);
            for constituent in &self.constituents {                
                //mapper_context.start_spec(constituent);
                let result = serialize(constituent, info_provider, writer, mapper_context).await;

                if result.is_err(){
                    warn!("error when serializing spec: {} optional: is {} error:{:?}",
                     constituent.get_meta_data().get_name().to_path_string(), constituent.get_meta_data().is_optional(), result);                    
                }
                
                if result.is_err() && !constituent.get_meta_data().is_optional() {
                    //mapper_context.end_spec(self)?;
                    return result;
                }
            };
            //mapper_context.end_spec(self)?;
            Ok(()) // or some other appropriate return value
        }
    }

    impl ListSpec {
        pub fn new(name: SpecName, value_type: ValueType, optional: bool) -> Self {
            ListSpec {
                spec_meta_data: SpecMetaData::new(name, value_type, optional),
                constituents: Vec::new(),
            }
        }

        pub fn add_spec(&mut self, constituent: Box<dyn ProtocolSpec> ) {
            self.constituents.push(constituent);
        }
    }

    impl Spec for ListSpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }
    

    #[derive(Default)]
    pub(crate) struct ValueSpec(pub Box<dyn ProtocolSpec>, pub SpecMetaData);

    impl Default for Box<dyn Spec> {
        fn default() -> Self {
            Box::new(DelimitedStringSpec::default())
        }
    }

    impl Default for Box<dyn ProtocolSpec> {
        fn default() -> Self {
            Box::new(DelimitedStringSpec::default())
        }
    }

    impl Spec for ValueSpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.1
        }
    }

    pub(crate) struct InlineKeyWithValue(pub Box<dyn ProtocolSpec>, /* pub String, */ pub SpecMetaData);
    
    impl Spec for InlineKeyWithValue {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.1
        }
    }
    
    #[derive(Default)]
    pub(crate) struct Key(pub Box<dyn ProtocolSpec>, pub SpecMetaData) ;

    impl Spec for Key {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.1
        }
    }

    

    #[async_trait]
    impl SpecSerialize for Key {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            //mapper_context.start_spec(self);
            //let name = self.1.get_name().to_name_string();
            //let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            //write_data(name, value, self.1.optional, writer).await.anyway(mapper_context)?;
            serialize(&self.0, info_provider,  writer, mapper_context).await?;            
            Ok(())
            
        }
    }

    impl Default for Box<dyn StringSpec> {
        fn default() -> Self {
            Box::new(DelimitedStringSpec::default())
        }
    }

    #[async_trait]
    impl SpecDeserialize for Key

    {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool,         
        ) -> Result<Value, ParserError>
        {
            undoable_deserialize(&self.0, info_provider, reader,update_info).await
        }
    }

    #[async_trait]
    impl SpecDeserialize for InlineKeyWithValue
    {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool,            
        ) -> Result<Value, ParserError>
        {
            undoable_deserialize(&self.0, info_provider, reader, update_info).await.map(|value| {
                if update_info{
                    //let spec_name = info_provider.get_mapper_context().get_last_available_spec_name();
                    //info_provider.add_info(spec_name, value.clone());
                    return Value::None;
                }else {
                    return value;
                }
            })
        }
    }

    #[async_trait]
    impl SpecDeserialize for ValueSpec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool,           
        ) -> Result<Value, ParserError>
        {
            undoable_deserialize(&self.0, info_provider, reader, update_info).await
        }
    }

    #[async_trait]
    impl SpecSerialize for ValueSpec {
        async fn serialize(
            &self,
            info_provider: & (dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            //mapper_context.start_spec(self);
            serialize(&self.0, info_provider, writer, mapper_context).await?;//.end_spec(mapper_context, self)?;
            Ok(())
        }
    }

    async fn write_data(name: String, value:Option<&Value>, optional:bool, writer: &mut (dyn SpecWrite)) -> Result<(), ParserError>{
        if let Some(value) = value{
                write(value, writer).await?;
                Ok(())
            }else if !optional {
                return Err(ParserError::MissingData(name));
            }else{
                Ok(())
            }
    }

    #[derive(Default)]
    pub struct NumberU64Spec(SpecMetaData) ;

    #[derive(Default)]
    pub struct NumberI64Spec(SpecMetaData) ;

    #[derive(Default)]
    pub struct NumberU32Spec(SpecMetaData) ;

    #[derive(Default)]
    pub struct NumberU16Spec(SpecMetaData) ;

    pub(crate) trait NumberSpec: SimpleValueSpec + Send + Sync{}

    impl <S> ToSpecType for S where S:SimpleValueSpec{
        /* fn to_spec_type(&self) ->SpecType {
            let spec_name = self.get_meta_data().get_name();
            match spec_name{
                SpecName::Name(name) |
                SpecName::Transient(name) =>
                    SpecType::Simple(name.to_owned()),
    
                SpecName::NoName => SpecType::Simple("Default".to_owned())
            }
        } */
    }

    impl SimpleValueSpec for NumberU64Spec{}
    impl SimpleValueSpec for NumberU32Spec{}
    impl SimpleValueSpec for NumberU16Spec{}
    impl SimpleValueSpec for NumberI16Spec{}
    impl SimpleValueSpec for NumberI64Spec{}


    impl NumberSpec for NumberU64Spec{}
    impl NumberSpec for NumberU32Spec{}
    impl NumberSpec for NumberU16Spec{}
    impl NumberSpec for NumberI16Spec{}
    impl NumberSpec for NumberI64Spec{}

    impl Spec for NumberU64Spec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.0
        }
    }

    impl Spec for NumberU16Spec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.0
        }
    }

    impl Spec for NumberU32Spec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.0
        }
    }

    
    impl Spec for NumberI16Spec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.0
        }
    }

    impl Spec for NumberI64Spec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.0
        }
    }

    
    #[derive(Default)]
    pub struct NumberI16Spec(SpecMetaData);
    
    
    #[async_trait]
    impl SpecDeserialize for NumberU64Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool,
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(8)).await?;
            if let Some(bytes) = bytes {
                if update_info{
                    if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                        info_provider.add_info(spec_name, ValueType::parse(&ValueType::UnSignedNumber64, &bytes))?;
                    }
                    return Ok(Value::None);
                }else {
                    Ok(ValueType::parse(&ValueType::UnSignedNumber64, &bytes))
                }
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name().to_name_string()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecDeserialize for NumberI64Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(8)).await?;
            if let Some(bytes) = bytes {
                if update_info{
                    if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                        info_provider.add_info(spec_name, ValueType::parse(&ValueType::SignedNumber64, &bytes))?;
                    }
                    return Ok(Value::None);
                }else {
                    Ok(ValueType::parse(&ValueType::SignedNumber64, &bytes))
                }
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name().to_name_string()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecDeserialize for NumberU32Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4)).await?;
            if let Some(bytes) = bytes {
                if update_info{
                    if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                        info_provider.add_info(spec_name, ValueType::parse(&ValueType::UnSignedNumber32, &bytes))?;
                    }
                    return Ok(Value::None);
                }else {
                    Ok(ValueType::parse(&ValueType::UnSignedNumber32, &bytes))
                }
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name().to_name_string()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecDeserialize for NumberU16Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4)).await?;
            if let Some(bytes) = bytes {
                if update_info{
                    if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                        info_provider.add_info(spec_name, ValueType::parse(&ValueType::UnSignedNumber16, &bytes))?;
                    }
                    return Ok(Value::None);
                }else {
                    Ok(ValueType::parse(&ValueType::UnSignedNumber16, &bytes))
                }
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name().to_name_string()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecDeserialize for NumberI16Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead), update_info: bool
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4)).await?;
            if let Some(bytes) = bytes {
                if update_info{
                    if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                        info_provider.add_info(spec_name, ValueType::parse(&ValueType::SignedNumber16, &bytes))?;
                    }
                    return Ok(Value::None);
                }else {
                    Ok(ValueType::parse(&ValueType::SignedNumber16, &bytes))
                }
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name().to_name_string()
                )))
            }
        }
    }


    /* #[async_trait]
    impl SpecSerialize for dyn Spec {


        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name();
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.0.optional, writer).await?;
            Ok(())
        }
    } */

   #[async_trait]
    impl <T> SpecSerialize for T where T: NumberSpec + ToSpecType {


        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.get_meta_data().get_name().to_name_string();
            //mapper_context.start_spec(self);
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());            
            write_data(name, value, self.get_meta_data().optional, writer).await?;//.end_spec(mapper_context, self)?;
            Ok(())
        }
    }

    /*  #[async_trait]
    impl SpecSerialize for NumberU16Spec {


        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name().to_name_string();
            let value = info_provider.get_info(&name);
            mapper_context.start_spec(self);
            write_data(name, value, self.0.optional, writer).await.anyway(mapper_context)?;
            Ok(())
        }
    } */
/*
    #[async_trait]
    impl SpecSerialize for NumberU32Spec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name().to_name_string();
            let value = info_provider.get_info(&name);
            write_data(name, value, self.0.optional, writer).await?;
            Ok(())
        }
    }

    #[async_trait]
    impl SpecSerialize for NumberU64Spec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name().to_name_string();
            let value = info_provider.get_info(&name);
            write_data(name, value, self.0.optional, writer).await?;
            Ok(())
        }
    }

    #[async_trait]
    impl SpecSerialize for NumberI64Spec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name().to_name_string();
            let value = info_provider.get_info(&name);
            write_data(name, value, self.0.optional, writer).await?;
            Ok(())
        }
    }

    #[async_trait]
    impl SpecSerialize for NumberI16Spec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name().to_name_string();
            let value = info_provider.get_info(&name);
            write_data(name, value, self.0.optional, writer).await?;
            Ok(())
        }
    } */

    
    pub trait BuilderState:Default{}

    pub trait BuildGenericString:BuilderState{}

    // Builder States

    #[derive(Default)]
    pub struct BuildFromScratch{}

    #[derive(Default)]
    pub struct BuildKey{
        key_spec_metadata: SpecMetaData,
    }

    #[derive(Default)]
    pub struct BuildKeyAvailable{
        key: Key,
    }

    #[derive(Default)]
    pub struct BuildValue{
        key: Key,
        value_spec_metadata: SpecMetaData,
    }

    #[derive(Default)]
    pub struct BuildInlineValue{        
        value_spec_metadata: SpecMetaData,
    }
    
    #[derive(Default)]
    pub struct BuildDelimiter<D:DelimitedSpec, B:BuilderState>{
        delimiter_spec: D,
        parent_builder_state: B,
    }

    //Builder State implementations

    impl  BuilderState for BuildFromScratch{}
    impl  BuildGenericString for BuildFromScratch{}
    impl  BuilderState for BuildKey{}
    //impl  BuildKeyString for BuildKey{}
    impl  BuilderState for BuildValue{}
    impl  BuilderState for BuildKeyAvailable{}
    impl  BuilderState for BuildInlineValue{}

    impl <D, B> BuilderState for BuildDelimiter<D, B> where D: DelimitedSpec, B:BuilderState{}

    
    //Proto Spec Builder trait
    pub trait ProtoSpecBuilder<S:BuilderState>: Default{
        fn build(self) -> ListSpec;
        fn add_spec(&mut self, spec: Box<dyn ProtocolSpec>);
        fn replace_current_state_with_default(&mut self)-> S;
        fn get_state(&mut self)-> &mut S;
        fn set_state(&mut self, s: S);
        fn set_spec(&mut self, composite_spec:ListSpec);
        fn wrap_with_data<D>(self, data:D)->BuilderWrapperWithData<Self, D, S>;
        fn wrap(self) -> BuilderWrapper<Self, S>;
        fn replace_current_spec_with_default(&mut self) -> ListSpec;
    }

    // Struct implementing Spec Builder 
    #[derive(Default)]
    pub struct ProtoSpecBuilderData<S:BuilderState>{
        composite_spec: ListSpec,
        state: S,
    }

    impl <S> ProtoSpecBuilder<S> for ProtoSpecBuilderData<S> where S:BuilderState {
        fn build(self) -> ListSpec {
            self.composite_spec
        }

        fn add_spec(&mut self, spec: Box<dyn ProtocolSpec>) {
            self.composite_spec.add_spec(spec);
        }
        
        fn replace_current_state_with_default(&mut self)-> S {
            mem::take(&mut self.state)   
        }
        
        fn set_spec(&mut self, composite_spec:ListSpec) {
            self.composite_spec = composite_spec;
        }

        fn replace_current_spec_with_default(&mut self) -> ListSpec{
            mem::take(&mut self.composite_spec)
        }
        
        fn get_state(&mut self)-> &mut S {
            &mut self.state
        }
        
        fn set_state(&mut self, s: S) {
            self.state = s;
        }

        fn wrap_with_data<D>(self, data:D)->BuilderWrapperWithData<Self, D, S>{
            BuilderWrapperWithData(self, data, PhantomData::default())
        }

        fn wrap(self) -> BuilderWrapper<Self, S>{
            BuilderWrapper(self, PhantomData::default())
        }
    }

    pub fn new_spec_builder(name: SpecName)-> ProtoSpecBuilderData<BuildFromScratch>{
        ProtoSpecBuilderData::<BuildFromScratch>::new_with(name, true)
    }

    pub fn new_mandatory_spec_builder(name: SpecName)-> ProtoSpecBuilderData<BuildFromScratch>{
        ProtoSpecBuilderData::<BuildFromScratch>::new_with(name, false)
    }

    impl <S> ProtoSpecBuilderData<S> where S:BuilderState {
        pub fn new_with_state(state: S, name: SpecName, optional: bool) -> Self {
            ProtoSpecBuilderData {
                composite_spec: ListSpec { 
                    spec_meta_data: {
                        SpecMetaData::new(name, ValueType::None, optional)
                    },
                    constituents: Vec::new() 
                },
                state,
            }
        }

        pub fn new() -> Self {
            ProtoSpecBuilderData::new_with_state(S::default(), SpecName::Name("Default".to_owned()), true)
        }        

        pub fn new_with(name: SpecName, optional: bool) -> Self {
            let result = ProtoSpecBuilderData::new_with_state(S::default(), name, optional);
            result
        }

        pub fn new_from_scratch(name: SpecName, optional: bool) -> ProtoSpecBuilderData<BuildFromScratch> {
            ProtoSpecBuilderData::new_with_state(BuildFromScratch::default(), name, optional)
        }

    }
    
    //Generators

    pub trait NumberSpecGenerator {
        fn get_u16_spec(&self, name: SpecName, optional: bool) -> NumberU16Spec{
            NumberU16Spec(SpecMetaData::new(name, ValueType::UnSignedNumber16, optional))       
        }
        fn get_u32_spec(&self, name: SpecName, optional: bool) -> NumberU32Spec{
            NumberU32Spec(SpecMetaData::new(name, ValueType::UnSignedNumber32, optional))       
        }
        fn get_u64_spec(&self, name: SpecName, optional: bool) -> NumberU64Spec{
            NumberU64Spec(SpecMetaData::new(name, ValueType::UnSignedNumber64, optional))       
        }
        fn get_i16_spec(&self, name: SpecName, optional: bool) -> NumberI16Spec{
            NumberI16Spec(SpecMetaData::new(name, ValueType::SignedNumber16, optional))       
        }
        fn get_i64_spec(&self, name: SpecName, optional: bool) -> NumberI64Spec{
            NumberI64Spec(SpecMetaData::new(name, ValueType::SignedNumber64, optional))
        }
    }

    pub trait StringSpecGenerator{
        fn get_string_spec(&self, name: SpecName, optional: bool) -> DelimitedStringSpec where  Self:Sized{
            DelimitedStringSpec { 
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional), 
                until: Separator::EndOfStream 
            }
        }
    }

    pub trait KeySpecGenerator{        
    }
    
    impl <S> StringSpecGenerator for ProtoSpecBuilderData<S> where S:BuilderState{}

    impl KeySpecGenerator for ProtoSpecBuilderData<BuildFromScratch>{}
    
    impl <IBS> NumberSpecGenerator for ProtoSpecBuilderData<IBS> where IBS: BuilderState + 'static{}

    impl NumberSpecBuilder <BuildValue, BuildFromScratch, ProtoSpecBuilderData<BuildFromScratch>> 
    for ProtoSpecBuilderData<BuildValue>{}

    impl NumberSpecBuilder <BuildFromScratch, BuildKeyAvailable, ProtoSpecBuilderData<BuildKeyAvailable>> 
    for ProtoSpecBuilderData<BuildFromScratch>{}

    impl  NumberSpecBuilder<BuildInlineValue, BuildFromScratch, ProtoSpecBuilderData<BuildFromScratch>>
    for ProtoSpecBuilderData<BuildInlineValue>{}
    
    pub trait CustomSpecBuilder<IBS>: ProtoSpecBuilder<IBS>
    where IBS: BuilderState + 'static,
    {
        fn use_spec(mut self, spec: Box<dyn ProtocolSpec>) -> Self{
            self.add_spec(spec);
            self
        }
    }

    impl CustomSpecBuilder<BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch>{
    }

    pub trait NumberSpecBuilder <IBS,OBS, OB> :NumberSpecGenerator + ProtoSpecBuilder<IBS>
    where 
        Self: Sized + ProtoSpecBuilder<IBS> + 'static, 
        OB: ProtoSpecBuilder<OBS> + 'static,
        OBS: BuilderState + 'static, 
        IBS: BuilderState + 'static,        
        {
            
        fn expect_u16(self, name: SpecName, optional: bool ) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberU16Spec, IBS>> + 'static,            
        {        
            let spec = self.get_u16_spec(name, optional);
            self.wrap_with_data(spec).into()
        }

        fn expect_u32(self, name: SpecName, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberU32Spec, IBS>> + 'static,            
        {
            let spec = self.get_u32_spec(name, optional);
            self.wrap_with_data(spec).into()
        }

        fn expect_u64(self, name: SpecName, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberU64Spec, IBS>> + 'static,            
        {
            let spec = self.get_u64_spec(name, optional);
            self.wrap_with_data(spec).into()
        }

        fn expect_i16(self, name: SpecName, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberI16Spec, IBS>> + 'static,            
        {
            let spec = self.get_i16_spec(name, optional);
            self.wrap_with_data(spec).into() 
        }

        fn expect_i64(self, name: SpecName, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberI64Spec, IBS>> + 'static,            
        {
            let spec = self.get_i64_spec(name, optional);
            self.wrap_with_data(spec).into()
        }
    }

    pub trait InlineValueBuilder <IBS, OBS> :StringSpecGenerator + ProtoSpecBuilder<IBS>  
    where 
        Self: Sized + 'static,
        IBS: BuilderState + 'static,
        OBS:BuilderState + 'static, 
    {

        fn inline_value_follows(self, key_name: SpecName, optional: bool) ->  ProtoSpecBuilderData<OBS>//impl ProtoSpecBuilder<BuildDelimiter<DelimitedStringSpec, IBS>> 
        where                        
        ProtoSpecBuilderData<OBS>:ProtoSpecBuilder<BuildInlineValue> + From<BuilderWrapperWithData<Self, BuildInlineValue, IBS>>  + 'static,            

        {
            self.wrap_with_data(BuildInlineValue{                
                value_spec_metadata:SpecMetaData { name:  key_name, value_type: ValueType::None, optional: optional }
            }).into()
        }
    }
    
    impl From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildFromScratch>, BuildInlineValue, BuildFromScratch>> for ProtoSpecBuilderData<BuildInlineValue>{
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildFromScratch>, BuildInlineValue, BuildFromScratch>) -> Self {
            let from_builder = value.0;
            //let from_state = from_builder.replace_current_state_with_default();
            let inline_value_data  = value.1;
            let mut to_builder = ProtoSpecBuilderData::default();
            to_builder.set_state(inline_value_data);
            to_builder.set_spec(from_builder.composite_spec);
            to_builder
        }
    }

    pub trait ValueBuilder <IBS> : ProtoSpecBuilder<IBS>  
    where 
        Self: Sized + 'static,
        IBS: BuilderState + 'static,
        
    {

        fn value_follows(self, name: SpecName, optional: bool) ->  ProtoSpecBuilderData<BuildValue>
        where ProtoSpecBuilderData<BuildValue>: From<BuilderWrapperWithData<Self, SpecMetaData, IBS>>
        

        {
            self.wrap_with_data(SpecMetaData::new(name, ValueType::None, optional)).into()
        }
    }

    impl ValueBuilder<BuildKeyAvailable> for ProtoSpecBuilderData<BuildKeyAvailable>{}

    impl  InlineValueBuilder<BuildFromScratch, BuildInlineValue> for ProtoSpecBuilderData<BuildFromScratch>{}

    


    pub trait CompositeBuilder<IBS, OBS>: ProtoSpecBuilder<IBS>
    where 
        IBS: BuilderState + 'static,
        OBS: BuilderState + 'static,
        Self: Sized + 'static
    {

        fn expect_composite(self, spec: ListSpec) -> ProtoSpecBuilderData<OBS>
        where ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, ListSpec, IBS>>,
        {
            //self.add_spec(Box::new(spec));
            self.wrap_with_data(spec).into()
        }
    }

    impl CompositeBuilder<BuildFromScratch, BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch> {}

    pub trait RepeatBuilder<IBS, OBS>: ProtoSpecBuilder<IBS>
    where 
        IBS: BuilderState + 'static,
        OBS: BuilderState + 'static,
        Self: Sized + 'static
    {

        fn repeat_many(self, name: SpecName, optional: bool, separator: Separator, spec: ListSpec) -> ProtoSpecBuilderData<OBS>
        where ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, RepeatManySpec, IBS>>,
        {
            let repeat_spec = RepeatManySpec{
                spec_meta_data: SpecMetaData::new(name, ValueType::None, optional),
                constituents: spec,
                repeat_count: RepeatCount::Delimited(separator),
                
            };
            self.wrap_with_data(repeat_spec).into()
        }

        fn repeat_n_times(self, name: SpecName, optional: bool, number_of_times: u32, spec: ListSpec) -> ProtoSpecBuilderData<OBS>
        where ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, RepeatManySpec, IBS>>,
        {
            let repeat_spec = RepeatManySpec{
                spec_meta_data: SpecMetaData::new(name, ValueType::None, optional),
                constituents: spec,
                repeat_count: RepeatCount::Fixed(number_of_times),
            };
            self.wrap_with_data(repeat_spec).into()
        }
    }

    impl RepeatBuilder<BuildFromScratch, BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch>{}

    pub trait DelimitedStringSpecBuilder <IBS> :StringSpecGenerator + ProtoSpecBuilder<IBS>  
    where 
        Self: Sized + 'static,
        IBS: BuilderState + 'static,
    //    OBS:BuilderState + 'static, 
    {

        fn expect_string(self, name: SpecName, optional: bool) ->  ProtoSpecBuilderData<BuildDelimiter<DelimitedStringSpec, IBS>>  //impl ProtoSpecBuilder<BuildDelimiter<OneOfSpec, IBS>>
        where                        
        ProtoSpecBuilderData<BuildDelimiter<DelimitedStringSpec, IBS>>:From<BuilderWrapperWithData<Self, DelimitedStringSpec, IBS>> + 'static,
        
        {            
            
            let spec = self.get_string_spec(name, optional);
            self.wrap_with_data(spec).into()            
        }

        fn expect_one_of_string(self, name: SpecName, optional: bool, options: Vec<String>) ->  ProtoSpecBuilderData<BuildDelimiter<OneOfSpec, IBS>>  //impl ProtoSpecBuilder<BuildDelimiter<OneOfSpec, IBS>>
        where
        ProtoSpecBuilderData<BuildDelimiter<OneOfSpec, IBS>>:From<BuilderWrapperWithData<Self, OneOfSpec, IBS>> + 'static
        {
            //let name = name.unwrap_or("expect_one_of_string".to_string());
            let one_of_spec = OneOfSpec::new(name, optional, options);
            self.wrap_with_data(one_of_spec).into()            
        }

    }

    pub trait StringSpecBuilder <IBS, OBS> :StringSpecGenerator + ProtoSpecBuilder<IBS>  
    where 
        Self: Sized + 'static,
        IBS: BuilderState + 'static,
        OBS:BuilderState + 'static, 
    {


        fn expect_exact_string(self, name: SpecName, input: String, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where
            Self: Sized + 'static,
            OBS: BuilderState +  'static,
            ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, ExactStringSpec, IBS>> + 'static,            
        {
            let exact_string = ExactStringSpec::new(name, input, optional);
            self.wrap_with_data(exact_string).into()
        }

        fn expect_newline(self) -> ProtoSpecBuilderData<OBS> 
        where
            Self: Sized + 'static,
            //OB: ProtoSpecBuilder<OBS> + 'static,
            OBS: BuilderState +  'static,
            ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, ExactStringSpec, IBS>> + 'static,{
                self.expect_exact_string(SpecName::Delimiter, "\r\n".to_string(), false)
            }

        fn expect_space(self,) -> ProtoSpecBuilderData<OBS> 
        where
            Self: Sized + 'static,
            //OB: ProtoSpecBuilder<OBS> + 'static,
            OBS: BuilderState +  'static,
            ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, ExactStringSpec, IBS>> + 'static,{
                self.expect_exact_string(SpecName::Delimiter, " ".to_string(), false)
            }
    }

   

    pub trait KeySpecBuilder<IBS>: KeySpecGenerator + ProtoSpecBuilder<IBS>
    where 
        Self:Sized,
        IBS: BuildGenericString,
    {
        fn key_follows(self, name: SpecName, optional: bool) -> ProtoSpecBuilderData<BuildKey>
        /* where 
            OB: ProtoSpecBuilder<BuildKey> */
        {
            
            let mut result= ProtoSpecBuilderData::default();
            result.set_spec(self.build());
            result.set_state(BuildKey { key_spec_metadata: SpecMetaData::new(name, ValueType::None, optional) });
            result
        }
    }

    impl KeySpecBuilder<BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch> {}


    impl  StringSpecBuilder<BuildKey, BuildKeyAvailable> for ProtoSpecBuilderData<BuildKey>     
    {       
    }

    impl  StringSpecBuilder<BuildInlineValue, BuildFromScratch> for ProtoSpecBuilderData<BuildInlineValue>     
    {       
    }

    impl  StringSpecBuilder<BuildFromScratch, BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch>     
    {       
    }

    impl  StringSpecBuilder<BuildValue, BuildFromScratch> for ProtoSpecBuilderData<BuildValue>     
    {       
    }

    impl  DelimitedStringSpecBuilder<BuildInlineValue> for ProtoSpecBuilderData<BuildInlineValue>     
    
    {       
    }

    impl DelimitedStringSpecBuilder<BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch>     
    
    {       
    }

    impl DelimitedStringSpecBuilder<BuildKey> for ProtoSpecBuilderData<BuildKey>     
    {       
    }

    impl DelimitedStringSpecBuilder<BuildValue> for ProtoSpecBuilderData<BuildValue>     
    {       
    }

    pub struct BuilderWrapperWithData<B,D, BS>(B, D , PhantomData<BS> ) 
    where
        B:ProtoSpecBuilder<BS> + 'static, 
        BS:BuilderState + 'static;
    
    pub struct BuilderWrapper<B,BS>(B , PhantomData<BS> ) where B:ProtoSpecBuilder<BS> + 'static, BS:BuilderState + 'static;

     impl <D, IBS> From<BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, D, IBS>> for ProtoSpecBuilderData<IBS> 
     where 
         D:ProtocolSpec + 'static,
         IBS:BuilderState + 'static,
        
     {
         fn from(mut value: BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, D, IBS>) -> Self 
         {
             let from_builder = &mut value.0;             
             from_builder.add_spec(Box::new(value.1));
             value.0
         }
     }
    

     impl <D, IBS> From<BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, D, IBS>> for ProtoSpecBuilderData<BuildDelimiter<D, IBS>> 
     where 
         D:DelimitedSpec + 'static,
         IBS:BuilderState + 'static,
        
     {
         fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, D, IBS>) -> Self 
         {
             let mut from_builder =  value.0;
             let mut to_builder = ProtoSpecBuilderData::default();             
             let to_state = BuildDelimiter{
                delimiter_spec: value.1,
                parent_builder_state: from_builder.replace_current_state_with_default(),
             };
             to_builder.set_spec(from_builder.build());
             to_builder.set_state(to_state);             
             to_builder
         }
     }

     

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildKey>>, String , BuildDelimiter<D, BuildKey>>> for ProtoSpecBuilderData<BuildKeyAvailable> 
    where 
        D:DelimitedSpec + StringSpec + ProtocolSpec + 'static,        
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildKey>>, String, BuildDelimiter<D, BuildKey>>) -> Self 
        {
            let mut from_builder = value.0;            
            let from_state = from_builder.replace_current_state_with_default();
            let mut spec = from_state.delimiter_spec;
            spec.set_delimiter(Separator::Delimiter(value.1));
            let mut result = ProtoSpecBuilderData::default();
            let key = Key(Box::new(spec), from_state.parent_builder_state.key_spec_metadata);
            result.set_state(BuildKeyAvailable{
                key
            });
            result.set_spec(from_builder.build());
            result
        }
    }

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildFromScratch>>, String , BuildDelimiter<D, BuildFromScratch>>> for ProtoSpecBuilderData<BuildFromScratch> 
    where 
        D:DelimitedSpec + StringSpec + ProtocolSpec + 'static,        
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildFromScratch>>, String, BuildDelimiter<D, BuildFromScratch>>) -> Self 
        {
            let mut from_builder = value.0;            
            let from_state = from_builder.replace_current_state_with_default();
            let mut spec = from_state.delimiter_spec;
            spec.set_delimiter(Separator::Delimiter(value.1));
            let mut result = ProtoSpecBuilderData::default();
            let new_state = from_state.parent_builder_state;
            result.set_state(new_state);
            result.set_spec(from_builder.build());
            result.add_spec(Box::new(spec));
            result
        }
    }

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildKeyAvailable>>, String , BuildDelimiter<D, BuildKeyAvailable>>> for ProtoSpecBuilderData<BuildKeyAvailable> 
    where 
        D:DelimitedSpec + StringSpec + ProtocolSpec + 'static,      
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildKeyAvailable>>, String, BuildDelimiter<D, BuildKeyAvailable>>) -> Self 
        {
            let mut from_builder = value.0;            
            let from_state = from_builder.replace_current_state_with_default();
            let mut spec = from_state.delimiter_spec;
            spec.set_delimiter(Separator::Delimiter(value.1));
            let mut result = ProtoSpecBuilderData::default();
            let new_state = from_state.parent_builder_state;
            result.set_state(new_state);
            result.set_spec(from_builder.build());
            result.add_spec(Box::new(spec));
            result
        }
    }

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildKey>, D, BuildKey>> for ProtoSpecBuilderData<BuildKeyAvailable>
     where D:StringSpec + ProtocolSpec + 'static{
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildKey>, D, BuildKey>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();
            let key = Key(Box::new(value.1), from_state.key_spec_metadata);
            result.set_state(BuildKeyAvailable { key: key });
            result.set_spec(from_builder.build());    
            result
        }
    }

    impl  From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildKeyAvailable>, SpecMetaData, BuildKeyAvailable>> for ProtoSpecBuilderData<BuildValue>
     {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildKeyAvailable>, SpecMetaData, BuildKeyAvailable>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();
            
            result.set_state(BuildValue { key: from_state.key, value_spec_metadata: value.1 });
            result.set_spec(from_builder.build());    
            result
        }
    }

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildValue>, D, BuildValue>> for ProtoSpecBuilderData<BuildFromScratch>
    where D: Spec + ProtocolSpec + 'static
     {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildValue>, D, BuildValue>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();
            let optional = from_state.key.1.optional;
            let key_value = KeyValueSpec::new(
                from_state.key,
                ValueSpec(Box::new(value.1), from_state.value_spec_metadata),
                SpecMetaData::new(SpecName::Transient("key-value-spec".to_owned()), ValueType::None, optional),
            );
            from_builder.add_spec(Box::new(key_value));
            result.set_state(BuildFromScratch{});
            result.set_spec(from_builder.build());    
            result
        }
    }
             
    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildValue>>, String, BuildDelimiter<D, BuildValue>>> for ProtoSpecBuilderData<BuildFromScratch>
    where D: DelimitedSpec + ProtocolSpec + 'static,
           // IBS: BuilderState + 'static,
     {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildValue>>, String, BuildDelimiter<D, BuildValue>>) -> Self {
            let mut from_builder = value.0;
            let mut from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();
            let optional = from_state.parent_builder_state.key.1.optional;
            from_state.delimiter_spec.set_delimiter(Separator::Delimiter(value.1));
            let key_value = KeyValueSpec::new(
                from_state.parent_builder_state.key,
                ValueSpec(Box::new(from_state.delimiter_spec), from_state.parent_builder_state.value_spec_metadata),
                SpecMetaData::new(SpecName::Name("key-value-spec".to_owned()), ValueType::None, optional),
            );
            from_builder.add_spec(Box::new(key_value));
            result.set_state(BuildFromScratch{});
            result.set_spec(from_builder.build());    
            result
        }
    }

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildInlineValue>>, String, BuildDelimiter<D, BuildInlineValue>>> for ProtoSpecBuilderData<BuildFromScratch>
    where D: DelimitedSpec + ProtocolSpec + 'static,   
           // IBS: BuilderState + 'static,
     {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildInlineValue>>, String, BuildDelimiter<D, BuildInlineValue>>) -> Self {
            let mut from_builder = value.0;
            let mut from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();            
            from_state.delimiter_spec.set_delimiter(Separator::Delimiter(value.1));
            let inline_key_value = InlineKeyWithValue(Box::new(from_state.delimiter_spec), from_state.parent_builder_state.value_spec_metadata);
            from_builder.add_spec(Box::new(inline_key_value));
            result.set_state(BuildFromScratch{});
            result.set_spec(from_builder.build());    
            result
        }
    }

    impl  From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildInlineValue>, ExactStringSpec, BuildInlineValue>>  for ProtoSpecBuilderData<BuildFromScratch>{
        
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildInlineValue>, ExactStringSpec, BuildInlineValue>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();            
            let spec = value.1;
            let inline_key_value = InlineKeyWithValue(Box::new(spec), from_state.value_spec_metadata);
            from_builder.add_spec(Box::new(inline_key_value));
            result.set_state(BuildFromScratch{});
            result.set_spec(from_builder.build());    
            result
            
        }
    }

    //Generators
    impl <OBS> DelimiterGenerator for ProtoSpecBuilderData<OBS> 
    where 
        OBS: BuilderState
    {}


    #[allow(dead_code)]
    pub trait DelimiterGenerator{
        fn get_newline(&self) -> Separator{
            Separator::Delimiter("\r\n".to_owned())
        }
        fn get_delimiter(&self, delimiter:String) -> Separator{
            Separator::Delimiter(delimiter)
        }
        fn get_space(&self) -> Separator {
            Separator::Delimiter(" ".to_owned())
        }
    }

    pub trait DelimiterBuilder<D,IBS,  OBS>: ProtoSpecBuilder<BuildDelimiter<D, IBS>> + DelimiterGenerator
    where 
        D: DelimitedSpec + 'static,
        IBS: BuilderState + 'static,
        OBS: BuilderState + 'static,
        Self: Sized + 'static,
    {
        fn delimited_by_newline(self)-> ProtoSpecBuilderData<OBS>
        where 
            
            ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, String, BuildDelimiter<D, IBS>>>,
            
        {
            self.create_delimiter("\r\n".to_owned())
        }

        

        fn delimited_by_space(self,)-> ProtoSpecBuilderData<OBS>
        where ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, String, BuildDelimiter<D, IBS>>>,
        {
            self.create_delimiter( " ".to_string())
        }

        fn delimited_by(self, delimiter: String)-> ProtoSpecBuilderData<OBS>
        where ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, String, BuildDelimiter<D, IBS>>>,
        {
            self.create_delimiter( delimiter)
        }

        fn create_delimiter(self, delimiter: String,) -> ProtoSpecBuilderData<OBS>
        where 
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, String, BuildDelimiter<D, IBS>>>,
        {
            let r: BuilderWrapperWithData<Self, String, BuildDelimiter<D, IBS>> = self.wrap_with_data(delimiter);
            r.into()
        }
    }


   impl <D, IBS, OBS> DelimiterBuilder<D, IBS, OBS> for ProtoSpecBuilderData<BuildDelimiter<D, IBS>>
   where D: DelimitedSpec + 'static,
         IBS: BuilderState + 'static,
         OBS: BuilderState + 'static,
        
   {}

   

    pub(crate) mod protocol_reader;
    mod protocol_writer;
}

pub mod http;
mod utils;

#[cfg(test)]
mod tests {
    use crate::core::{
        BuildFromScratch, DelimitedStringSpecBuilder, DelimiterBuilder, ProtoSpecBuilderData
    };

    
    #[allow(unused)]
    fn test_string_placeholder(){
        let spec_builder = ProtoSpecBuilderData::new_with_state(BuildFromScratch::default(), crate::core::SpecName::NoName, true);
        let spec = spec_builder.expect_string(crate::core::SpecName::NoName, false);
        let _spec = spec.delimited_by_space();
                       
    }
}

#[cfg(test)]
mod test_utils {    

    use tracing::warn;

    use crate::{core::{InfoProvider, Mapper, RequestInfo}, mapping_extractor::DefaultMapper};

    pub fn assert_result_has_string(
        result: Result<Option<Vec<u8>>, crate::core::ParserError>,
        data: String,
    ) {
        match result {
            Ok(Some(result_data)) => {
                assert!(data == String::from_utf8(result_data).unwrap());
            }
            Ok(None) => {
                assert!(false);
            }
            Err(e) => {
                warn!("Error occured{}", e.to_string());
                assert!(false);
            }
        }
    }

    //#[derive(Default)]
    #[derive(Debug)]
    #[allow(unused)]
    pub struct TestRequestInfo(pub Box<dyn Mapper>, Vec<String>);

    #[allow(unused)]
    impl TestRequestInfo {
        pub fn new() -> Self {
            TestRequestInfo( Box::new(DefaultMapper::new()), Vec::new())
        }

        pub fn add_simple_keys(&mut self, mut keys:Vec<String>){
            self.1.append(&mut keys);
        }
    }

    impl RequestInfo for TestRequestInfo{}

    impl InfoProvider for TestRequestInfo {
        /* fn get_info(&self, key: &String) -> Option<&Value> {
            let key_ref = key.as_str();
            if self.1.contains(key) {
                return self.1
            }
            else{
                if let Some(spec_name) = spec_name{
                    return self.get_mapper().get_value_from_key_value_list(key_ref, &spec_name).clone();
                }
            }


        }

        fn add_info(&mut self, key: String, value: Value) {
            match key.as_str() {
                "request_method" | "protocol_version" | "request_uri" | "request_body" => {
                    self.get_mapper_mut().add_simple_data(key, value);
                }
                _ => {
                    //self.headers.insert(key.to_string(), value);
                    self.get_mapper_mut().add_to_key_value_list(key, value, "header-name".to_string(), "header-value".to_string());
                }
            }
        }     */
        
        fn get_mapper_mut(&mut self) ->&mut Box<dyn crate::core::Mapper> {
            &mut self.0
        }
        
        fn get_mapper(&self) ->&Box<dyn crate::core::Mapper> {
            &self.0
        }
        
        
    }
}
