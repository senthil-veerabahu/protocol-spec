
//#![debugger_visualizer(natvis_file = "./Foo.natvis")]
pub(crate) mod mapping_extractor{
    use std::collections::HashMap;



    use crate::core::{extract_name_and_spec_path, InlineKeyWithValue, Key, KeyValueSpec, ListSpec, MappableSpec, Mapper, MapperContext, RepeatManySpec, SimpleValueSpec, Spec, SpecMapper, SpecName, SpecType, Value, ValueSpec};

     

    pub trait SpecTraverse{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>);
    }

    #[derive(Clone, Default)]
    pub struct DefaultMapper{
        protocol_to_spec_field_map: HashMap<String, String>,
        protocol_to_spec_template_map: HashMap<String, String>,
        spec_data_map: HashMap<String, Value>,
        mapper_context: MapperContext,
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
   
    fn get_mapping_data(&self) -> &HashMap<String, String> {
        &self.protocol_to_spec_field_map
    }
   
    fn get_spec_data_mut(&mut self) -> &mut HashMap<String, crate::core::Value> {
        &mut self.spec_data_map
    }
   
    fn get_repeater_context_mut(&mut self, context_name: String) -> &mut crate::core::RepeaterContext {
        todo!()
    }
   
    fn get_mapper_context_mut(&mut self) -> &mut crate::core::MapperContext {
        &mut self.mapper_context
    }
    
    fn get_mapper_context(&self) -> &MapperContext {
        &self.mapper_context
    }
    
    fn get_mapping_data_template(&self) -> & HashMap<String, String> {
        todo!()
    }
    
    fn get_spec_data(&self) -> &HashMap<String, Value> {
        todo!()
    }
    
    fn get_repeater_context_map_mut(&mut self) -> &mut HashMap<String, crate::core::RepeaterContext> {
        todo!()
    }
   }

   pub trait ToSpecType: Spec{
        fn to_spec_type(&self) ->SpecType {
            let spec_name = self.get_meta_data().get_name();
            match spec_name{
                SpecName::Name(name) |
                SpecName::Transient(name) =>
                    SpecType::Simple(name.to_owned()),

                SpecName::NoName => SpecType::Simple("Default".to_owned())
            }
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

    /* impl ToSpecType for OneOfSpec {
    } */

    impl ToSpecType for KeyValueSpec {
    }

    /* impl ToSpecType for NBytesSpec {
    } */

    /* impl ToSpecType for ExactStringSpec {
    }

    impl ToSpecType for AllBytesSpec {
    }
 */
    /* impl ToSpecType for DelimitedStringSpec {
    }
 */
    impl ToSpecType for InlineKeyWithValue {
    }

   impl ToSpecType for RepeatManySpec{
        fn to_spec_type(&self) ->SpecType{
            let spec_name = self.get_meta_data().get_name();
            match spec_name{
                SpecName::Name(name) |
                SpecName::Transient(name) =>
                    SpecType::RepeatMany(name.to_owned(), self.repeat_count.clone(), 0),

                SpecName::NoName => SpecType::RepeatMany("Default".to_owned(), self.repeat_count.clone(), 0),
            }
        }
    }

    impl  SpecTraverse for Key{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper)
        }
    }

    impl <S> SpecTraverse for S where S:SimpleValueSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper)
        }
    }

    /* impl  SpecTraverse for NBytesSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper)
        }
    } */

    /* impl  SpecTraverse for AllBytesSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper)
        }
    } */

    impl  SpecTraverse for ValueSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper)
        }
    }

    /* impl  SpecTraverse for OneOfSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper)
        }
    }

    impl  SpecTraverse for ExactStringSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper)
        }
    } */

    impl  SpecTraverse for KeyValueSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper)
        }
    }

    impl SpecTraverse for RepeatManySpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper);
        }
    }

    impl SpecTraverse for InlineKeyWithValue{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper);
        }
    }

    
    pub(crate) fn traverse_spec<S>(spec: &S, mapper: &mut Box<dyn Mapper>) where S:MappableSpec + ?Sized{
        let meta_data = spec.get_meta_data();
            if let SpecName::Name(_) = meta_data.get_name(){
                mapper.get_mapper_context_mut().start_spec_type(spec.to_spec_type());    
                spec.add_mapping_template(mapper);
                mapper.get_mapper_context_mut().end_current_spec();    
            }else{
                spec.add_mapping_template(mapper);
            }
    }

    impl SpecTraverse for ListSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper);
        }
    }

    /* impl SpecTraverse for DelimitedStringSpec{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            traverse_spec(self, mapper);
        }
    } */

    impl SpecMapper for InlineKeyWithValue{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) {
            let key_name = &self.1;
            let path = mapper.get_mapper_context_mut().get_current_spec_path_template();
            mapper.add_mapping_template(key_name.to_owned(), path);
        }
    }
   
    impl SpecMapper for RepeatManySpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) {
            self.constituents.traverse(mapper);
        }
    }

    
    impl <T> SpecMapper for T where T:SimpleValueSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) {
            let key_name = self.get_meta_data().get_name();
            if let SpecName::Name(name) = key_name{
                mapper.get_mapper_context_mut().start_spec_type(SpecType::Simple(key_name.into()));
                let path = mapper.get_mapper_context_mut().get_current_spec_path_template();
                mapper.add_mapping_template(key_name.into(), path);
                mapper.get_mapper_context_mut().end_current_spec();
            }
        }
    }

    impl SpecMapper for ValueSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) {
            
            let key_name = self.1.get_name();
            if let SpecName::Name(name) = key_name{
                mapper.get_mapper_context_mut().start_spec_type(SpecType::Simple(key_name.into()));
                self.0.traverse(mapper);
                mapper.get_mapper_context_mut().end_current_spec();
            }else{
                &self.0.traverse(mapper);
            }
        }
    }

    impl SpecMapper for KeyValueSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) {

            
            let path_finder =  |mapper:  &Box<dyn Mapper>| {mapper.get_mapper_context().get_current_spec_path_template()};
            let ( key_name,  key_spec_path,) = extract_name_and_spec_path(path_finder, mapper, self.key.get_meta_data(), &self.key.0);
            match (&key_name, &key_spec_path){                
                (Some(name), Some(path)) => {
                    mapper.add_mapping_template(name.clone(), path.clone());
                }
                (_,_) =>{}
            }

            let ( value_name, value_spec_path,) = extract_name_and_spec_path(path_finder, mapper, self.value.get_meta_data(), &self.value.0);
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
        }
    }

    impl SpecMapper for Key{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) {
            if let SpecName::Name(name) = self.get_meta_data().get_name(){
                let path = mapper.get_mapper_context_mut().get_current_spec_path_template();
                mapper.add_mapping_template(name.to_owned(), path);
            }
        }
    }

    

    impl SpecMapper for ListSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>){
            self.constituents.iter().for_each(|s| s.traverse(mapper));
        }
    }

}



pub mod core {
    use crate::core::protocol_reader::ReadBytesSize;
    use crate::core::protocol_writer::PlaceHolderWrite;
    use crate::core::protocol_reader::PlaceHolderRead;
    use crate::mapping_extractor::{SpecTraverse, ToSpecType};    
    use async_trait::async_trait;
    use derive_builder::Builder;
    use protocol_reader::ProtocolBuffReader;
    use protocol_reader::{ MarkAndRead};

    use protocol_writer::ProtocolBuffWriter;
    
    
    
    
    use std::collections::HashMap;
    use std::marker::PhantomData;
    use std::string::ParseError;
    use std::{
        fmt::{Debug, Display, Formatter}, mem::{self}, str::Utf8Error
    };
    use tokio::{
        io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
        net::{TcpListener, TcpStream},
    };


    #[allow(dead_code)]
    pub trait ProtocolInfo {
        fn get_name() -> String;
        fn get_version() -> String;
        fn get_transport_type() -> Transport;
        fn get_format() -> ProtocolFormat;
    }

    #[allow(unused)]
    #[derive(Debug)]
    pub enum ParserError {
        InvalidPlaceHolderTypeFound {
            line_index: usize,
            char_index: usize,
            message: String,
        },
        TokenExpected {
            line_index: usize,
            char_index: usize,
            message: String,
        },
        InvalidToken {
            line_index: usize,
            char_index: usize,
            message: String,
        },
        UnexpectedKeyOrValue {          
            message: String,
        },
        MissingKey(String),
        MissingData(String),
        MissingValue(String),
        SerdeError(String),
        Utf8Error(Utf8Error),
        EndOfStream,
        InvalidMarker {
            line_index: usize,
            char_index: usize,
            message: String,
        },

        IOError {
            error: std::io::Error,
        },
    }

    impl<'l> Display for ParserError {
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
                ParserError::InvalidToken {
                                            line_index: line_pos,
                                            char_index: char_pos,
                                            message,
                                        } => {
                                            write!(
                                                f,
                                                "Invalid token at line {}  position {} {}",
                                                line_pos, char_pos, message
                                            )
                                        }
                ParserError::IOError { error } => {
                                            write!(f, "IO Error {}", error)
                                        }
                ParserError::MissingKey(msg) => write!(f, "{}", msg),
                ParserError::InvalidPlaceHolderTypeFound {
                                            line_index: line_pos,
                                            char_index: char_pos,
                                            message,
                                        } => {
                                            write!(
                                                f,
                                                "Invalid placeholder type found at line {}  position {} {}",
                                                line_pos, char_pos, message
                                            )
                                        }
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
                ParserError::InvalidMarker { line_index, char_index, message } => write!(
                    f,
                    "Invalid Marker provided during mark/reset operation at line {} char_pos {}: {}", line_index, char_index, message                       
                ),
                Self::UnexpectedKeyOrValue { message } => {
                    write!(f, "Unexpected key or value found: {}", message)
                }
            }
        }
    }

    #[allow(unused)]
    pub trait ValueExtractor<'a> {
        fn get_string_value_unchecked(&self) -> Result<String, ParserError>;
        fn get_signed_num_64_value_unchecked(&self) -> Result<i64, ParserError>;
        fn get_unsigned_num_64_value_unchecked(&self) -> Result<u64, ParserError>;
        fn get_unsigned_num_32_value_unchecked(&self) -> Result<u32, ParserError>;
        fn get_signed_num_16_value_unchecked(&self) -> Result<i16, ParserError>;
        fn get_unsigned_num_16_value_unchecked(&self) -> Result<u16, ParserError>;
        fn get_u8_vec_unchecked(&self) -> Result<&Vec<u8>, ParserError>;
        fn get_string_value(&self) -> Option<String>;
        fn get_signed_num_64_value(&self) -> Option<i64>;
        fn get_unsigned_num_64_value(&self) -> Option<u64>;
        fn get_unsigned_num_32_value(&self) -> Option<u32>;

        fn get_signed_num_16_value(&self) -> Option<i16>;
        fn get_unsigned_num_16_value(&self) -> Option<u16>;

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
        Map(String, HashMap<String, Value>),
        Composite(Vec<Value>),
        
        None,
    }

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
                Value::Map(_, hash_map) => todo!(),
                Value::Composite(values) => todo!(),
            }
            Ok(())
        }
    }



    pub trait InfoProvider:  Send + Sync{
        #[allow(unused)]
        fn get_info(&self, key: &String) -> Option<&Value>;

        #[allow(unused)]
        fn get_info_by_spec_path(&self, spec_path: &String) -> Option<&Value>{
            self.get_mapper().get_spec_data().get(spec_path)
        }

        #[allow(unused)]
        fn get_info_mut(&mut self, key: &String) -> Option<&mut Value>;

        #[allow(unused)]
        fn get_keys_by_group_name(&self, name: String) -> Option<Vec<& String>>;

        fn add_info(&mut self, key: String, value: Value);

        //fn add_transient_info(&mut self, key: String, value: Value);

        fn has_all_data(&self) -> bool;

        fn get_mapper_mut(&mut self) ->&mut Box<dyn Mapper>;

        fn get_mapper(&self) ->&Box<dyn Mapper>;

        
        
        fn get_mapper_context(&mut self) ->&mut MapperContext{
            self.get_mapper_mut().get_mapper_context_mut()
        }
    }

    pub(crate) struct RepeaterContext{
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

        fn reset(&mut self) {
            self.count = 0;
        }
    }



    pub trait RequestInfo: InfoProvider {
        
    }

    pub trait ResponseInfo: InfoProvider {
    }

    #[allow(unused)]
    pub trait RequestFactory<REQI, REQSER, REQH, REQERRH, RESI> : Send + Sync
    where
        REQI: RequestInfo,
        REQSER: RequestSerializer<REQI>,
        REQH: RequestHandler<REQI, RESI>,
        REQERRH: RequestErrorHandler<REQI, RESI>,
        RESI: ResponseInfo,
    {
        fn get_request_spec(&self) -> &ListSpec;
        fn create_request_info(&self) -> REQI;
        fn create_request_serializer(&self) -> REQSER;
        fn create_request_handler(&self) -> REQH;
        fn create_error_request_handler(&self) -> REQERRH;
    }

    #[allow(unused)]
    pub trait ResponseFactory<RESI, RESS, RESH, RESERRH>: Send + Sync
    where
        RESI: ResponseInfo,
        RESS: ResponseSerializer<RESI>,
        RESH: ResponseHandler<RESI>,
        RESERRH: ResponseErrorHandler<RESI>,
    {
        fn get_response_spec(&self) -> &ListSpec;
        fn create_response_info(&self) -> RESI;
        fn create_response_serializer(&self) -> RESS;
        fn create_response_handler(&self) -> RESH;
        fn create_error_responset_handler(&self) -> RESERRH;
    }

    #[async_trait]
    pub trait RequestHandler<REQI, RESI> : Send + Sync
    where
        REQI: RequestInfo,
        RESI: ResponseInfo,
    {
        async fn handle_request(&self, request: &REQI, response: &mut RESI) -> Result<RESI, ParserError>;
    }

    pub trait ResponseHandler<RESI> : Send + Sync
    where
        RESI: ResponseInfo,
    {
        #[allow(unused)]
        fn handle_response(&self, response: &RESI) -> Result<(), ParserError>;
    }

    pub trait ResponseErrorHandler<RESI>  : Send + Sync
    where
        RESI: ResponseInfo,
    {
        #[allow(unused)]
        fn handle_response_error<E>(
            &self,
            response_info: &RESI,
            error: E,
        ) -> Result<(), ParserError>;
    }

    

    pub trait RequestErrorHandler<REQI, RESI>: Send + Sync
    where
        REQI: RequestInfo,
        RESI: ResponseInfo,
    {
        #[allow(unused)]
        fn handle_request_error<E>(&self, request: &REQI, error: E) -> Result<RESI, ParserError>;
    }

    #[async_trait]
    pub trait RequestSerializer<
        REQI: RequestInfo> : Send + Sync
    {
        #[allow(unused)]
        async fn serialize_to<W>(
            &self,
            req: &mut REQI,
            writer: W,
            spec: &dyn SpecSerialize,
        ) -> Result<(), ParserError>
        where W: AsyncWrite + Unpin + Send + Sync;

        async fn deserialize_from<'a, B>(
            &self,
            request_info: &'a mut REQI,
            reader: B,
            spec: &dyn SpecDeserialize,
        ) -> Result<&'a mut REQI, ParserError> where B:AsyncRead + Unpin + Send + Sync;        
    }


    #[async_trait]
    pub trait ResponseSerializer<RSI>: Send + Sync 
    where RSI: ResponseInfo ,
        
    {
        async fn serialize_to<W>(
            &self,
            req: RSI,
            writer: W,
            spec: &dyn SpecSerialize,
        ) -> Result<(), ParserError>
        where W: AsyncWrite + Unpin + Send + Sync;

        #[allow(unused)]
        async fn deserialize_from<'a, R>(&self,  
            response_info: &'a mut RSI,
            reader: &mut BufReader<R>,
            spec: &dyn SpecDeserialize) -> Result<&'a mut RSI, ParserError>
        where R:SpecRead;
    }

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
            spec: &dyn SpecSerialize,
        ) -> Result<(), ParserError> 
        where W: AsyncWrite + Unpin + Send + Sync {
            let mut mapper_context = MapperContext::new();
            let mut protocol_writer = ProtocolBuffWriter::new(writer);
            spec.serialize(request_info, &mut mapper_context, &mut protocol_writer).await?;
            Ok(())
        }

        async fn deserialize_from<'a, B>(
            &self,
            mut request_info:  &'a mut REQI,
            reader: B,
            spec: &dyn SpecDeserialize,
        )  -> Result<&'a mut REQI, ParserError> 
        where B:AsyncRead + Unpin + Send + Sync  {
            let mut protocol_reader = ProtocolBuffReader::new( BufReader::new(reader), 1024);
            let parse_result = spec.deserialize(request_info,&mut  protocol_reader).await;
            if let Err(parser_error) = parse_result{
                if let ParserError::EndOfStream = parser_error  {
                    if request_info.has_all_data() {
                        return Ok(request_info);
                    }
                    return Err(ParserError::EndOfStream);
                } else {
                    return Err(parser_error);
                }
            }
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
            spec: &dyn SpecSerialize,
        ) -> Result<(), ParserError> where W: AsyncWrite + Unpin + Send + Sync {
            let mut protocol_writer = ProtocolBuffWriter::new(writer);
            let mut mapper_context= MapperContext::new();
            spec.serialize(&response_info,&mut mapper_context, &mut protocol_writer).await?;
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
            let parse_result = spec.deserialize(response_info,&mut  protocol_reader).await;
            /* let result = protocol_reader
            .parse_composite(&mut request_info, spec).await; */
            
            if let Err(parser_error) = parse_result{
                if let ParserError::EndOfStream = parser_error  {
                    if response_info.has_all_data() {
                        return Ok(response_info);
                    }
                    return Err(ParserError::EndOfStream);
                } else {
                    return Err(parser_error);
                }
            }
            Ok(response_info)
        }        
    }

    #[allow(unused)]
    pub struct Protocol {
        name: ProtocolVersion,
        transport: Transport,
        format: ProtocolFormat,
        request_place_holder: ListSpec, //Placeholder,
        response_place_holder: ListSpec,
    }

    #[allow(unused)]
    pub enum Transport {
        UDP,
        TCP,
    }

    #[allow(unused)]
    pub enum ProtocolFormat {
        Text,
        Binary,
    }

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

    #[derive(Debug,)]
    #[allow(unused)]
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

        /* async fn handle_request(&self, request: RQI) -> Result<RSI, ServerError>;
        async fn send_response(&self, response: RSI) -> Result<(), ServerError>; */
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
    
    #[derive(Builder)]
    #[builder(pattern = "owned")]
    pub struct ServerInstance<CFG> 
    where CFG: ProtocolConfig{
        hosts: Vec<String>,
        request_factory: CFG::REQF,
        #[allow(unused)]
        response_factory: CFG::RESF,

        #[builder(setter(skip))]
        listeners: Vec<TcpListener>,
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
                    println!("Accepted connection from {}", addr);

                    let _handle = tokio::spawn(async move {
                        self.handle_connection(socket).await;
                    });
                }
            });
        }

        async fn handle_connection(&'static self, mut socket: TcpStream) {
            let mut req_info = self.request_factory.create_request_info();
            let serializer = self.request_factory.create_request_serializer();

            let mut res_info = self.response_factory.create_response_info();
            let mut buf_reader  = BufReader::new(&mut socket);  
             let request_info = 
             serializer
                .deserialize_from(
                    &mut req_info,
                    &mut buf_reader,
                    self.request_factory.get_request_spec(),
                )
                .await
                .unwrap(); 
            let result = CFG::REQH::handle_request(
                &self.request_factory.create_request_handler(),
                &request_info,
                &mut res_info
            ).await;
            match result {
                Ok(response_info) => {
                    let serializer = self.response_factory.create_response_serializer();
                     serializer
                        .serialize_to(
                            response_info,
                            socket,
                            self.response_factory.get_response_spec(),
                        )
                        .await
                        .unwrap(); 
                }
                Err(e) => {
                    println!("Error handling request: {:?}", e);
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
                println!("hh{:?}", listener);
            }

            Ok(())
        }

        async fn stop(&mut self) -> Result<(), ServerError> {
            todo!()
        }
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

    fn is_send<T:Send>(t: T){}
    
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
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>;
    }

    struct SpecDeserializer<'a, S>{
        inner: &'a S
    }

    fn begin<S>(spec:&S, mapper_context:&mut MapperContext) where S: ProtocolSpec{
            let spec_type = spec.to_spec_type();
            mapper_context.start_spec_type(spec_type);
    }

    fn end_current_context(mapper_context: &mut MapperContext){
        mapper_context.end_current_spec();
    }

    #[async_trait]
    impl <'a, S> SpecDeserialize for SpecDeserializer<'a, S> where S: ProtocolSpec{
        async fn deserialize (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>{            
            begin(self.inner, info_provider.get_mapper_mut().get_mapper_context_mut());
            let value_result = self.inner.deserialize(info_provider, reader).await;

            
            if let Ok(value) = value_result{
                if let SpecName::Name(_) = self.inner.get_meta_data().get_name() {
                    let context: &mut MapperContext = info_provider.get_mapper_mut().get_mapper_context_mut();
                    let spec_name = context.get_current_spec_path();
                    info_provider.get_mapper_mut().get_spec_data_mut().insert(spec_name, value );
                    end_current_context(info_provider.get_mapper_mut().get_mapper_context_mut());
                    return Ok(Value::None);
                }else{
                    end_current_context(info_provider.get_mapper_mut().get_mapper_context_mut());
                    return Ok(value);
                }
                
            }else {
                end_current_context(info_provider.get_mapper_mut().get_mapper_context_mut());
                return value_result;
            }
            
            // should we clone the value instead of sending None?
            
        }
    }

    #[derive(Clone)]
    pub enum SpecType{
        Composite(String),
        RepeatMany(String, RepeatCount, u16),
        
        Key(String),
        Value(String),
        Simple(String),
    }

    struct SpecContext{
        contexts: Vec<SpecType>,
    }

    impl SpecContext{
        fn new()->Self{
            Self { contexts: vec!() }
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

    struct SpecData{
        data: HashMap<String, Value>
    }

    #[derive(PartialEq, Clone)]
    pub enum SpecName{
        NoName,
        Name(String),
        Transient(String)
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
            let name = self.get_meta_data().get_name().to_name_string();
            mapper_context.start_spec(self);
            
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            if let Some(value) = value{
                write(value, writer).await.end_current_spec(mapper_context)?;
                Ok(())
            }else if !self.2.optional {
                mapper_context.end_current_spec();
                return Err(ParserError::MissingData(name.to_owned()));
            }else{
                mapper_context.end_current_spec();
                Ok(())
            }
        }
    }

    struct UndoableDeserializer<'a, S>{
        inner:  &'a S
    }

    async fn undoable_deserialize<S>(spec: &S, info_provider: &mut ( dyn InfoProvider + Send + Sync ), reader: &mut (dyn SpecRead)) -> Result<Value, ParserError>
        where S: ProtocolSpec{

        let serialier = SpecDeserializer{
            inner: spec
        };
        let undoable_serializer = UndoableDeserializer{
                inner: &serialier,
        };
        undoable_serializer.deserialize(info_provider, reader).await
    }    

    #[async_trait]
    impl <'a, T: SpecDeserialize + Send + Sync> SpecDeserialize for UndoableDeserializer<'a, T>{        
        async fn deserialize (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>{
            let marker = reader.mark();
            let result = self.inner.deserialize(info_provider, reader).await;            
            match result {
                Ok(value_type) => {
                    reader.unmark(&marker);
                    return Ok(value_type);                    
                }
                Err(e) => {
                    reader.reset(&marker)?;
                    return Err(e);
                }
            }
        }
    }

    #[derive(Debug, Clone,)]
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
            match self{
                SpecName::Name(name) | SpecName::Transient(name) => name.to_owned(),
                SpecName::NoName => "Default".to_owned()
            }
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

    pub(crate) trait SimpleValueSpec: Spec + SpecSerialize + SpecDeserialize + MappableSpec + SpecTraverse + ToSpecType {}

    pub trait DelimitedSpec: SimpleValueSpec + Default{
        fn set_delimiter(&mut self, delimiter: Separator) ;
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
    }

    #[derive(Clone, )]
    pub(crate) enum RepeatCount{
        Fixed(u32),
        Delimited(Separator),
    }

    impl Default for RepeatCount{
        fn default() -> Self {
            RepeatCount::Fixed(2)
        }
    }

    #[derive(Default)]
    pub(crate) struct RepeatManySpec{
        spec_meta_data: SpecMetaData,        
        pub(crate) repeat_count: RepeatCount,
        pub(crate) constituents: ListSpec,
    }

    impl  RepeatManySpec{
        fn set_delimiter(&mut self, delimiter: Separator)  {
            self.repeat_count = RepeatCount::Delimited(delimiter);
        }
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
            reader: &mut dyn SpecRead,
        ) -> Result<Value, ParserError> {
            // Implementation for parsing repeat many spec
            let mut repeat_count = 0;
            loop{
                info_provider.get_mapper_context().increment_current_repeat_spec();
                self.constituents.deserialize(info_provider, reader).await?;
                repeat_count += 1;
                //info_provider.get_mapper().
                if let RepeatCount::Fixed(count) = &self.repeat_count {
                    if repeat_count >= *count {
                        break;
                    }
                } else if let RepeatCount::Delimited(ref delimiter) = &self.repeat_count {
                        let result = undoable_deserialize(&DelimitedStringSpec::new(SpecName::NoName, delimiter.clone(), false),
                        info_provider, reader).await;
                    if result.is_ok(){
                        break;
                    }
                }
            }
            info_provider.get_mapper_context().end_current_spec();
            Ok(Value::None) // Return appropriate value based on parsing
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
            mapper_context.start_spec(self);
            let mut index = 0;
            loop{
                let result = self.constituents.serialize(info_provider, mapper_context, writer).await;
                if result.is_err() {
                    match result.as_ref().expect_err("error expected"){
                        
                        ParserError::MissingData(_) => {
                            mapper_context.end_current_spec();
                            return Ok(());
                        },
                        _ => {
                            mapper_context.end_current_spec();
                            return result;
                        }
                        
                    }
                }                
                index+=1;
                mapper_context.increment_current_repeat_spec();
            }
        }
    }


    pub(crate) trait Spec: Send + Sync  {
        fn get_meta_data(&self) -> &SpecMetaData;
    }

    impl Spec for Box<dyn ProtocolSpec>{
        fn get_meta_data(&self) -> &SpecMetaData {
            (**self).get_meta_data()
        }
    }

    pub(crate) trait SpecMapper{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>);
    }

    impl SpecMapper for Box<dyn ProtocolSpec>{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) {
            (**self).add_mapping_template(mapper);
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
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>{
            (**self).deserialize(info_provider, reader).await
        }
    }

    impl ToSpecType for Box<dyn ProtocolSpec>{
        fn to_spec_type(&self) ->SpecType {
            (**self).to_spec_type()
        }
    }

    impl SpecTraverse for Box<dyn ProtocolSpec>{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
            (**self).traverse(mapper);
        }
    }
   

    impl <T> SerializableSpec for T where T: Spec + SpecSerialize + SpecDeserialize + ToSpecType{}

    pub trait MappableSpec: Spec + SpecTraverse + SpecMapper + ToSpecType{}

    impl <T> MappableSpec for T where T: Spec + SpecTraverse + SpecMapper + ToSpecType{}

    

    pub(crate) trait ProtocolSpec: SerializableSpec + MappableSpec{        
    }

    impl <T> ProtocolSpec for T where T: SerializableSpec + MappableSpec{}

    trait Anyway{
        fn end_current_spec(self, mapper_context: &mut MapperContext) -> Self;
    }

    impl <R, E> Anyway for Result<R, E> 
    {
        fn end_current_spec(self, mapper_context: &mut MapperContext,  ) -> Self {
            mapper_context.end_current_spec();
            self
        }
    }

    fn end_context(context: &mut MapperContext){

    }

    #[derive(Clone)]
    pub struct MapperContext{
        types: Vec<SpecType>,
    }
    fn test_m1<S:SerializableSpec>(s: S){}

    impl MapperContext{
        pub fn new() -> MapperContext{
            Self { types: vec!() }
        }

        pub fn start_spec<S>(&mut self, spec: &S) where S: ToSpecType{
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

        pub fn increment_current_repeat_spec(&mut self){
            let last = self.types.last_mut();
            if let Some( repeater) = last{
                match repeater{
                    
                    SpecType::RepeatMany(_, repeat_count, current_index) => {
                        *current_index += 1;
                    },
                    _ =>{}
                }
            }
        }

       
        pub fn get_current_spec_path_template(&self) -> String{
            let mut spec_template = String::new();
            self.types.iter().for_each(|spec_type|{
                spec_template = match spec_type{
                    SpecType::Composite(name) => format!("{}.{}", spec_template,name),
                    SpecType::RepeatMany(name,_repeat_count, _current_index) => format!("{}.{}.{{}}", spec_template,name),                    
                    SpecType::Key(name) => format!("{}.{}", spec_template,name),
                    SpecType::Value(name) => format!("{}.{}", spec_template,name),
                    SpecType::Simple(name) => format!("{}.{}", spec_template,name),
                }
            });
            spec_template
        }

        pub fn get_current_spec_path(&self) -> String{
            let mut spec_template = "$".to_string();
            self.types.iter().for_each(|spec_type|{
                spec_template = match spec_type{
                    SpecType::Composite(name) => format!("{}.{}", spec_template,name),
                    SpecType::RepeatMany(name, _, current_index) => format!("{}.{}.{}", spec_template, name, current_index),
                    
                    SpecType::Key(name) => format!("{}.{}", spec_template,name),
                    SpecType::Value(name) => format!("{}.{}", spec_template,name),
                    SpecType::Simple(name) => format!("{}.{}", spec_template,name),
                }
            });
            spec_template
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

    pub(crate) trait Mapper:  Send + Sync {

            fn get_value_by_key(&self, spec_name: &str) -> Option<&Value>{
            let value_path = self.get_mapping_data_template().get(spec_name);
            if let Some(value_path) = value_path{
                self.get_spec_data().get(value_path)
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

            // Map key to value spec name for quick lookup of value e.g. headers.0.HeaderName.Content-Type -> headers.0.HeaderValue
            self.get_mapping_data_mut().insert(key_quick_lookup_name, value_spec_name);

            Ok(())
        }

        


        fn add_mapping_data(&mut self, proto_name: String, spec_name: String) {
            self.get_mapping_data_mut().insert(proto_name, spec_name);

        }

        fn add_mapping_template(&mut self, proto_name: String, spec_name: String) {
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
        fn new(name: SpecName, delimiter: Separator,  optional: bool) -> Self {
            DelimitedStringSpec {                
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional),
                until: delimiter,
            }
        }
    }

    #[async_trait]
    impl SpecDeserialize for DelimitedStringSpec
    {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut dyn SpecRead,
        ) -> Result<Value, ParserError>      
        {
            //let mut buf = vec![];
             let value = match self.until {
                Separator::Delimiter(ref delimiter) => {
                    reader.read_placeholder_until(delimiter.to_owned()).await?
                }
                Separator::NBytes(size) => {
                    reader.read_bytes( ReadBytesSize::Fixed(size)).await?
                }
                Separator::EndOfStream => {
                    reader.read_bytes(ReadBytesSize::Full).await?
                }
            };

            if let Some(value) = value {
                info_provider.add_info(self.spec_meta_data.name.to_name_string(), Value::U8Vec(value.clone()));
                return Ok(ValueType::parse(&self.get_meta_data().value_type, &value));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read value for placeholder: {:?}",
                    self.get_meta_data().name.to_name_string()
                )))
            }
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
            mapper_context.start_spec(self);
            let name = self.get_meta_data().get_name();            
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name.to_name_string(), value, self.get_meta_data().is_optional(), writer).await.end_current_spec(mapper_context)?;
            if let Separator::Delimiter(delimiter) = &self.until{
                writer.write(delimiter.as_bytes()).await?;
            }
            Ok(())
        }
    }

    pub(crate) struct ExactStringSpec{
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
            reader: &mut dyn SpecRead,
        ) -> Result<Value, ParserError>
        {
            let value = reader.read_placeholder_as_string(self.input.clone()).await?;
            if let Some(value) = value {
                info_provider.add_info(self.spec_meta_data.name.to_name_string(), Value::U8Vec(value.clone()));
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
            mapper_context.start_spec(self);
            let name = self.get_meta_data().get_name().to_name_string();            
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name, value, self.get_meta_data().is_optional(), writer).await.end_current_spec(mapper_context)?;

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

    pub(crate) fn extract_name_and_spec_path<F> (
        spec_path_finder: F,
        mapper: &mut Box<dyn Mapper>, spec_meta_data: &SpecMetaData, inner_spec: &Box<dyn ProtocolSpec> ) -> (Option<String>, Option<String>)
        where F: Fn(&Box<dyn Mapper>) -> String
        {
        
        let (mut spec_name, mut spec_path): (Option<String>, Option<String>) = (None, None);

        if let SpecName::Name(name) = spec_meta_data.get_name(){
            spec_name = Some(name.clone());
            mapper.get_mapper_context_mut().start_spec_type(inner_spec.to_spec_type());
        }
        if let SpecName::Name(name) = inner_spec.get_meta_data().get_name(){
            spec_name = Some(name.clone());
            mapper.get_mapper_context_mut().start_spec_type(inner_spec.to_spec_type());
            spec_path = Some(
                spec_path_finder(mapper)                
            );
            //mapper.get_mapper_context().get_current_spec_path_template()
            mapper.get_mapper_context_mut().end_current_spec();    
        }else{                    
            spec_path = Some(spec_path_finder(mapper));
            mapper.get_mapper_context_mut().end_current_spec();                
        }
        return (spec_name, spec_path)
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
            /* let key_name = self.key.get_meta_data().get_name();
            let key_inner_name = self.key.0.get_meta_data().get_name();

            let key_value = match (key_name, key_inner_name) {
                (_, SpecName::Name(name)) => info_provider.get_mapper().get_key_value_map(name)?,
                (SpecName::Name(name), _) => info_provider.get_mapper().get_key_value_map(name)?,
                (_, _) => HashMap::new(),
            };

            key_value.iter().for_each(|item| {

            });

            writer.write(src) */
            mapper_context.start_spec(self);
            self.key.serialize(info_provider, mapper_context, writer).await.end_current_spec(mapper_context)?;

            mapper_context.start_spec(self);
            self.value.serialize(info_provider, mapper_context, writer).await.end_current_spec(mapper_context)?;


            //self.key.serialize(info_provider, writer).await?;
            Ok(())
        }
    }

    #[async_trait]
    impl SpecDeserialize for KeyValueSpec{
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
            
        ) -> Result<Value, ParserError>
        {
            let mapper = info_provider.get_mapper_mut();
            
            let path_finder =  |mapper:  &Box<dyn Mapper>| {mapper.get_mapper_context().get_current_spec_path()};

            let ( key_spec_name,  key_spec_path,) = extract_name_and_spec_path(path_finder, mapper, self.key.get_meta_data(), &self.key.0);
            let ( value_spec_name,  value_spec_path,) = extract_name_and_spec_path(path_finder,mapper, self.value.get_meta_data(), &self.value.0);


            

            let key_name = undoable_deserialize(&self.key, info_provider, reader).await?;
            let value = undoable_deserialize(&self.value, info_provider, reader).await?;
            /* if let Some(ref key_spec_path) = key_spec_path {
                info_provider.get_mapper_mut().get_spec_data_mut().insert(key_spec_path.clone(), key_name);
            }

            if let Some(ref value_spec_path) = value_spec_path {
                info_provider.get_mapper_mut().get_spec_data_mut().insert(value_spec_path.clone(), value);
            } */

            match (key_spec_path, value_spec_path){
                (None, None) => {},
                (None, Some(ref value_spec_path)) => {
                    info_provider.get_mapper_mut().get_spec_data_mut().insert(value_spec_path.clone(), value);
                },
                (Some(ref key_spec_path), None) => {
                    info_provider.get_mapper_mut().get_spec_data_mut().insert(key_spec_path.clone(), key_name);
                },
                (Some(ref key_spec_path), Some(ref value_spec_path)) => {
                    let key_spec_template = info_provider.get_mapper_mut().get_mapping_data_template().get(&key_spec_name.unwrap()).unwrap();

                    let key_spec_template_path = format!("{}.{}", key_spec_template, key_name.get_string_value_unchecked().unwrap());
                    info_provider.get_mapper_mut().get_spec_data_mut().insert(value_spec_path.clone(), value);
                    info_provider.get_mapper_mut().get_spec_data_mut().insert(key_spec_path.clone(), key_name);
                    info_provider.get_mapper_mut().get_mapping_data_mut().insert(key_spec_path.clone(), value_spec_path.clone());
                    info_provider.get_mapper_mut().get_mapping_data_mut().insert(key_spec_path.clone(), value_spec_path.clone());
                    info_provider.get_mapper_mut().get_mapping_data_mut().insert(key_spec_template_path, value_spec_path.clone());
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
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>
        {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(self.size)).await?;
            if let Some(bytes) = bytes {
                info_provider.add_info(self.spec_meta_data.name.to_name_string(), Value::U8Vec(bytes.clone()));
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
            
            mapper_context.start_spec(self);
            let name = self.get_meta_data().get_name().to_name_string();
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name, value, self.get_meta_data().is_optional(), writer).await.end_current_spec(mapper_context)?;
            Ok(())
        }
    }

    

    pub(crate) struct AllBytesSpec{     
        pub spec_meta_data: SpecMetaData,           
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
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            let bytes = reader.read_bytes(ReadBytesSize::Full).await?;
            if let Some(bytes) = bytes {
                info_provider.add_info(self.spec_meta_data.name.to_name_string(), Value::U8Vec(bytes.clone()));
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
            mapper_context.start_spec(self);
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name, value, self.get_meta_data().is_optional(), writer).await.end_current_spec(mapper_context)?;
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
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>
        {
            let spec = DelimitedStringSpec::new(
                self.get_meta_data().get_name().to_owned(),
                self.until.clone(),
                self.get_meta_data().is_optional());
                let result = undoable_deserialize(&spec, info_provider, reader).await?;
                //.undoable_parse(info_provider, reader).await?;
            if let Some(value) = &result.get_string_value() {
                if self.values.contains(value) {
                info_provider.add_info(self.spec_meta_data.name.to_name_string(), Value::String(value.clone()));
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
            mapper_context.start_spec(self);
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name, value, self.get_meta_data().is_optional(), writer).await.end_current_spec(mapper_context)?;
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
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            for constituent in &self.constituents {   
                undoable_deserialize(constituent, info_provider, reader).await?;
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
            mapper_context.start_spec(self);
            for constituent in &self.constituents {                
                constituent.serialize(info_provider, mapper_context, writer).await.or_else(
                    |err| {
                        mapper_context.end_current_spec();
                        Err(err)
                })?;
            }
            mapper_context.end_current_spec();
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

    pub(crate) struct InlineKeyWithValue(pub Box<dyn ProtocolSpec>, pub String, pub SpecMetaData);
    
    impl Spec for InlineKeyWithValue {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.2
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
            mapper_context.start_spec(self);
            //let name = self.1.get_name().to_name_string();
            //let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            //write_data(name, value, self.1.optional, writer).await.anyway(mapper_context)?;
            self.0.serialize(info_provider, mapper_context, writer).await.end_current_spec(mapper_context)?;            
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
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            undoable_deserialize(&self.0, info_provider, reader).await
        }
    }

    #[async_trait]
    impl SpecDeserialize for InlineKeyWithValue
    {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            undoable_deserialize(&self.0, info_provider, reader).await.map(|value| {
                info_provider.add_info(self.1.clone(), value);
                Value::None // or some other appropriate return value
            })
        }
    }

    #[async_trait]
    impl SpecDeserialize for ValueSpec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            undoable_deserialize(&self.0, info_provider, reader).await
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
            mapper_context.start_spec(self);
            self.serialize(info_provider, mapper_context, writer).await.end_current_spec(mapper_context)?;
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
    struct NumberU64Spec(SpecMetaData) ;

    #[derive(Default)]
    struct NumberI64Spec(SpecMetaData) ;

    #[derive(Default)]
    struct NumberU32Spec(SpecMetaData) ;

    #[derive(Default)]
    struct NumberU16Spec(SpecMetaData) ;

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
    struct NumberI16Spec(SpecMetaData);
    
    
    #[async_trait]
    impl SpecDeserialize for NumberU64Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(8)).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::UnSignedNumber64, &bytes))
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
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(8)).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::SignedNumber64, &bytes))
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
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4)).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::UnSignedNumber32, &bytes))
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
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4)).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::UnSignedNumber16, &bytes))
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
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4)).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::SignedNumber16, &bytes))
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
            mapper_context.start_spec(self);
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());            
            write_data(name, value, self.get_meta_data().optional, writer).await.end_current_spec(mapper_context)?;
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

    pub(crate)  trait ByteSpecGenerator{
        fn get_bytes_spec_of_size(&mut self, name: SpecName, size:u32, optional: bool) -> NBytesSpec{
            NBytesSpec::new(name, size, optional)
        }
    }

    pub(crate) trait BuilderState:Default{}

    pub(crate) trait BuildGenericString:BuilderState{}

    pub(crate) trait BuildKeyString:BuilderState{}
    

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
        key_name: String,
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
    impl  BuildKeyString for BuildKey{}
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
    pub(crate) struct ProtoSpecBuilderData<S:BuilderState>{
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

    pub fn new_spec_builder(name: SpecName, optional:bool)-> ProtoSpecBuilderData<BuildFromScratch>{
        ProtoSpecBuilderData::<BuildFromScratch>::new_with(name, optional)
    }

    impl <S> ProtoSpecBuilderData<S> where S:BuilderState {
        pub fn new_with_state(state: S, name: SpecName) -> Self {
            ProtoSpecBuilderData {
                composite_spec: ListSpec { 
                    spec_meta_data: {
                        SpecMetaData::new(name, ValueType::None, true)
                    },
                    constituents: Vec::new() 
                },
                state,
            }
        }

        pub fn new() -> Self {
            ProtoSpecBuilderData::new_with_state(S::default(), SpecName::Name("Default".to_owned()))
        }        

        pub fn new_with(name: SpecName, optional: bool) -> Self {
            let mut result = ProtoSpecBuilderData::new_with_state(S::default(), name);
            result
        }

        pub fn new_from_scratch(name: SpecName, optional: bool) -> ProtoSpecBuilderData<BuildFromScratch> {
            let mut result = ProtoSpecBuilderData::new_with_state(BuildFromScratch::default(), name);
            result
        }

    }
    
    //Generators

    pub(crate) trait NumberSpecGenerator {
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

    pub(crate) trait StringSpecGenerator{
        fn get_string_spec(&self, name: SpecName, optional: bool) -> DelimitedStringSpec where  Self:Sized{
            DelimitedStringSpec { 
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional), 
                until: Separator::EndOfStream 
            }
        }

        fn get_one_of_string(&self, name: SpecName, optional: bool, options: Vec<String>) ->  OneOfSpec where Self:Sized{
            OneOfSpec{ 
                spec_meta_data: SpecMetaData::new(name, ValueType::String, 
                optional), until: Separator::EndOfStream,  
                values: options 
            }
        }

        fn get_exact_string(&self, name: SpecName, input: String, optional: bool) -> ExactStringSpec where Self:Sized {
            ExactStringSpec::new(name, input, optional)
        }
    }

    pub(crate) trait KeySpecGenerator{
        fn get_key_spec(&self, name: SpecName, optional: bool) -> Key{
            let mut spec= DelimitedStringSpec::default();
            spec.spec_meta_data = SpecMetaData::new(name.clone(), ValueType::String, optional);
            Key(Box::new(spec), SpecMetaData::new(name , ValueType::None, optional))
        }
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

    pub(crate) trait NumberSpecBuilder <IBS,OBS, OB> :NumberSpecGenerator + ProtoSpecBuilder<IBS>
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
            let mut spec = self.get_u16_spec(name, optional);
            self.wrap_with_data(spec).into()
        }

        fn expect_u32(self, name: SpecName, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberU32Spec, IBS>> + 'static,            
        {
            let mut spec = self.get_u32_spec(name, optional);
            self.wrap_with_data(spec).into()
        }

        fn expect_u64(self, name: SpecName, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberU64Spec, IBS>> + 'static,            
        {
            let mut spec = self.get_u64_spec(name, optional);
            self.wrap_with_data(spec).into()
        }

        fn expect_i16(self, name: SpecName, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberI16Spec, IBS>> + 'static,            
        {
            let mut spec = self.get_i16_spec(name, optional);
            self.wrap_with_data(spec).into() 
        }

        fn expect_i64(self, name: SpecName, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberI64Spec, IBS>> + 'static,            
        {
            let mut spec = self.get_i64_spec(name, optional);
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
                key_name:key_name.to_name_string(),
                value_spec_metadata:SpecMetaData { name:  key_name, value_type: ValueType::None, optional: optional }
            }).into()
        }
    }
    
    impl From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildFromScratch>, BuildInlineValue, BuildFromScratch>> for ProtoSpecBuilderData<BuildInlineValue>{
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildFromScratch>, BuildInlineValue, BuildFromScratch>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
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
            
            let mut spec = self.get_string_spec(name, optional);
            self.wrap_with_data(spec).into()            
        }

        fn expect_one_of_string(self, name: SpecName, optional: bool, options: Vec<String>) ->  ProtoSpecBuilderData<BuildDelimiter<OneOfSpec, IBS>>  //impl ProtoSpecBuilder<BuildDelimiter<OneOfSpec, IBS>>
        where
        ProtoSpecBuilderData<BuildDelimiter<OneOfSpec, IBS>>:From<BuilderWrapperWithData<Self, OneOfSpec, IBS>> + 'static
        {
            //let name = name.unwrap_or("expect_one_of_string".to_string());
            let mut one_of_spec = OneOfSpec::new(name, optional, options);
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
                self.expect_exact_string(SpecName::NoName, "\r\n".to_string(), false)
            }

        fn expect_space(self,) -> ProtoSpecBuilderData<OBS> 
        where
            Self: Sized + 'static,
            //OB: ProtoSpecBuilder<OBS> + 'static,
            OBS: BuilderState +  'static,
            ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, ExactStringSpec, IBS>> + 'static,{
                self.expect_exact_string(SpecName::NoName, " ".to_string(), false)
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

    fn generate_key_name() -> String{
        "key-1".to_string()
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
    

    fn test(){
        //let t: ProtoSpecBuilderData<BuildKeyAvailable> = ProtoSpecBuilderData::new(BuildFromScratch::default()).expect_u16("rar".to_owned(), true);
         let t= ProtoSpecBuilderData::new_with_state(BuildFromScratch::default(), SpecName::NoName);
            let t = t.key_follows(SpecName::Name("keyname".to_string()), false);
            let t = t.expect_string(SpecName::NoName, true)
            .delimited_by_newline()
            //.expect_exact_string("name".to_owned(), "input".to_owned(), false)
            
            ;
        //let t = t.expect_string(SpecName::NoName,false);
        //let t: ProtoSpecBuilderData<BuildKeyAvailable> = t.delimited_by_space();
        //let t: ProtoSpecBuilderData<BuildKeyAvailable> = t.expect_exact_string(None, "test".to_string(), false);
        
        //t.expect_delimiter("dem".to_owned(), "delin".to_owned(), false);
            /* let t1= t.expect_exact_string("test".to_owned(), "delimiter".to_owned(), true);
            //let t1 = t1.expect_string("newstr".to_owned(), false);
            ``
            let  t1 = t1.expect_i16("name".to_owned(), false);
            let t1 = t1.expect_i16("tet".to_owned(), true); */
            
            //let x= t.expect_exact_string("fasf".to_owned(), "test".to_owned(),  false);
            //let x = t.expect_one_of_string(None,  false, vec!());
            //let x = x.delimited_by_space(); 
            let x = t.value_follows(SpecName::Name("test".to_string()), true)  ;
            let x = x.expect_string(SpecName::NoName,  true);
            let x = x.delimited_by("\r\n".to_owned());
            let x = x.expect_exact_string(SpecName::NoName, "test".to_string(), true);



            let x = x.repeat_many(SpecName::Name("repeat".to_string()), true, Separator::Delimiter("\r\n".to_owned()), ListSpec::new(SpecName::Name("test".to_owned()), ValueType::None, false));
                
                
            
            //x.expect_string("name".to_owned(), false);
                
                
    }

    

    pub(crate) struct BuilderWrapperWithData<B,D, BS>(B, D , PhantomData<BS> ) 
    where
        B:ProtoSpecBuilder<BS> + 'static, 
        BS:BuilderState + 'static;
    struct BuilderWrapper<B,BS>(B , PhantomData<BS> ) where B:ProtoSpecBuilder<BS> + 'static, BS:BuilderState + 'static;

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
                SpecMetaData::new(SpecName::Name("key-value-spec".to_owned()), ValueType::None, optional),
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
            let optional = from_state.parent_builder_state.value_spec_metadata.optional;
            from_state.delimiter_spec.set_delimiter(Separator::Delimiter(value.1));
            let inline_key_value = InlineKeyWithValue(Box::new(from_state.delimiter_spec), from_state.         parent_builder_state.key_name, from_state.parent_builder_state.value_spec_metadata);
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


    pub(crate) trait DelimiterGenerator{
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
            let r = self.wrap_with_data(delimiter);
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
        BuildFromScratch, DelimiterBuilder, InlineValueBuilder, ProtoSpecBuilderData, StringSpecBuilder, DelimitedStringSpecBuilder
    };

    

    fn test_string_placeholder(){
        let mut spec_builder = ProtoSpecBuilderData::new_with_state(BuildFromScratch::default(), crate::core::SpecName::NoName);
        let spec = spec_builder.expect_string(crate::core::SpecName::NoName, false);
        let spec = spec.delimited_by_space();
                       
    }

    #[test]
    fn test_protocol_spec_builder() {
        let mut spec_builder = ProtoSpecBuilderData::new_with_state(BuildFromScratch::default(), crate::core::SpecName::NoName);


        let spec_builder = spec_builder.inline_value_follows(crate::core::SpecName::Name("key-1".to_owned()), false);
    }
}

#[cfg(test)]
mod test_utils {
    use std::collections::HashMap;

    use crate::{core::{InfoProvider, Value}, mapping_extractor::DefaultMapper};

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
                println!("Error occured{}", e.to_string());
                assert!(false);
            }
        }
    }

    #[derive(Default)]
    pub struct TestRequestInfo(HashMap<String, Value>, HashMap<String, Value>, HashMap<String, HashMap<String, Value>>, DefaultMapper);

    impl TestRequestInfo {
        pub fn new() -> Self {
            TestRequestInfo(HashMap::new(), HashMap::new(), HashMap::new(), DefaultMapper::new())
        }
    }

    impl InfoProvider for TestRequestInfo {
        fn add_info(&mut self, key: String, value: Value) {
            self.0.insert(key, value);
        }

        fn get_info(&self, key: &String) -> Option<&crate::core::Value> {
            if let Some(value) = self.0.get(key) {
                Some(value)
            }else if let Some(value) = self.1.get(key){
                Some(value)
            }
            else {
                None
            }
        }

        fn get_keys_by_group_name(&self, _name: String) -> Option<Vec<&String>> {
            let mut keys = Vec::new();
            for (key, _value) in &self.0 {
                keys.push(key);
            }
            if keys.is_empty() {
                None
            } else {
                Some(keys)
            }
        }

        fn get_info_mut(&mut self, key: &String) -> Option<&mut Value> {
            if let Some(value) = self.0.get_mut(key) {
                Some(value)
            }else if let Some(value) = self.1.get_mut(key){
                Some(value)
            }
            else {
                None
            }
        }
        
        fn has_all_data(&self) -> bool {
            todo!()
        }
        
        fn get_mapper_mut(&mut self) ->&mut Box<dyn crate::core::Mapper> {
            todo!()
        }
        
        fn get_mapper(&self) ->&Box<dyn crate::core::Mapper> {
            todo!()
        }
    }
}
