
//#![debugger_visualizer(natvis_file = "./Foo.natvis")]
pub(crate) mod mapping_extractor{
    use std::collections::HashMap;



    use crate::core::{extract_name_and_spec_path, InlineKeyWithValue, Key, KeyValueSpec, ListSpec, MappableSpec, Mapper, MapperContext, ParserError, RepeatManySpec, RepeaterContext, SimpleValueSpec, Spec, SpecMapper, SpecName, SpecType, Value, ValueSpec};

     

    pub trait SpecTraverse{
        fn traverse(&self, mapper: &mut Box<dyn Mapper>);
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
            SpecType::RepeatMany(spec_name.clone(), self.repeat_count.clone(), 0)
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

    //TODO change the return value to Result instead of unit
    pub(crate) fn traverse_spec<S>(spec: &S, mapper: &mut Box<dyn Mapper>) where S:MappableSpec + ?Sized{
        let meta_data = spec.get_meta_data();
            //if let SpecName::Name(_) = meta_data.get_name(){
                mapper.get_mapper_context_mut().start_spec_type(spec.to_spec_type());    
                spec.add_mapping_template(mapper);
                mapper.get_mapper_context_mut().end_spec(spec);
            /* }else{
                spec.add_mapping_template(mapper);
            } */
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
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>)->Result<(), ParserError>  {
            self.0.traverse(mapper);
            Ok(())
            /* let key_name = &self.1;
            let path = mapper.get_mapper_context_mut().get_current_spec_path_template();
            mapper.add_mapping_template(key_name.to_owned(), path); */
        }
    }
   
    impl SpecMapper for RepeatManySpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) ->Result<(), ParserError>  {
            self.constituents.traverse(mapper);
            Ok(())
        }
    }

    
    impl <T> SpecMapper for T where T:SimpleValueSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>)->Result<(), ParserError>  {
            println!("delimited string spec {}", self.get_meta_data().get_name().to_string());

            if let Some(key_name) = mapper.get_mapper_context().get_last_available_spec_name() {
                let path = mapper.get_mapper_context_mut().get_current_spec_path_template();
                mapper.add_mapping_template(key_name, path);
            }
            Ok(())
            /* if let SpecName::Name(name) = key_name{
                //mapper.get_mapper_context_mut().start_spec_type(SpecType::Simple(SpecName::Name(name.to_owned())));
                let path = mapper.get_mapper_context_mut().get_current_spec_path_template();
                mapper.add_mapping_template(key_name.into(), path);
                //mapper.get_mapper_context_mut().end_current_spec();
            } */
        }
    }

    impl SpecMapper for ValueSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError> {
            self.0.traverse(mapper);
            Ok(())
            /* let key_name = self.1.get_name();
            if let SpecName::Name(name) = key_name{
                mapper.get_mapper_context_mut().start_spec_type(SpecType::Simple(key_name.clone()));
                
                mapper.get_mapper_context_mut().end_current_spec();
            }else{
                &self.0.traverse(mapper);
            } */
        }
    }

    impl SpecMapper for KeyValueSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) ->Result<(), ParserError> {

            println!("keyvalue name {}, key name {}, inner keyspec name {}", self.get_meta_data().get_name(), self.key.get_meta_data().get_name(), self.key.0.get_meta_data().get_name());
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
            //mapper.get_mapper_context_mut().end_current_spec();

            //mapper.get_mapper_context_mut().start_spec(&self.value);

            let ( value_name, value_spec_path,) = extract_name_and_spec_path(path_finder, mapper,&self.value, &self.value.0)?;
            match ( &value_name,  &value_spec_path){                
                (Some(name), Some(path)) => {
                    mapper.add_mapping_template(name.clone(), path.clone());
                }
                (_,_) =>{}
            }
            //mapper.get_mapper_context_mut().end_current_spec();

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

            self.0.traverse(mapper);
            Ok(())
            /* let path = mapper.get_mapper_context_mut().get();
                mapper.add_mapping_template(name.to_owned(), path); */
            /* if let SpecName::Name(name) = self.get_meta_data().get_name(){
                
            } */
        }   
    }

    

    impl SpecMapper for ListSpec{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) ->Result<(), ParserError> {
            self.constituents.iter().for_each(|s| s.traverse(mapper));
            Ok(())
        }
    }

}



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
    
    
    
    
    use std::collections::HashMap;
    use std::fmt::format;
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
        NoValidListConstituents(String),
        InvalidMarker {
            line_index: usize,
            char_index: usize,
            message: String,
        },

        IOError {
            error: std::io::Error,
        },
    }

    impl ParserError{
        fn is_eof(&self) -> bool{
            match self{
                ParserError::EndOfStream => true,
                _ => false,
            }
        }
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

                ParserError::NoValidListConstituents(name) => write!(
                                f,
                                "No consituent of the list spec {} has valid value", name                    
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
        fn get_info(&self, key: &String) -> Option<&Value>{
            self.get_mapper().get_value_by_key(key)
        }

        #[allow(unused)]
        fn get_info_by_spec_path(&self, spec_path: &String) -> Option<&Value>{
            self.get_mapper().get_spec_data().get(spec_path)
        }

        #[allow(unused)]
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

        fn add_info(&mut self, key: String, value: Value){
            self.get_mapper_mut().add_simple_data(key, value);
        }

        #[allow(unused)]
        fn add_info_by_spec_path(&mut self, key: String, key_spec_name: String, value: Value , value_spec_name: String) {
            self.get_mapper_mut().add_to_key_value_list(key, value, key_spec_name, value_spec_name);
        }

        //fn add_transient_info(&mut self, key: String, value: Value);

        //fn has_all_data(&self) -> bool;

        fn get_mapper_mut(&mut self) ->&mut Box<dyn Mapper>;

        fn get_mapper(&self) ->&Box<dyn Mapper>;

        
        
        fn get_mapper_context(&mut self) ->&mut MapperContext{
            self.get_mapper_mut().get_mapper_context_mut()
        }
    }

    #[derive(Clone, Debug)]
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
        fn add_defaults(&mut self);

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

        /* fn get_default_mapper_mut(&mut self) -> &mut Box<dyn Mapper>;        

        fn set_default_mapper(&mut self, mapper:Box<dyn Mapper>);        

        fn get_default_mapper(&self) -> & Box<dyn Mapper>;         */
        
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
        fn create_error_response_handler(&self) -> RESERRH;
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
            let parse_result = spec.deserialize(request_info,&mut  protocol_reader, true).await?;
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
            let parse_result = spec.deserialize(response_info,&mut  protocol_reader, true).await?;
            /* let result = protocol_reader
            .parse_composite(&mut request_info, spec).await; */
            
            /* if let Err(parser_error) = parse_result{
                if let ParserError::EndOfStream = parser_error  {
                    if response_info.has_all_data() {
                        return Ok(response_info);
                    }
                    return Err(ParserError::EndOfStream);
                } else {
                    return Err(parser_error);
                }
            } */
           //todo handle the above
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

        /* #[allow(unused)]
        async fn configure_mappers(&mut self) -> Result<(), ServerError>; */

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
            inner.get_request_spec().traverse(&mut mapper);
            Self { inner, mapper }
        }
    }

    impl <T> RequestFactory<T::REQI, T::REQSER, T::REQH, T::REQERRH, T::RESI> for MapperAwareRequestFactory<T> where T: ProtocolConfig{
        fn get_request_spec(&self) -> &ListSpec {
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
            inner.get_response_spec().traverse(&mut mapper);
            Self { inner, mapper }
        }
    }

    impl <T> ResponseFactory<T::RESI, T::RESSER, T::RESH, T::RESERRH, > for MapperAwareResponseFactory<T> where T: ProtocolConfig{
        fn get_response_spec(&self) -> &ListSpec {
            self.inner.get_response_spec()
        }
    
        fn create_response_info(&self) -> T::RESI {
            let mut response_info = self.inner.create_response_info();
            self.mapper.get_mapping_data_template().clone_into(response_info.get_mapper_mut().get_mapping_data_template_mut());
            response_info.add_defaults();
            response_info            

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
                    let result = serializer
                        .serialize_to(
                            response_info,
                            socket,
                            self.response_factory.get_response_spec(),
                        )
                        .await; 
                    if result.is_err(){
                        println!("error trying to handle the connection {} ", result.err().unwrap());
                    }
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
            reader: &mut (dyn SpecRead), update_info: bool,
        ) -> Result<Value, ParserError>;
    }

    struct SpecDeserializer<'a, S> where S: SerializableSpec{
        inner: &'a S
    }

    fn begin<S>(spec:&S, mapper_context:&mut MapperContext) where S: SerializableSpec{
            let spec_type = spec.to_spec_type();
            mapper_context.start_spec_type(spec_type);
    }

    fn end_current_context(mapper_context: &mut MapperContext){
        mapper_context.end_current_spec();
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
            (Self::RepeatMany(l0, l1, l2), Self::RepeatMany(r0, r1, r2)) => l0 == r0,
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
            let name = self.get_meta_data().get_name().to_name_string();
            mapper_context.start_spec(self);
            let result = self.0.serialize(info_provider, mapper_context, writer).await.end_spec(mapper_context, self);
            //let value = info_provider.get_info(&mapper_context.get_last_available_spec_name());
            if !self.2.optional & result.is_err(){
                //mapper_context.end_spec(self)?;
                //return Err(ParserError::MissingData(name.to_owned()));
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
                    reader.reset(&marker);
                    return Ok(value_type);                    
                }
                Err(e) => {
                    reader.reset(&marker);
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
                    reader.unmark(&marker);
                    return Ok(value_type);                    
                }
                Err(e) => {
                    match e{
                        
                        ParserError::EndOfStream => {
                            let optional = self.inner.inner.get_meta_data().is_optional();
                            reader.reset(&marker);
                            return Err(e);
                        }
                        _ => {
                            reader.reset(&marker);
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

    impl Separator{
        fn is_delimiter(&self) -> bool{
            match self{
                Separator::Delimiter(_) => true,
                _ => false,
            }
        }

        fn get_delimiter(&self) -> Option<String>{
            match self{
                Separator::Delimiter(delimiter) => Some(delimiter.clone()),
                _ => None,
            }
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

    pub(crate) trait SimpleValueSpec: Spec + SpecSerialize + SpecDeserialize + MappableSpec + SpecTraverse + ToSpecType {}

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
    pub(crate) enum RepeatCount{
        Fixed(u32),
        Delimited(Separator),
    }

    impl RepeatCount{
        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ), mapper_context: &mut MapperContext,
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
            return info_provider.get_mapper_context().end_spec(self)
            .map(|_| Value::None);
            
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
            mapper_context.start_spec(self);
            let mut index = 0;
            let mut has_one_success = false;
            loop{
                let result = self.constituents.serialize(info_provider, mapper_context, writer).await;
                has_one_success = has_one_success | result.is_ok();
                if result.is_ok(){
                    index+=1;
                    mapper_context.increment_current_repeat_spec();
                    continue;
                }else if !has_one_success && !self.get_meta_data().is_optional()  {
                    
                    mapper_context.end_spec(self)?;
                    self.repeat_count.serialize(info_provider, mapper_context, writer).await?;
                    return result;
                }else if !has_one_success && self.get_meta_data().is_optional(){
                    mapper_context.end_spec(self)?;
                    self.repeat_count.serialize(info_provider, mapper_context, writer).await?;
                    return Ok(());
                }else if has_one_success {
                    mapper_context.end_spec(self)?;
                    self.repeat_count.serialize(info_provider, mapper_context, writer).await?;
                    return Ok(());
                }
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
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) -> Result<(), ParserError>;
    }

    impl SpecMapper for Box<dyn ProtocolSpec>{
        fn add_mapping_template(&self, mapper: &mut Box<dyn Mapper>) ->Result<(), ParserError>  {
            (**self).add_mapping_template(mapper);
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

    pub(crate) trait Anyway{
        fn end_current_spec(self, mapper_context: &mut MapperContext) -> Self;

        fn end_spec<S>(self, mapper_context: &mut MapperContext,  spec: &S) -> Self where S: ToSpecType + ?Sized;
    }

    impl <R, E> Anyway for Result<R, E> 
    {
        fn end_current_spec(self, mapper_context: &mut MapperContext,  ) -> Self {
            mapper_context.end_current_spec();
            self
        }

        fn end_spec<S>(self, mapper_context: &mut MapperContext,  spec: &S) -> Self where S: ToSpecType + ?Sized{
            mapper_context.end_spec(spec);
            self
        }
    }

    fn end_context(context: &mut MapperContext){
        context.end_current_spec();
    }

    #[derive(Clone, Debug)]
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

        pub fn end_spec<S>(&mut self, in_spec: &S) -> Result<(), ParserError> where 
        S: ToSpecType + ?Sized{
            println!();
            println!("trying to close spec {:?}, the total spec {:?}", in_spec.to_spec_type(), self.types);
            println!();
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
                    
                    SpecType::RepeatMany(_, repeat_count, current_index) => {
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
                    SpecType::RepeatMany(name, _, current_index) => {
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

    pub(crate) trait Mapper:  Send + Sync + Debug{

        fn get_value_by_key(&self, spec_name: &str) -> Option<&Value>{
            let value_path = self.get_mapping_data_template().get(spec_name);
            if let Some(value_path) = value_path{
                self.get_spec_data().get(value_path)
            }else{
                None
            }
        }

        fn get_value_from_key_value_list(&self,key: String, spec_name: &str) -> Option<&Value>{
            let spec_path = self.get_mapping_data_template().get(spec_name);

            if let Some(spec_path) = spec_path{
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
            self.get_mapping_data_template_mut().insert(key_quick_lookup_name, value_spec_name);

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
                    info_provider.add_info(spec_name, value.clone());
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
            mapper_context.start_spec(self);
            let name = self.get_meta_data().get_name();            
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name.to_name_string(), value, self.get_meta_data().is_optional(), writer).await.end_spec(mapper_context, self)?;
            if let Separator::Delimiter(delimiter) = &self.until{
                writer.write(delimiter.as_bytes()).await?;
            }

            if let Separator::NBytes(delimiter) = &self.until{
                writer.write_data_u32(*delimiter).await?;
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
            reader: &mut dyn SpecRead, update_info: bool,
        ) -> Result<Value, ParserError>
        {
            let value = reader.read_placeholder_as_string(self.input.clone()).await?;
            if let Some(value) = value {
                if update_info && !self.spec_meta_data.get_name().is_delimiter() {

                    if let Some(name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                       info_provider.add_info(name, ValueType::parse(&self.get_meta_data().value_type, &value));
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
            mapper_context.start_spec(self);

            let name = self.get_meta_data().get_name().to_name_string();
            if let SpecName::Delimiter = self.get_meta_data().get_name() {
                write_data(name, Some(&Value::String(self.input.to_owned())), 
                    self.get_meta_data().is_optional(), 
                    writer).await.end_spec(mapper_context, self)?;
            }else{
                let value = info_provider.get_info(&mapper_context.get_current_spec_path());
                write_data(name, value, self.get_meta_data().is_optional(), writer).await.end_spec(mapper_context, self)?;
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
        
        let (mut spec_name, mut spec_path): (Option<String>, Option<String>) = (None, None);

        /* if let SpecName::Name(name) = spec_meta_data.get_name(){
            spec_name = Some(name.clone());
            
        } */
        mapper.get_mapper_context_mut().start_spec_type(spec.to_spec_type());
        mapper.get_mapper_context_mut().start_spec_type(inner_spec.to_spec_type());
        spec_name = mapper.get_mapper_context().get_last_available_spec_name();
        spec_path = Some(
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
            mapper_context.start_spec(self);
            self.key.serialize(info_provider, mapper_context, writer).await.end_spec(mapper_context, self)?;

            mapper_context.start_spec(self);
            self.value.serialize(info_provider, mapper_context, writer).await.end_spec(mapper_context, self)?;


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
                (Some(ref key_spec_path), Some(ref value_spec_path)) => {
                    if update_info{
                        info_provider.get_mapper_mut().add_to_key_value_list(key_name.get_string_value_unchecked().unwrap(),
                            value, key_spec_name.unwrap(), value_spec_name.unwrap());
                    }
                    /* let key_spec_template = info_provider.get_mapper_mut().get_mapping_data_template().get(&key_spec_name.unwrap()).unwrap();

                    let key_spec_template_path = format!("{}.{}", key_spec_template, );
                    info_provider.get_mapper_mut().get_spec_data_mut().insert(value_spec_path.clone(), value);
                    info_provider.get_mapper_mut().get_spec_data_mut().insert(key_spec_path.clone(), key_name);
                    info_provider.get_mapper_mut().get_mapping_data_mut().insert(key_spec_path.clone(), value_spec_path.clone());
                    info_provider.get_mapper_mut().get_mapping_data_mut().insert(key_spec_path.clone(), value_spec_path.clone());
                    info_provider.get_mapper_mut().get_mapping_data_mut().insert(key_spec_template_path, value_spec_path.clone()); */
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
                        info_provider.add_info(spec_name, Value::U8Vec(bytes.clone()));
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
            
            mapper_context.start_spec(self);
            let name = self.get_meta_data().get_name().to_name_string();
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name, value, self.get_meta_data().is_optional(), writer).await.end_spec(mapper_context, self)?;
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
            reader: &mut (dyn SpecRead), update_info: bool,   
        ) -> Result<Value, ParserError>
        {
            let bytes = reader.read_bytes(ReadBytesSize::Full).await?;
            if let Some(bytes) = bytes {
                if update_info{
                    if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                        info_provider.add_info(spec_name, Value::U8Vec(bytes.clone()));
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
            mapper_context.start_spec(self);
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name, value, self.get_meta_data().is_optional(), writer).await.end_spec(mapper_context, self)?;
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
                            info_provider.add_info(spec_name, Value::String(value.clone()));
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
            mapper_context.start_spec(self);
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());
            write_data(name, value, self.get_meta_data().is_optional(), writer).await.end_spec(mapper_context, self)?;
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
                println!("deserializing {}", constituent.get_meta_data().get_name());
                match result{
                    Ok(_) => {
                        has_one_success = true;
                        continue;
                    },
                    Err(ref e) => {
                        println!("{} is optional? {}, {}", constituent.get_meta_data().get_name(), constituent.get_meta_data().is_optional(),e);
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
            mapper_context.start_spec(self);
            for constituent in &self.constituents {                
                //mapper_context.start_spec(constituent);
                let result = constituent.serialize(info_provider, mapper_context, writer).await;
                println!("error is {:?}", result);
                if result.is_err() && !constituent.get_meta_data().is_optional() {
                    mapper_context.end_spec(self)?;
                    return result;
                }
            };
            mapper_context.end_spec(self)?;
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
            self.0.serialize(info_provider, mapper_context, writer).await.end_spec(mapper_context, self)?;            
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
            mapper_context.start_spec(self);
            self.0.serialize(info_provider, mapper_context, writer).await.end_spec(mapper_context, self)?;
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
            reader: &mut (dyn SpecRead), update_info: bool,
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(8)).await?;
            if let Some(bytes) = bytes {
                if update_info{
                    if let Some(spec_name) = info_provider.get_mapper_context().get_last_available_spec_name(){
                        info_provider.add_info(spec_name, ValueType::parse(&ValueType::UnSignedNumber64, &bytes));
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
                        info_provider.add_info(spec_name, ValueType::parse(&ValueType::SignedNumber64, &bytes));
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
                        info_provider.add_info(spec_name, ValueType::parse(&ValueType::UnSignedNumber32, &bytes));
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
                        info_provider.add_info(spec_name, ValueType::parse(&ValueType::UnSignedNumber16, &bytes));
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
                        info_provider.add_info(spec_name, ValueType::parse(&ValueType::SignedNumber16, &bytes));
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
            mapper_context.start_spec(self);
            let value = info_provider.get_info_by_spec_path(&mapper_context.get_current_spec_path());            
            write_data(name, value, self.get_meta_data().optional, writer).await.end_spec(mapper_context, self)?;
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

    pub fn new_spec_builder(name: SpecName)-> ProtoSpecBuilderData<BuildFromScratch>{
        ProtoSpecBuilderData::<BuildFromScratch>::new_with(name, false)
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
            let optional = from_state.parent_builder_state.value_spec_metadata.optional;
            from_state.delimiter_spec.set_delimiter(Separator::Delimiter(value.1));
            let inline_key_value = InlineKeyWithValue(Box::new(from_state.delimiter_spec), from_state.parent_builder_state.key_name, from_state.parent_builder_state.value_spec_metadata);
            from_builder.add_spec(Box::new(inline_key_value));
            result.set_state(BuildFromScratch{});
            result.set_spec(from_builder.build());    
            result
        }
    }

    impl  From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildInlineValue>, ExactStringSpec, BuildInlineValue>>  for ProtoSpecBuilderData<BuildFromScratch>{
        
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildInlineValue>, ExactStringSpec, BuildInlineValue>) -> Self {
            let mut from_builder = value.0;
            let mut from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();
            let optional = from_state.value_spec_metadata.optional;
            let spec = value.1;
            let inline_key_value = InlineKeyWithValue(Box::new(spec), from_state.key_name, from_state.value_spec_metadata);
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

    use crate::{core::{InfoProvider, Mapper, RequestInfo, Value}, mapping_extractor::DefaultMapper};

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

    //#[derive(Default)]
    #[derive(Debug)]
    pub struct TestRequestInfo(pub Box<dyn Mapper>, Vec<String>);

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
