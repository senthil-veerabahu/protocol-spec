//#![debugger_visualizer(natvis_file = "./Foo.natvis")]


pub mod core {
    use crate::core::protocol_reader::ReadBytesSize;
    use crate::core::{protocol_reader::PlaceHolderRead, PlaceHolderIdentifier::Name};
    use crate::core::PlaceHolderType::OneOf;
    use async_trait::async_trait;
    use derive_builder::Builder;
    use protocol_reader::ProtocolBuffReader;
    use protocol_reader::{ MarkAndRead};

    use protocol_writer::ProtocolBuffWriter;
    use serde::de;
    
    use std::marker::PhantomData;
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
        MissingKey,
        MissingData,
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
                ParserError::MissingKey => write!(
                                            f,
                                            "Key value pair is expected. But key is missing, only value is present "
                                        ),
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
                ParserError::MissingData => {
                                            write!(f, "Expected data but found none whle writing to writer",)
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
            }
        }
    }

    /*fn get_value_from_enum<T>(val_type: ValueType) -> Box<dyn ValueExtractor<T>> {
        match val_type {
            ValueType::String(s) => { return Box::new(Value(s)); }
            ValueType::SignedNumber64(data) => { return Box::new(Value(data)); }
            ValueType::UnSignedNumber64(data) => { return Box::new(Value(data)); }
            ValueType::UnSignedByteSlice(data) => { return Box::new(Value(data)); }
            ValueType::None => { Box::new(Value(())) }
        }
    }*/

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
        //fn get_unsigned_byte_slice(&self) -> Option<&'a [u8]>;

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

        /* fn get_unsigned_byte_slice(&self) -> Option<&'a [u8]> {
            return match self {
                ValueType::UnSignedByteSlice(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        } */

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

    //#[derive(Clone)]
    #[allow(unused)]
    #[derive(Debug)]
    pub enum Value {
        String(String),
        SignedNumber64(i64),
        UnSignedNumber64(u64),
        UnSignedNumber32(u32),
        SignedNumber16(i16),
        UnSignedNumber16(u16),
        //UnSignedByteSlice([u8]),
        U8Vec(Vec<u8>),
        //StreamData(&'static mut protocol_reader::ProtoStream<BufReader<tokio::net::TcpStream>>),
        //StreamData1(StreamValueType<ProtoStream<BufReader<TcpStream>>>),
        None,
        //KeyValueCollection(Vec<(String, ValueType)>),
    }

    pub enum ValueType {
        String,
        SignedNumber64,
        UnSignedNumber64,
        UnSignedNumber32,
        SignedNumber16,
        UnSignedNumber16,
        U8Vec,
        None,
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
                ValueType::None => {
                                            Value::None
                                        }
            }
        }
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
            }
            Ok(())
        }
    }

    /*     #[allow(unused)]
    pub trait RequestProcessorRegistrar {
        fn register_request_processor<'a, H, RI>(
            request_type: String,
            request_info: RI,
            request_handler: H,
        ) where
            H: RequestHandler,
            RI: RequestInfo;
    } */

    

    /* pub trait RequestParse {
        #[allow(unused)]
        async fn parse_request<RI, RequestStream>(
            &self,
            reader: RequestStream,
            request_spec: &Placeholder,
        ) -> Result<RI, ParserError>
        where
            /*H: RequestHandler,*/
            RI: RequestInfo,
            RequestStream: AsyncRead + Unpin;
    } */

    /* impl RequestParse for Parser {
        async fn parse_request<RI, RequestStream, >(
            &self,
            reader: RequestStream,
        ) -> Result<RI, ParserError>
        where
            RI: RequestInfo,
            RequestStream: AsyncRead + Unpin,
        {


            todo!()
        }
    } */

    /*
    use tokio::stream::StreamExt;



    async fn process_stream(mut stream: impl Stream<Item = String>) {

    while let Some(value) = stream.next().await {

        println!("Value: {}", value);

    }

    }*/

    pub trait InfoProvider:  Send + Sync{
        #[allow(unused)]
        fn get_info(&self, key: &String) -> Option<&Value>;

        #[allow(unused)]
        fn get_info_mut(&mut self, key: &String) -> Option<&mut Value>;

        #[allow(unused)]
        fn get_keys_by_group_name(&self, name: String) -> Option<Vec<& String>>;

        fn add_info(&mut self, key: String, value: Value);

        fn has_all_data(&self) -> bool;

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
            spec: &dyn Spec,
        ) -> Result<(), ParserError>
        where W: AsyncWrite + Unpin + Send + Sync;

        async fn deserialize_from<'a, B>(
            &self,
            request_info: &'a mut REQI,
            reader: B,
            spec: &dyn Spec,
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
            spec: &dyn Spec,
        ) -> Result<(), ParserError>
        where W: AsyncWrite + Unpin + Send + Sync;

        #[allow(unused)]
        async fn deserialize_from<'a, R>(&self,  
            response_info: &'a mut RSI,
            reader: &mut BufReader<R>,
            spec: &dyn Spec) -> Result<&'a mut RSI, ParserError>
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
            spec: &dyn Spec,
        ) -> Result<(), ParserError> 
        where W: AsyncWrite + Unpin + Send + Sync {
            let mut protocol_writer = ProtocolBuffWriter::new(writer);
            protocol_writer
                .write_composite(spec, request_info, None)
                .await?;
            Ok(())
        }

        async fn deserialize_from<'a, B>(
            &self,
            mut request_info:  &'a mut REQI,
            reader: B,
            spec: &dyn Spec,
        )  -> Result<&'a mut REQI, ParserError> 
        where B:AsyncRead + Unpin + Send + Sync  {
            let mut protocol_reader = ProtocolBuffReader::new( BufReader::new(reader), 1024);
            let parse_result = spec.parse(request_info,&mut  protocol_reader).await;
            /* let result = protocol_reader
            .parse_composite(&mut request_info, spec).await; */
            
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
            spec: &dyn Spec,
        ) -> Result<(), ParserError> where W: AsyncWrite + Unpin + Send + Sync {
            let mut protocol_writer = ProtocolBuffWriter::new(writer);
            protocol_writer
                .write_composite(spec, &response_info, None)
                .await?;
            Ok(())
        }

        //(&self, mut response_info: RSI,reader: R, spec: &Placeholder)
        //async fn deserialize_from(&self,  response_info: &mut RSI,reader: &mut BufReader<&mut R>, spec: &Placeholder) -> Result<RSI, ParserError>;

        async fn deserialize_from<'a, R>(
            &self,
            response_info:&'a mut RESI,
            reader: &mut BufReader< R>,
            spec: &dyn Spec,
        ) -> Result<&'a mut RESI, ParserError> 
        where R:SpecRead {
            let mut protocol_reader = ProtocolBuffReader::new(reader, 1024);
            let parse_result = spec.parse(response_info,&mut  protocol_reader).await;
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
            /* protocol_reader
                .parse_composite(response_info, spec).await?; */
                
            //Ok(())
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
        /* type R: AsyncRead + Unpin + Send + Sync;
        type W: AsyncWrite + Unpin + Send + Sync; */
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

        /* #[allow(unused)]
        protocol: Protocol, */

        request_factory: CFG::REQF,
        #[allow(unused)]
        response_factory: CFG::RESF,

        #[builder(setter(skip))]
        listeners: Vec<TcpListener>,

       // phantom_req_info: std::marker::PhantomData<CFG::REQF>,
       // phantom_res_info: std::marker::PhantomData<CFG::RESF>,
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
            //let read = Box::new(&mut socket);
            //let read = & mut socket;
            
           // let x = *read;
            let mut buf_reader  = BufReader::new(&mut socket);  
            //serializer.serialize_to(req_info, socket, self.request_factory.get_request_spec(),).await;
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
            //let host = String::new();
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

        /* async fn handle_request(&self, request: RequestInfo) -> Result<ResponseInfo, ParserError> {
            todo!()
        }

        async fn send_response(&self, response: ResponseInfo) -> Result<(), ParserError> {
            todo!()
        } */
    }

    /* impl<RQI, RSI> ProtocolBuilder<RQI, RSI>
    where
        RQI: RequestInfo,
        RSI: ResponseInfo,
    {
        pub fn new(
            name: String,
            version: Option<String>,
            transport: Transport,
            format: ProtocolFormat,
        ) -> Self {
            ProtocolBuilder {
                name,
                version,
                transport,
                format,
                request_info: RequestInfo::default(),
                response_info: ResponseInfo::default(),
            }
        }
    } */

    #[allow(unused)]
    pub trait Processor {
        fn process(_req: Request, _res: Response) {}
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

    /* #[derive(Default)]
    #[allow(unused)]
    pub enum PlaceHolderType {
        #[default]
        AnyString,
        ExactString(String),
        OneOf(Vec<String>),
        BytesOfSizeFromHeader(String),
        Bytes,
        BytesOfSizeN(u32),
        Space,
        NewLine,
        Delimiter(String),
        Composite,
        RepeatMany,
        RepeatN(u8),
        StreamValue(String),
    } */

    /* #[allow(unused)]
    struct Key(String); */

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

    /* fn test(){
        is_send(Placeholder::default());
        //is_send(Req::default());
        //is_send(Rc::new("test".to_owned()));
    } */

    /* #[derive(Default)]
    pub struct Placeholder<T> where T: Parse + Send + Sync {
        #[allow(dead_code)]
        name: String,
        parser: T,
        //pub constituents: Option<Vec<Box<dyn Parse + Send + Sync>>>,

        #[allow(dead_code)]
        pub optional: bool,
    } */

    /* impl <T:Parse  +Send + Sync> Placeholder<T> {
        pub fn new(
            name: String,            //constituents: Option<Vec<Placeholder>>,
            parse: T,
            optional: bool,
        ) -> Self {
            Placeholder {
                name,
                parse,                
                optional,
            }
        }

        #[allow(unused)]
        pub fn new_key_placeholder(
            name: String,
            parse: T,
            optional: bool,
        ) -> Placeholder<Key<T>> {
            Placeholder::<Key<T>> {
                name,
                optional,
                parse:Key(parse),
                
            }
        }

        #[allow(unused)]
        pub fn new_placeholder_with_key(
            name: String,
            constituents: Option<Vec<Placeholder>>,
            place_holder_type: PlaceHolderType,
            optional: bool,
        ) -> Self {
            Placeholder {
                name,
                place_holder_type,                
                optional,
            }
        }

        #[allow(unused)]
        pub fn new_value_placeholder(
            name: String,
            place_holder_type: PlaceHolderType,
            optional: bool,
        ) -> Self {
            Placeholder {
                name,
                place_holder_type,
                optional,
            }
        }
    } */

    pub enum PlaceHolderValue {
        #[allow(unused)]
        AnyString(String),
        #[allow(unused)]
        OneOf(String),
        #[allow(unused)]
        Delimiter(String),
        #[allow(unused)]
        AnyBytes(Vec<u8>),
    }

    impl PlaceHolderValue {
        #[allow(unused)]
        pub fn parse(place_holder_type: &PlaceHolderType, value: &[u8]) -> Value {
            match place_holder_type {
                PlaceHolderType::AnyString => {
                                            Value::String(String::from_utf8(value.to_vec()).unwrap())
                                        }
                PlaceHolderType::ExactString(input) => {
                                            Value::String(String::from_utf8(value.to_vec()).unwrap())
                                            //todo!("Implement ExactString")
                                        }
                PlaceHolderType::OneOf(_) => {
                                            Value::String(String::from_utf8(value.to_vec()).unwrap())
                                        }
                PlaceHolderType::Delimiter(_) => {
                                            Value::String(String::from_utf8(value.to_vec()).unwrap())
                                        }
                PlaceHolderType::Space => Value::String(" ".to_string()),
                PlaceHolderType::NewLine => Value::String("\r\n".to_string()),
                PlaceHolderType::Composite => todo!(),
                PlaceHolderType::RepeatMany => todo!(),
                PlaceHolderType::RepeatN(_) => todo!(),
                OneOf(items) => todo!(),
                PlaceHolderType::StreamValue(data) => todo!(),
                PlaceHolderType::BytesOfSizeFromHeader(_) => Value::U8Vec(value.to_vec()),
                PlaceHolderType::BytesOfSizeN(_) => Value::U8Vec(value.to_vec()),
                PlaceHolderType::Bytes => {
                    Value::U8Vec(value.to_vec())                
                },
            }
        }
    }

    #[allow(dead_code)]
    pub struct TextProtocolSpec {
        request_spec: PlaceHolderType,
        response_spec: PlaceHolderType,
    }

    #[allow(dead_code)]
    trait TokenParser {
        async fn read_string(until_delimiter: String) -> String;
    }

    /* #[allow(unused)]
    async fn parse_request<RI: RequestInfo, Reader: AsyncRead + Unpin>(
        reader: Reader,
        request_info: &mut RI,
        request_spec: &Placeholder,
    ) -> Result<(), ParserError> {
        let mut protocol_reader =
            crate::core::protocol_reader::ProtocolBuffReader::new(BufReader::new(reader), 1024);
        protocol_reader
            .parse_composite(request_info, request_spec)
            .await?;
        Ok(())
    } */

   trait SpecRead: PlaceHolderRead + MarkAndRead + AsyncRead + Unpin + Send + Sync {
    }

    #[async_trait]
    trait Parse{
    /* where IP: InfoProvider + Send + Sync,
              B: PlaceHolderRead + MarkAndRead + AsyncRead + Unpin + Send + Syn *///{        
        /* type IP: InfoProvider + Send + Sync;
        type B: PlaceHolderRead + MarkAndRead + AsyncRead + Unpin + Send + Sync; */

        async fn parse (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> ;
        
    }

    #[async_trait]
pub trait MarkAndResetParse: Parse {
    async fn mark_and_parse(
        &self,
        info_provider: &mut ( dyn InfoProvider + Send + Sync ),
        reader: &mut (dyn SpecRead),
        
    ) -> Result<crate::core::Value, ParserError>{
            let marker = reader.mark();
            let result = self.parse(info_provider, reader).await;
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

impl<P> MarkAndResetParse for P where P: Parse {}





impl MarkAndResetParse for dyn Spec {}
    /* async fn mark_and_parse(
        &self,
        info_provider: &mut (dyn InfoProvider + Send + Sync),
        reader: &mut (dyn SpecRead),
    ) -> Result<crate::core::Value, ParserError> {
        (**self).mark_and_parse(info_provider, reader).await
    }
} */

impl MarkAndResetParse for dyn Parse + Send + Sync {}

impl MarkAndResetParse for dyn Spec + Send + Sync {}
    /* async fn mark_and_parse(
        &self,
        info_provider: &mut (dyn InfoProvider + Send + Sync),
        reader: &mut (dyn SpecRead),
    ) -> Result<crate::core::Value, ParserError> {
        let marker = reader.mark();
        let result = self.parse(info_provider, reader).await;
        match result {
            Ok(value_type) => {
                reader.unmark(&marker);
                Ok(value_type)
            }
            Err(e) => {
                reader.reset(&marker)?;
                Err(e)
            }
        }
    }
} */

 

    #[derive(Debug, Clone)]
    enum Separator{
        Delimiter(String),
        NBytes(u32),
        EndOfStream,
    }

    #[derive(Debug, Clone)]
    pub struct SpecMetaData {
        name: String,
        value_type: ValueType,
        optional: bool,
    }

    impl Default for SpecMetaData {
        fn default() -> Self {
            SpecMetaData {
                name: String::new(),
                value_type: ValueType::None,
                optional: false,
            }
        }
    }

    impl SpecMetaData {
        pub fn new(name: String, value_type: ValueType, optional: bool) -> Self {
            SpecMetaData {
                name,
                value_type,
                optional,
            }
        }

        pub fn get_name(&self) -> &String {
            &self.name
        }
        pub fn get_value_type(&self) -> &ValueType {
            &self.value_type
        }
        pub fn is_optional(&self) -> bool {
            self.optional
        }
    }

    trait DelimitedSpec: Spec{
        fn set_delimiter(&mut self, delimiter: Separator) ;
    }

    struct StringSpec{
        spec_meta_data: SpecMetaData,
        until: Separator,
    }

    impl DelimitedSpec for StringSpec {
        fn set_delimiter(&mut self, delimiter: Separator)  {
            self.until = delimiter;
        }
    }

    enum RepeatCount{
        Fixed(u32),
        Delimited(Separator),
    }

    struct RepeatManySpec {
        spec_meta_data: SpecMetaData,        
        repeat_count: RepeatCount,
        constituents: ListSpec,
    }

    impl DelimitedSpec for RepeatManySpec {
        fn set_delimiter(&mut self, delimiter: Separator)  {
            self.repeat_count = RepeatCount::Delimited(delimiter);
        }
    }

    impl Spec for RepeatManySpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    #[async_trait]
    impl Parse for RepeatManySpec {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut dyn SpecRead,
        ) -> Result<Value, ParserError> {
            // Implementation for parsing repeat many spec
            let mut repeat_count = 0;
            loop{
                self.constituents.mark_and_parse(info_provider, reader).await?;
                repeat_count += 1;
                if let RepeatCount::Fixed(count) = &self.repeat_count {
                    if repeat_count >= *count {
                        break;
                    }
                } else if let RepeatCount::Delimited(ref delimiter) = &self.repeat_count {
                    let result = StringSpec::new("".to_owned(), delimiter.clone(), false).mark_and_parse(info_provider, reader).await;
                    //let next_value = reader.read_placeholder_until(delimiter.to_owned(), self.get_meta_data()).await?;
                    if result.is_ok(){
                        break;
                    }
                }

            }

            Ok(Value::None) // Return appropriate value based on parsing
            
        }
    }


    pub trait Spec: Parse + Send + Sync{
        fn get_meta_data(&self) -> &SpecMetaData;
    }

    impl Spec for StringSpec {
        fn get_meta_data(&self)-> &SpecMetaData {
            &self.spec_meta_data
        }
    }

 /*    trait SimpleValueSpecBuilder{



    } */

    struct CompositeKeyType;

    struct CompositeValueType;

    struct CompositeInlineKeyType;

    struct CompositeBuilder<T>(T);

    impl CompositeBuilder<CompositeKeyType>{

    }

    impl StringSpec{
        fn new(name: String, delimiter: Separator,  optional: bool) -> Self {
            StringSpec {                
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional),
                until: delimiter,
            }
        }
    }

    #[async_trait]
    impl Parse for StringSpec 
    {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut dyn SpecRead,
        ) -> Result<Value, ParserError>      
        {
            //let mut buf = vec![];
             let value = match self.until {
                Separator::Delimiter(ref delimiter) => {
                    reader.read_placeholder_until(delimiter.to_owned(), self.get_meta_data()).await?
                }
                Separator::NBytes(size) => {
                    reader.read_bytes( ReadBytesSize::Fixed(size), self.get_meta_data()).await?
                }
                Separator::EndOfStream => {
                    reader.read_bytes(ReadBytesSize::Full, self.get_meta_data()).await?
                }
            };

            if let Some(value) = value {
                return Ok(ValueType::parse(&self.get_meta_data().value_type, &value));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read value for placeholder: {:?}",
                    self.get_meta_data().name
                )))
            }
        }
    }

    struct ExactStringSpec {
        input: String,
        spec_meta_data: SpecMetaData,
    }

    impl ExactStringSpec{
        fn new(name: String, input: String, optional: bool) -> Self {
            ExactStringSpec {
                input,
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional),
            }
        }
    }

    impl Spec for ExactStringSpec {
        
        fn get_meta_data(&self)-> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    #[async_trait]
    impl Parse for ExactStringSpec{
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut dyn SpecRead,
        ) -> Result<Value, ParserError>
        {
            let value = reader.read_placeholder_as_string(self.input.clone(), self.get_meta_data()).await?;
            if let Some(value) = value {
                return Ok(ValueType::parse(&self.get_meta_data().value_type, &value));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read exact string for placeholder: {:?}",
                    self.get_meta_data().get_name()
                )))
            }
        }
    }

    struct KeyValueSpec {
        spec_metadata: SpecMetaData,
        key: Key,
        value: ValueSpec,
    }

    impl  Spec for KeyValueSpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_metadata
        }
    }

    #[async_trait]
    impl Parse for KeyValueSpec{
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
            
        ) -> Result<Value, ParserError>
        {
            let key_name = self.key.mark_and_parse(info_provider, reader).await?;
            if let Value::String(key) = key_name {
                let value = self.value.mark_and_parse(info_provider, reader).await?;
                info_provider.add_info(key, value);
                return Ok(Value::None);
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read key-value pair for placeholder: {:?}",
                    self.get_meta_data().get_name()
                )))
            }
        }
    }

    struct NBytesSpec {
        spec_meta_data: SpecMetaData,
        size: u32,
    }

    impl NBytesSpec{
        pub fn new(name: String, size: u32, optional: bool) -> Self {
            NBytesSpec {
                spec_meta_data: SpecMetaData::new(name, ValueType::U8Vec, optional),
                size,
            }
        }
    }

    impl Spec for NBytesSpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    #[async_trait]
    impl  Parse for NBytesSpec {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>
        {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(self.size), self.get_meta_data()).await?;
            if let Some(bytes) = bytes {
                return Ok(ValueType::parse(&self.get_meta_data().value_type, &bytes));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read {} bytes for placeholder: {:?}",
                    self.size, self.get_meta_data().get_name()
                )))
            }
        }
    }

    struct AllBytesSpec {     
        spec_meta_data: SpecMetaData,           
    }

    impl Spec for AllBytesSpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    #[async_trait]
    impl Parse for AllBytesSpec {    
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            let bytes = reader.read_bytes(ReadBytesSize::Full, self.get_meta_data()).await?;
            if let Some(bytes) = bytes {
                return Ok(ValueType::parse(&self.get_meta_data().get_value_type(), &bytes));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read {} bytes for placeholder: {:?}",
                    "remaining ", self.get_meta_data().name
                )))
            }
        }
    }

/*     struct Spec<T:Parse + Send + Sync>{
        name: String,
        spec: T,
    } */



 /*    macro_rules! create_composite_parser_fields {
        ([$($args:ident),*]) => {
            paste!  {
                $( [<$args:lower>]:$args ),*
            }
        };
    } */

    macro_rules! create_composite_parser {
        
        ([$($args:ident),*], $arg_size:expr) => {
            paste::paste! {
                
                struct [< CompositeParser $arg_size>]<$($args),*> where
                    $($args: Parse + Send + Sync),* {
                        //create_composite_parser_fields!([$($args),*]),
                        $( [<$args:lower>]:$args ),*
                }


                #[async_trait]
                impl <$($args),*> Parse for [< CompositeParser $arg_size>]<$($args),*> where
                $($args: Parse + Send + Sync),*  {
                    async fn parse<IP, B>(
                        &self,
                        info_provider: &mut IP,
                        reader: &mut B,
                        spec: &Placeholder,
                    ) -> Result<ValueType, ParserError>
                    where
                        IP: InfoProvider + Send + Sync,
                        B: PlaceHolderRead + MarkAndRead + AsyncRead + Unpin + Send + Sync,
                    {
                        paste::paste! {
                            $(self.[<$args:lower>].mark_and_parse(info_provider, reader, spec).await?;)*
                            Ok(ValueType::None) // or some other appropriate return value
                        }
                    }
                }
            }
        }
    }


/*     create_composite_parser!([T1], 1);
    create_composite_parser!([T1, T2], 2);
    create_composite_parser!([T1, T2, T3], 3);
    create_composite_parser!([T1, T2, T3, T4], 4);
    create_composite_parser!([T1, T2, T3, T4, T5], 5); */

    trait Parent{
        fn add_child(&mut self, s: Box<dyn Spec>) ;
    }

    /* trait SingleChildSpec<T>{
        fn set_child(&mut self, s: T);
    } */

    impl Parent for Key{
        fn add_child(&mut self, s: Box<dyn Spec>)  {
            self.0 = s;
        }
    }

    impl  Parent for ListSpec{
        fn add_child(&mut self, s: Box<dyn Spec>)  {
            self.add_spec(s);
        }
    }

    struct OneOfSpec{
        spec_meta_data: SpecMetaData,
        values: Vec<String>,        
        until: Separator,
    }

    impl DelimitedSpec for OneOfSpec {
        fn set_delimiter(&mut self, delimiter: Separator)  {
            self.until = delimiter;
        }
    }

    impl Spec for OneOfSpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    #[async_trait]
    impl Parse for OneOfSpec {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>
        {
            let result = StringSpec::new(
                format!("{}-expect-one-of", self.get_meta_data().get_name()),
                self.until.clone(),
                self.get_meta_data().is_optional(),)
                .parse(info_provider, reader).await?;
            if let Some(value) = &result.get_string_value() {
                if self.values.contains(value) {
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

    

    #[derive(Default)]
    pub struct ListSpec{            
        spec_meta_data: SpecMetaData,
        constituents: Vec<Box<dyn Spec>>,  
        //phantom: PhantomData<T>,
    }

    

    #[async_trait]
    impl Parse for ListSpec {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            for constituent in &self.constituents {                
                constituent.mark_and_parse(info_provider, reader).await?;
            }
            Ok(Value::None) // or some other appropriate return value
        }
    }

    impl ListSpec {
        pub fn new(name: String, value_type: ValueType, optional: bool) -> Self {
            ListSpec {
                spec_meta_data: SpecMetaData::new(name, value_type, optional),
                constituents: Vec::new(),
            }
        }

        pub fn add_spec(&mut self, constituent: Box<dyn Spec> ) {
            self.constituents.push(constituent);
        }
    }

    impl Spec for ListSpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }
    

    struct ValueSpec(Box<dyn Spec>, SpecMetaData);

    impl Spec for ValueSpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.1
        }
    }

    struct InlineKeyWithValue(Box<dyn Spec>, String, SpecMetaData);

    impl Spec for InlineKeyWithValue {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.2
        }
    }
    
    struct Key(Box<dyn Spec>, SpecMetaData) ;

    impl Spec for Key {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.1
        }
    }

    /* impl <P: Parse + Send + Sync> Parent<P> for Key<P>{
        fn add_child(&mut self, s: P) {
            self.0 = s;
        }
    } */

    #[async_trait]
    impl Parse for Key

    {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            self.0.mark_and_parse(info_provider, reader).await
        }
    }

    #[async_trait]
    impl Parse for InlineKeyWithValue
    {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            self.0.mark_and_parse(info_provider, reader).await.map(|value| {
                info_provider.add_info(self.1.clone(), value);
                Value::None // or some other appropriate return value
            })
        }
    }

    #[async_trait]
    impl Parse for ValueSpec {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            self.0.mark_and_parse(info_provider, reader).await
        }
    }

    
    

    

    

    
        /* impl RequestParse for HttpParser {;
        async fn parse_request<RI: RequestInfo, Reader: AsyncRead + Unpin>(
            &self,
            reader: Reader,
            request_info:&mut RI,
            request_spec: &Placeholder,

        ) -> Result<RI, ParserError> {
            let protocol_reader = crate::core::protocol_reader::ProtocolBuffReader::new(BufReader::new(reader), 1024);
            protocol_reader.parse_composite(request_info, request_spec).await?;
            Ok(())

        }
    } */

    //struct SpecPlaceHolderParser<T>;

    /*impl <T> Future for SpecPlaceHolderParser<T>{
        type Output = T;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        }
    }

    struct ReadStringUntil<R:AsyncRead + Unpin>{
        reader: BufReader<R>,
        until: u8,
    }

    impl <R:AsyncRead + Unpin> Future for ReadStringUntil<R>{
        type Output = String;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let  buffer = self.reader.buffer();
            let mut buf = vec![];
            self.reader.read_until( self.until,  &mut buf);
            if buffer.len() == 0 {
                ready!(Pin::new(& mut self.reader.read_line()).poll_fill_buf(cx));
            }
        }
    }*/

    struct NumberU64Spec(SpecMetaData) ;

    struct NumberI64Spec(SpecMetaData) ;

    struct NumberU32Spec(SpecMetaData) ;

    struct NumberU16Spec(SpecMetaData) ;

    struct NumberI16Spec(SpecMetaData) ;

    trait SimpleTypeSpecBuilder: StringSpecBuilder + NumberSpecBuilder{}

    trait NumberSpecBuilder{
        fn expect_u16(name: String, optional: bool) -> NumberU16Spec{
            NumberU16Spec(SpecMetaData::new(name, ValueType::UnSignedNumber16, optional))
        }

        fn expect_u32(name: String, optional: bool) -> NumberU32Spec{
            NumberU32Spec(SpecMetaData::new(name, ValueType::UnSignedNumber32, optional))
        }

        fn expect_u64(name: String, optional: bool) -> NumberU64Spec{
            NumberU64Spec(SpecMetaData::new(name, ValueType::UnSignedNumber64, optional))
        }


        fn expect_i16(name: String, optional: bool) -> NumberI16Spec{
            NumberI16Spec(SpecMetaData::new(name, ValueType::SignedNumber16, optional))
        }

        

        fn expect_i64(name: String, optional: bool) -> NumberI64Spec{
            NumberI64Spec(SpecMetaData::new(name, ValueType::SignedNumber64, optional))
        }
    }

    trait KeyBuilder: StringSpecBuilder{
    }

    trait ValueBuilder: StringSpecBuilder + NumberSpecBuilder + ByteSpecBuilder{
    }

    trait InlineKeyWithValueBuilder: ValueBuilder{
    }

    struct KeyBuilderData<'a, T:Parent>(&'a mut T, SpecMetaData);

    struct ValueBuilderData<'a, T:Parent>(&'a mut T, SpecMetaData);

    impl <'a, T:Parent> Parent for KeyBuilderData<'a, T> {
        fn add_child(&mut self, s: Box<dyn Spec>) {
            let key = Key(s, self.1.clone());
            self.0.add_child(Box::new(key));
        }
    }

    impl <'a, T:Parent> Parent for ValueBuilderData<'a, T> {
        fn add_child(&mut self, s: Box<dyn Spec>) {
            let key = ValueSpec(s, self.1.clone());
            self.0.add_child(Box::new(key));
        }
    }

    impl <'a, P:Parent> StringSpecBuilder for KeyBuilderData<'a, P> {
    }

    impl <'a, P:Parent> StringSpecBuilder for ValueBuilderData<'a, P> {
    }

    impl <'a, P:Parent> NumberSpecBuilder for ValueBuilderData<'a, P> {
    }

    impl <'a, P:Parent> ByteSpecBuilder for ValueBuilderData<'a, P> {
    }

    impl <'a, P:Parent> ValueBuilder for ValueBuilderData<'a, P> {
    }



    trait StringSpecBuilder {

        fn expect_string<'a,>(&'a mut self, name: String, optional: bool) -> impl DelimiterSpecBuilder<'a, Self> where  Self:Parent + Sized, //impl DelimiterSpecBuilder<'a, P> where P: Parent, Self:Parent + Sized, 
        // -> 
        {
            DelimitedItemBuilder(self, StringSpec { spec_meta_data: SpecMetaData::new(name, ValueType::String, 
                false), until: Separator::EndOfStream } )
            
        }

        fn expect_one_of_string<'a,>(&'a mut self, name: String, optional: bool, options: Vec<String>) -> impl DelimiterSpecBuilder<'a, Self>  where  Self:Parent + Sized, //impl DelimiterSpecBuilder<'a, P> where P: Parent, Self:Parent + Sized, 
        // -> 
        {
            DelimitedItemBuilder(self, OneOfSpec { spec_meta_data: SpecMetaData::new(name, ValueType::String, 
                false), until: Separator::EndOfStream,  values: options } )
            
        }

        fn expect_exact_string(
            &mut self,
            name: String, 
            input: String,
            optional: bool,
        ) -> &mut Self where Self:Parent + Sized{
            let exact_string = ExactStringSpec::new(name, input, optional);
            self.add_child(Box::new(exact_string));
            self

        }
    }





    
    #[async_trait]
    impl Parse for NumberU64Spec {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(8), &self.0).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::UnSignedNumber64, &bytes))
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name()
                )))
            }
        }
    }

    #[async_trait]
    impl Parse for NumberI64Spec {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(8), &self.0).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::SignedNumber64, &bytes))
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name()
                )))
            }
        }
    }

    #[async_trait]
    impl Parse for NumberU32Spec {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4), &self.0).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::UnSignedNumber32, &bytes))
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name()
                )))
            }
        }
    }

    #[async_trait]
    impl Parse for NumberU16Spec {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4), &self.0).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::UnSignedNumber16, &bytes))
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name()
                )))
            }
        }
    }

    #[async_trait]
    impl Parse for NumberI16Spec {
        async fn parse(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4), &self.0).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::SignedNumber16, &bytes))
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name()
                )))
            }
        }
    }

    

    trait DelimiterSpecBuilder<'a, T: Parent> {
        fn expect_newline(self) -> &'a mut T;
        fn expect_delimiter(self, delimiter: String) ->  &'a mut T;
        fn expect_space(self) ->  &'a mut T;
    }

    struct ValueBuilder<'a, T>(&'a mut T, SpecMetaData);

    

    /* impl <'a,T> ValueBuilder<'a, T> where  T:ProtocolSpecBuilder{
        fn expect_string_value(&mut self, name:String, ) ->StringBuilder<T>{
            StringBuilder(self.0, StringSpec { spec_meta_data: SpecMetaData::new(name, ValueType::String, false), until: Separator::EndOfStream })
        }

        fn expect_u64_value(&mut self, name:String, optional:bool  ) -> & mut T{
            self.0.add_spec(ValueSpec(NumberU64Spec(
                SpecMetaData::new(name, ValueType::UnSignedNumber64, 
                false)), SpecMetaData::new(
                    "Value ".to_owned(), ValueType::None, optional)));
            self.0
        }

        fn expect_i64_value(&mut self, name:String, optional:bool  ) -> & mut T{
            self.0.add_spec(ValueSpec(NumberI64Spec(
                SpecMetaData::new(name, ValueType::SignedNumber64, 
                false)), SpecMetaData::new(
                    "Value ".to_owned(), ValueType::None, optional)));
            self.0
        }
    } */

    


    struct DelimitedItemBuilder<'a, T:Parent, S:DelimitedSpec>(&'a mut T, S);

    struct RepeatManyBuilder<'a, T:Parent, S:DelimitedSpec>(&'a mut T, S);
    

    trait OneOfStringSpecBuilder<'a, T> {
        fn expect_one_of_string(
            self, 
            name: String, 
            one_of: Vec<String>,            
            optional: bool,
        ) -> &'a mut T;
    }

    /* impl <'a, T:Parent, S:Spec> OneOfStringSpecBuilder<'a, T> for StringBuilder<'a, T, S>{
        fn expect_one_of_string(
            mut self, 
            name: String, 
            one_of: Vec<String>,            
            optional: bool,
        ) -> &'a mut T {
            let mut spec = ExpectOneOfSpec {
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional),
                values: one_of,
                until: Separator::Delimiter(" ".to_owned()),
            };
            self.0.add_child(Box::new(spec));
            &mut self
        }
    }  */

    impl <'a, T:Parent, S:DelimitedSpec + 'static> DelimiterSpecBuilder<'a, T> for DelimitedItemBuilder<'a, T, S>{
        fn expect_newline(mut self) -> &'a mut T {
            let mut spec = self.1;
            spec.set_delimiter(Separator::Delimiter("\r\n".to_owned()));
            self.0.add_child(Box::new(spec));
            self.0
        }

        /* fn expect_newline(mut self) -> T {
            self.1.until = Separator::Delimiter("\r\n".to_owned());
            self.0.add_spec(self.1);
            self.0
        } */
    
        fn expect_delimiter(mut self, delimiter: String) -> &'a mut T {
            let mut spec = self.1;
            spec.set_delimiter(Separator::Delimiter("\r\n".to_owned()));
            self.0.add_child(Box::new(spec));
            self.0
        }
    
        fn expect_space(mut self) -> &'a mut T {
            let mut spec = self.1;
            spec.set_delimiter(Separator::Delimiter(" ".to_owned()));
            self.0.add_child(Box::new(spec));
            self.0
        }
    }

    pub trait ByteSpecBuilder{
        fn expect_bytes_of_size(&mut self, name: String, size:u32, optional: bool) -> NBytesSpec{
            NBytesSpec::new(name, size, optional)
        }
    }

    trait BuilderState{        
        fn get_shared_state_data(&self) -> &SharedStateData;
        fn get_mut_shared_state_data(&mut self) -> &mut SharedStateData;
        fn get_owned_shared_state_data(self) -> SharedStateData;
    }

    enum PlaceHolderType{
        Key, Value, InlineValue, None
    }

    struct BuildFromScratch{
        common_data: SharedStateData,
    }

    struct BuildKey{
        common_data: SharedStateData,
    }

    struct BuildValue{
        key_spec: Key,
        value_spec: Option<ValueSpec>,
        common_data: SharedStateData,
    }

    

    struct BuildString {     
        common_data: SharedStateData,
    }

    struct BuildDelimiter<D:DelimitedSpec>{
        common_data: SharedStateData,
        delimiter_spec: D,
    }

    struct BuildInlineValue{
        common_data: SharedStateData,
        
    }

    struct SharedStateData {
        composite_spec: ListSpec,
        current_key: Option<Key>,
        current_value: Option<ValueSpec>,
    }

    impl  SharedStateData {
        fn add_spec(&mut self, spec: Box<dyn Spec>) {
            self.composite_spec.add_spec(spec);
        }
    }

    impl BuilderState for BuildFromScratch {
        fn get_shared_state_data(&self) -> &SharedStateData {
            &self.common_data
        }
        
        fn get_mut_shared_state_data(&mut self) -> &mut SharedStateData {
            &mut self.common_data
        }

        fn get_owned_shared_state_data(self) -> SharedStateData {
            self.common_data
        }
    }

    impl BuilderState for BuildKey {
        fn get_shared_state_data(&self) -> &SharedStateData {
            &self.common_data
        }
        
        fn get_mut_shared_state_data(&mut self) -> &mut SharedStateData {
            &mut self.common_data
        }

        fn get_owned_shared_state_data(self) -> SharedStateData {
            self.common_data
        }
    }
    impl BuilderState for BuildValue {
        fn get_shared_state_data(&self) -> &SharedStateData {
            &self.common_data
        }
        
        fn get_mut_shared_state_data(&mut self) -> &mut SharedStateData {
            &mut self.common_data
        }

        fn get_owned_shared_state_data(self) -> SharedStateData {
            self.common_data
        }
    }     
    impl BuilderState for BuildInlineValue {
        fn get_shared_state_data(&self) -> &SharedStateData {
            &self.common_data
        }
        
        fn get_mut_shared_state_data(&mut self) -> &mut SharedStateData {
            &mut self.common_data
        }
        
        fn get_owned_shared_state_data(self) -> SharedStateData {
            self.common_data
        }

    }
    impl BuilderState for BuildString {
        fn get_shared_state_data(&self) -> &SharedStateData {
            &self.common_data
        }
        
        fn get_mut_shared_state_data(&mut self) -> &mut SharedStateData {
            &mut self.common_data
        }

        fn get_owned_shared_state_data(self) -> SharedStateData {
            self.common_data
        }
    }
    impl <D:DelimitedSpec > BuilderState for BuildDelimiter<D> {
        fn get_shared_state_data(&self) -> &SharedStateData {
            &self.common_data
        }
        
        fn get_mut_shared_state_data(&mut self) -> &mut SharedStateData {
            &mut self.common_data
        }

        fn get_owned_shared_state_data(self) -> SharedStateData {
            self.common_data
        }
    }
/* 
    trait StringStateSpecBuilder {
        key().
        fn expect_string<'a,>(&'a mut self, name: String, optional: bool) -> 
        // -> 
        {
            DelimitedItemBuilder(self, StringSpec { spec_meta_data: SpecMetaData::new(name, ValueType::String, 
                false), until: Separator::EndOfStream } )
            
        }

        fn expect_one_of_string<'a,>(&'a mut self, name: String, optional: bool, options: Vec<String>) -> impl DelimiterSpecBuilder<'a, Self>  where  Self:Parent + Sized, //impl DelimiterSpecBuilder<'a, P> where P: Parent, Self:Parent + Sized, 
        // -> 
        {
            DelimitedItemBuilder(self, OneOfSpec { spec_meta_data: SpecMetaData::new(name, ValueType::String, 
                false), until: Separator::EndOfStream,  values: options } )
            
        }

        fn expect_exact_string(
            &mut self,
            name: String, 
            input: String,
            optional: bool,
        ) -> &mut Self where Self:Parent + Sized{
            let exact_string = ExactStringSpec::new(name, input, optional);
            self.add_child(Box::new(exact_string));
            self

        }
    } */


    trait ProtoSpecBuilder<S:BuilderState>{        
    }

    struct ProtoSpecBuilderData<S:BuilderState>{
        //composite_spec: ListSpec,
        state: S,
        /* current_key: Option<Key>,
        current_value: Option<ValueSpec>, */
    }

    impl <S> ProtoSpecBuilderData<S> where S:BuilderState {
        pub fn new(state: S) -> Self {
            ProtoSpecBuilderData {
                //composite_spec,
                state,
                //current_key: None,
                //current_value: None,
            }
        }

        pub fn add_spec(&mut self, spec: Box<dyn Spec>) {
            self.state.get_mut_shared_state_data().add_spec(spec);
        }
    }

    
    impl From<ProtoSpecBuilderData<BuildFromScratch>> for ProtoSpecBuilderData<BuildKey> {
        fn from(builder: ProtoSpecBuilderData<BuildFromScratch>) -> Self {
            ProtoSpecBuilderData {                
                state: BuildKey{
                    common_data: builder.state.common_data,
                },                
            }
        }
    }

    impl SharedStateData {
        fn expect_string<'a,>(self, name: String, optional: bool) -> ProtoSpecBuilderData<BuildDelimiter<StringSpec>>
        //where Self:Parent + Sized, //impl DelimiterSpecBuilder<'a, P> where P: Parent, Self:Parent + Sized, 
        {
            let spec = StringSpec { 
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional), 
                until: Separator::EndOfStream 
            };

            ProtoSpecBuilderData{
                state:BuildDelimiter{
                    common_data: self,
                    delimiter_spec: spec,
                },
            }

            //self.0.add_spec(Box::new(spec));
            //ProtoSpecBuilderData(self.0, BuildDelimiter(spec)/* , self.2.clone(), self.3.clone() */)            
            /* ProtoSpecBuilderData(
                state:BuildDelimiter{
                    common_data: self.get_mut_common_data().clone(),
                    delimiter_spec: spec,
                },
            ) */
        }
    }

    impl <S:BuilderState> From<S> for ProtoSpecBuilderData<S> {
        fn from(builder: S) -> Self {
            ProtoSpecBuilderData {
                state: builder,
            }
        }
    }

    impl StringSpecBuilder1 for  BuilderState1 {
        fn expect_string(self, name: String, optional: bool) -> BuildDelimiter<StringSpec> {
            let spec = StringSpec { 
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional), 
                until: Separator::EndOfStream 
            };

            BuildDelimiter{
                common_data: self.get_owned_shared_state_data(),
                delimiter_spec: spec,
            }
        }
    }

    pub trait StringSpecBuilder1:BuilderState {
        fn expect_string( self, name: String, optional: bool) ->  BuildDelimiter<StringSpec> where Self: Sized, {
            let spec = StringSpec { 
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional), 
                until: Separator::EndOfStream 
            };

            
            BuildDelimiter{
                common_data: self.get_owned_shared_state_data(),
                delimiter_spec: spec,
            }
        }
    }

        fn expect_one_of_string(self, name: String, optional: bool, options: Vec<String>) ->  BuildDelimiter<StringSpec> where Self: Sized,  {
            let one_of_spec = OneOfSpec { spec_meta_data: SpecMetaData::new(name, ValueType::String, 
                false), until: Separator::EndOfStream,  values: options };
            BuildDelimiter{
                common_data: self.get_owned_shared_state_data(),
                delimiter_spec: one_of_spec,
            }

        }

        fn expect_exact_string(self, name: String, input: String, optional: bool) -> BuildFromScratch {
            let exact_string = ExactStringSpec::new(name, input, optional);
            self.get_mut_shared_state_data().add_spec(Box::new(exact_string));
            BuildFromScratch {
                common_data: self.get_owned_shared_state_data(),
            }
        }
    }

    pub trait NumberSpecBuilder1 {
        fn expect_u16(name: String, optional: bool) -> NumberU16Spec;
        fn expect_u32(name: String, optional: bool) -> NumberU32Spec;
        fn expect_u64(name: String, optional: bool) -> NumberU64Spec;
        fn expect_i16(name: String, optional: bool) -> NumberI16Spec;
        fn expect_i64(name: String, optional: bool) -> NumberI64Spec;
    }

    trait DelimiterSpecBuilder1{
        fn expect_newline(self) -> impl StringSpecBuilder1;
        fn expect_delimiter(self, delimiter: String) -> impl StringSpecBuilder1;
        fn expect_space(self) -> impl StringSpecBuilder1;
    }

    impl <D> DelimiterSpecBuilder1 for ProtoSpecBuilderData<BuildDelimiter<D>> where D:DelimitedSpec {
        fn expect_newline(self) -> impl StringSpecBuilder1 {
            let mut spec = StringSpec { 
                spec_meta_data: SpecMetaData::new("newline".to_owned(), ValueType::String, false), 
                until: Separator::Delimiter("\r\n".to_owned()) 
            };
            spec.set_delimiter(Separator::Delimiter("\r\n".to_owned()));
            ProtoSpecBuilderData(self.0, BuildDelimiter(spec))
        }

        fn expect_delimiter(self, delimiter: String) -> impl StringSpecBuilder1 {
            let mut spec = StringSpec { 
                spec_meta_data: SpecMetaData::new("delimiter".to_owned(), ValueType::String, false), 
                until: Separator::Delimiter(delimiter) 
            };
            spec.set_delimiter(Separator::Delimiter(delimiter));
            ProtoSpecBuilderData(self.0, BuildDelimiter(spec))
        }

        fn expect_space(self) -> impl StringSpecBuilder1 {
            let mut spec = StringSpec { 
                spec_meta_data: SpecMetaData::new("space".to_owned(), ValueType::String, false), 
                until: Separator::Delimiter(" ".to_owned()) 
            };
            spec.set_delimiter(Separator::Delimiter(" ".to_owned()));
            ProtoSpecBuilderData(self.0, BuildDelimiter(spec))
        }
        
    }

    impl ProtoSpecBuilderData<BuildFromScratch> {
        pub fn key<S>(self) -> ProtoSpecBuilderData<BuildKey<S>> where S:Spec {
            self.into()
        }
    }

    impl <S> ProtoSpecBuilderData<BuildKey<S>> where S:Spec {
        pub fn value(self, spec: S) -> ProtoSpecBuilderData<BuildValue> {
            ProtoSpecBuilderData {
                composite_spec: self.0,
                state: BuildValue {
                    key_spec: Key(spec, SpecMetaData::new("key".to_owned(), ValueType::String, false)),
                    value_spec: None,
                },
                current_key: Some(Key(spec, SpecMetaData::new("key".to_owned(), ValueType::String, false))),
                current_value: None,
            }
        }
    }

    impl StringSpecBuilder1 for  ProtoSpecBuilderData<BuildFromScratch> {
        
        fn expect_string(self, name: String, optional: bool) -> impl DelimiterSpecBuilder1
        {
            self.state.get_mut_shared_state_data().expect_string(name, optional)
        }

        fn expect_one_of_string(self, name: String, optional: bool, options: Vec<String>) -> impl DelimiterSpecBuilder1
        // -> 
        {
            let one_of_spec = OneOfSpec { spec_meta_data: SpecMetaData::new(name, ValueType::String, 
                false), until: Separator::EndOfStream,  values: options };
            ProtoSpecBuilderData(self.0, BuildDelimiter(one_of_spec))
        }

        fn expect_exact_string(
            mut self,
            name: String, 
            input: String,
            optional: bool,
        ) ->  impl StringSpecBuilder1{
            let exact_string = ExactStringSpec::new(name, input, optional);
            self.0.add_spec(Box::new(exact_string));
            self

        }
    }

    impl StringSpecBuilder for ProtoSpecBuilderData<BuildKey>{

    }

    impl StringSpecBuilder for ProtoSpecBuilderData<BuildKey>{

    }

    #[allow(dead_code)]
    pub trait ProtocolSpecBuilder:Parent + StringSpecBuilder + NumberSpecBuilder where Self:Sized {

        //fn expect_newstring(&mut self) -> &mut String;
        fn expect_newline(&mut self, name: String, optional:bool) -> &mut Self ;
        fn expect_delimiter(&mut self, name: String, delimiter: String, optional: bool) -> &mut Self;
        fn expect_space(&mut self, name: String,  optional: bool) -> &mut Self; 

        //fn add_key_parser<T>(&mut self, name: String, parser:T) -> &mut Self where T:Parse + Send + Sync;

        //fn add_value_parser<T>(&mut self, name: String, parser:T) -> &mut Self where T:Parse + Send + Sync;

        //fn add_inlinekey_value_parser<T>(&mut self, name: String, parser:T) -> &mut Self where T:Parse + Send + Sync;

        //fn expect_composite(&mut self, place_holder: Placeholder, name: String) -> &mut Self;
        /* fn expect_string<S:Spec>(&mut self, name: String, optional: bool)
            -> StringBuilder<S, Self> where Self:Sized; */

        /* fn expect_string(&mut self, name: String, optional: bool) -> StringBuilder<Self, StringSpec> where Self:Sized;
        fn expect_exact_string(
            &mut self,
            name: String, 
            input: String,
            optional: bool,
        ) -> &mut Self; */

        

        /* fn expect_one_of_string(
            &mut self,
            name: String, 
            one_of: Vec<String>,            
            optional: bool,
        ) -> &mut Self; */

        fn expect_repeat_many<'a>(&'a mut self, composite_spec: ListSpec, name: String) -> impl DelimiterSpecBuilder<'a, Self>;
        fn expect_repeat_n<'a>(
            &'a mut self,
            composite_spec: ListSpec,
            repeat_count: u32,
            name: String,
        ) -> impl DelimiterSpecBuilder<'a, Self>;

        /* fn expect_stream(
            &mut self,            
            name: String,
            optional: bool,
        ) -> &mut Self; */

        //fn expect_key_string(&mut self, identifier: PlaceHolderIdentifier) -> &mut Self;

        //fn expect_value_string(&mut self, name: String, optional: bool) -> &mut Self;

        fn key(&mut self) -> impl Parent {
            KeyBuilderData(self, SpecMetaData::new("key".to_owned(), ValueType::String, false))
        }

        fn add_spec<T:Spec + 'static>(&mut self, constituent: T) -> &mut Self;

        fn build(&mut self) -> ListSpec;
    }

    

    /* impl Placeholder {
        pub fn add_composite_place_holder(&mut self, place_holder: Placeholder) {
            /* match self.constituents {
                None => {
                    let mut vec = vec![];
                    vec.push(place_holder);
                    self.constituents = Some(vec);
                }
                Some(ref mut place_holders) => {
                    place_holders.push(place_holder);
                }
            } */
        }
    } */

    //#[derive(Default)]
    pub struct SpecBuilder(pub ListSpec);

    impl Parent for SpecBuilder {
        fn add_child(&mut self, s: Box<dyn Spec>) {
            self.0.add_spec(s);
        }
    }

    /* #[allow(dead_code)]
    impl <R:Parse> SpecBuilder<R> {
        pub fn new(r: R) -> Self {
            SpecBuilder(r)
        }
    } */

  impl StringSpecBuilder for SpecBuilder {
        fn expect_string(&mut self, name: String, optional: bool) -> DelimitedItemBuilder<Self, StringSpec> {
            DelimitedItemBuilder(self, StringSpec::new(name, Separator::Delimiter(" ".to_owned()), optional))
        }
    }

    impl NumberSpecBuilder for SpecBuilder {        
    }

    impl ByteSpecBuilder for SpecBuilder {
        /* fn expect_bytes_of_size(&mut self, name: String, size:u32, optional: bool) -> &mut Self {
            self.0.add_spec(Box::new(NBytesSpec::new(name, size, optional)));
            self
        } */
    }

    impl ProtocolSpecBuilder for SpecBuilder{
        fn expect_newline(&mut self, name: String, optional:bool) -> &mut Self {
            self.0.add_spec(Box::new(ExactStringSpec::new(
                name,
                "\r\n".to_owned(),
                optional,
            )));
            return self;
        }

        fn expect_delimiter(&mut self, name: String, delimiter: String, optional: bool) -> &mut Self {
            self.0.add_spec(Box::new(ExactStringSpec::new(
                name,
                delimiter, 
                optional,
            )));
            return self;
        }

        fn expect_space(&mut self, name: String,  optional: bool) -> &mut Self {
            self.0.add_spec(Box::new(ExactStringSpec::new(
                "space".to_string(), 
                ' '.to_string(),
                optional,
            )));
            return self;
        }

        /* fn expect_composite(&mut self, place_holder: Placeholder, name: String) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                name,
                PlaceHolderType::Composite,
                false,
            ));
            return self;
        } */

        /* fn expect_string(&mut self, name: String, optional: bool) -> StringBuilder<Self, StringSpec> {
            
            StringBuilder(self, StringSpec::new(
                name,
                Separator::Delimiter(" ".to_owned()),
                optional,
            ))
            //return self;
        } */

        /* fn expect_exact_string(
            &mut self,
            name: String,
            input: String,
            optional: bool,
        ) -> &mut Self {
            self.0.add_spec(Box::new(ExactStringSpec::new(
                name,
                input,
                optional,
            )));
            return self;
        } */

        /* fn expect_key_string(&mut self, id: PlaceHolderIdentifier) -> &mut Self {
            self.0.add_place_holder(Placeholder::new_key_placeholder(
                id,
                None,
                PlaceHolderType::AnyString,
            ));
            return self;
        } */

        /* fn expect_value_string(&mut self, name: String, optional: bool) -> &mut Self {
            self.0.add_spec(Placeholder::new_value_placeholder(
                String::new(),
                PlaceHolderType::AnyString,
                optional,
            ));
            return self;
        } */

        /* fn expect_one_of_string(
            &mut self,
            name: String,
            one_of: Vec<String>,            
            optional: bool,
        ) -> &mut Self {
            self.0.add_spec(Placeholder::new(
                String::new(),
                
                PlaceHolderType::OneOf(one_of),
                optional,
            ));
            return self;
        } */

        /* fn expect_stream(
            &mut self,
            id: PlaceHolderIdentifier,
            name: String,
            optional: bool,
        ) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                String::new(),                
                PlaceHolderType::StreamValue(name),
                optional,
            ));
            return self;
        } */

        fn expect_repeat_many<'a>(&'a mut self, composite_spec: ListSpec, name: String) -> impl DelimiterSpecBuilder<'a, Self> {
            let repeat_many = RepeatManySpec {
                spec_meta_data: SpecMetaData::new(name, ValueType::None, false),
                constituents: composite_spec,
                repeat_count: RepeatCount::Fixed(0),
            };
            DelimitedItemBuilder(self, repeat_many)
            //self.0.add_spec(Box::new(repeat_many));
            //return self;
        }

        fn expect_repeat_n<'a>(
            &'a mut self,
            composite_spec: ListSpec,
            repeat_count: u32,
            name: String,
        ) -> impl DelimiterSpecBuilder<'a, Self> {
            let repeat_many = RepeatManySpec {
                spec_meta_data: SpecMetaData::new(name, ValueType::None, false),
                constituents: composite_spec,
                repeat_count: RepeatCount::Fixed(repeat_count),
            };
            DelimitedItemBuilder(self, repeat_many)
        }

        fn build(&mut self) -> ListSpec {
            return mem::take(&mut self.0);
        }

        
        
        /* fn expect_bytes_of_size_from_header(&mut self, id: PlaceHolderIdentifier, header:String, optional:bool) -> &mut Self {
            self.0
                .add_place_holder(Placeholder::new(String::new(), PlaceHolderType::BytesOfSizeFromHeader(header), optional));
            return self;
        } */
        
        /* fn add_key_parser<T>(&mut self, parser:T) -> &mut Self where T:Parse + Send + Sync {
            self.0.
        }
        
        fn add_value_parser<T>(&mut self, parser:T) -> &mut Self where T:Parse + Send + Sync {
            todo!()
        }
        
        fn add_inlinekey_value_parser<T>(&mut self, parser:T) -> &mut Self where T:Parse + Send + Sync {
            todo!()
        } */
        
        fn add_spec<T:Spec + 'static>(&mut self, spec: T) -> &mut Self{
            self.0.add_spec(Box::new(spec));
            self
        }
    }


    pub mod parser {
        use super::*;
        use std::ops::Deref;

        /*pub trait Parser {
            fn parse<'a, T: RequestInfo<'a>>(&self, t: T) -> Result<(), ParserError>;
        }*/
        /* #[allow(unused)]
        async fn parse_request<P: RequestParse, T: RequestInfo, RequestStream>(
            request_stream: RequestStream,
            parser: P,
        ) -> Result<T, ParserError>
        where
            RequestStream: Unpin + AsyncRead,
        {
            let result = parser.parse_request(request_stream).await;
            return result;
        } */

        pub trait RequestValidator<T> {
            type Input: IntoIterator<Item = T>;
            #[allow(unused)]
            fn validate(&self, request_data: &Self::Input) -> Result<(), ParserError>;
        }

        #[allow(unused)]
        struct ExpectOneOf<T> {
            data: Vec<T>,
        }

        impl<'a, T> ExpectOneOf<T> {
            #[allow(unused)]
            pub fn new(data: Vec<T>) -> Self {
                ExpectOneOf { data }
            }
        }

        /* static HTTP_METHODS: ExpectOneOf<&str> = ExpectOneOf {
            data: &["GET", "POST", "PUT", "DELETE", "OPTIONS"],
        }; */

        /* static HTTP_METHODS: ExpectOneOf<&str> = ExpectOneOf {
            data: &["GET"]
        };*/

        impl<'a, T> RequestValidator<T> for ExpectOneOf<T>
        where
            T: Deref<Target = T> + PartialEq + Display,
        {
            type Input = Option<T>;

            fn validate(&self, request_data: &Self::Input) -> Result<(), ParserError> {
                for item in request_data.into_iter() {
                    if self.data.contains(&item) {
                        return Ok(());
                    } else {
                        return Err(ParserError::InvalidToken {
                            line_index: 0,
                            char_index: 0,
                            message: format!("Unexpected  tokens {}", item),
                        });
                    }
                }
                return Err(ParserError::InvalidToken {
                    line_index: 0,
                    char_index: 0,
                    message: format!(
                        "Expected one of these tokens {:?}",
                        slice_to_string(self.data.as_slice())
                    ),
                });
            }
        }

        fn slice_to_string<T>(slice: &[T]) -> String
        where
            T: ToString,
        {
            slice
                .iter()
                .map(|item| item.to_string())
                .reduce(|acc, item| format!("{},{}", acc, item))
                .unwrap()
        }

        struct ExpectAllOf<'a, T> {
            data: Vec<&'a T>,
        }

        impl<'a, T> ExpectAllOf<'a, T> {
            #[allow(dead_code)]
            pub fn new(data: Vec<&'a T>) -> Self {
                ExpectAllOf { data: data }
            }
        }

        impl<'a, T> RequestValidator<T> for ExpectAllOf<'a, T>
        where
            T: PartialEq + Display,
        {
            type Input = Option<T>;

            #[allow(unused)]
            fn validate(&self, request_data: &Self::Input) -> Result<(), ParserError> {
                let mut iter = request_data.into_iter();
                for item in &mut iter {
                    if !self.data.contains(&item) {
                        return Err(ParserError::InvalidToken {
                            line_index: 0,
                            char_index: 0,
                            message: format!("Unexpected  tokens {}", item),
                        });
                    }
                }
                return Ok(());
            }
        }

        #[allow(unused)]
        fn parse<R>(stream: R, placeholder: Placeholder, next_place_holder: Option<Placeholder>)
        where
            R: AsyncRead + Unpin,
        {
            match placeholder.place_holder_type {
                PlaceHolderType::AnyString => {
                                /*let buffer = buf_reader.buffer();
                    if(buffer.len() == 0){
                        let p = buf_reader.poll_fill_buf();
                        buf_reader.consume()
                    }*/
                            }
                PlaceHolderType::ExactString(input) => {}
                OneOf(_) => {}
                PlaceHolderType::Space => {}
                PlaceHolderType::NewLine => {}
                PlaceHolderType::Delimiter(_) => {}
                PlaceHolderType::Composite => {}
                PlaceHolderType::RepeatMany => {}
                PlaceHolderType::RepeatN(_) => {}
                PlaceHolderType::StreamValue(name) => todo!(),
                PlaceHolderType::BytesOfSizeFromHeader(_) => todo!(),
                PlaceHolderType::BytesOfSizeN(_) => todo!(),
                PlaceHolderType::Bytes => todo!(),
            }
        }
    }

    pub(crate) mod protocol_reader;
    mod protocol_writer;
}

pub mod http;
mod utils;

#[cfg(test)]
mod tests {
    use crate::core::{
        PlaceHolderIdentifier::{InlineKeyWithValue, Name},
        PlaceHolderType, Placeholder, ProtocolSpecBuilder, SpecBuilder,
    };

    fn test_string_placeholder(){
        let mut spec_builder = SpecBuilder(Placeholder::new(
            "Request".to_string(),            
            PlaceHolderType::Composite,
            false,
        ));
        let spec = spec_builder.expect_string(crate::core::PlaceHolderIdentifier::InlineKeyWithValue("test"), optional);
        //spec.
    }

    #[test]
    fn test_protocol_spec_builder() {
        let mut spec_builder = SpecBuilder(Placeholder::new(
            "Request".to_string(),            
            PlaceHolderType::Composite,
            false,
        ));

        spec_builder.expect_string(InlineKeyWithValue("request_method".to_string()), false);
    }
}

#[cfg(test)]
mod test_utils {
    use std::collections::HashMap;

    use crate::core::{InfoProvider, Value};

    pub fn assert_result_has_string(
        result: Result<Option<Vec<u8>>, crate::core::ParserError>,
        data: String,
    ) {
        match result {
            Ok(Some(result_data)) => {
                assert!(data == String::from_utf8(result_data).unwrap());
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[derive(Default)]
    pub struct TestRequestInfo(HashMap<String, Value>);

    impl TestRequestInfo {
        pub fn new() -> Self {
            TestRequestInfo(HashMap::new())
        }
    }

    impl InfoProvider for TestRequestInfo {
        fn add_info(&mut self, key: String, value: Value) {
            self.0.insert(key, value);
        }

        fn get_info(&self, key: &String) -> Option<&crate::core::Value> {
            self.0.get(key)
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
            } else {
                None
            }
        }
        
        fn has_all_data(&self) -> bool {
            todo!()
        }
    }
}
