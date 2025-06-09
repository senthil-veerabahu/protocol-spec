//#![debugger_visualizer(natvis_file = "./Foo.natvis")]


pub mod core {
    use crate::core::PlaceHolderIdentifier::Name;
    use crate::core::PlaceHolderType::OneOf;
    use async_trait::async_trait;
    use derive_builder::Builder;
    use protocol_reader::ProtocolBuffReader;

    use protocol_writer::ProtocolBuffWriter;

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

    impl<'a> ValueExtractor<'a> for ValueType {
        fn get_string_value(&self) -> Option<String> {
            return match &self {
                ValueType::String(ref data) => Some(data.clone()),
                ValueType::UnSignedNumber16(ref data) => Some(data.to_string()),
                ValueType::UnSignedNumber32(ref data) => Some(data.to_string()),
                ValueType::UnSignedNumber64(ref data) => Some(data.to_string()),
                ValueType::SignedNumber16(ref data) => Some(data.to_string()),
                ValueType::SignedNumber64(ref data) => Some(data.to_string()),

                _ => {
                    return None;
                }
            };
        }

        fn get_signed_num_64_value(&self) -> Option<i64> {
            return match self {
                ValueType::SignedNumber64(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        fn get_unsigned_num_64_value(&self) -> Option<u64> {
            return match self {
                ValueType::UnSignedNumber64(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        fn get_unsigned_num_32_value(&self) -> Option<u32> {
            return match self {
                ValueType::UnSignedNumber32(data) => Some(*data),
                ValueType::String(data) => Some(data.parse::<u32>().unwrap()),

                _ => {
                    return None;
                }
            };
        }

        fn get_signed_num_16_value(&self) -> Option<i16> {
            return match self {
                ValueType::SignedNumber16(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        fn get_unsigned_num_16_value(&self) -> Option<u16> {
            return match self {
                ValueType::UnSignedNumber16(data) => Some(*data),

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
                ValueType::U8Vec(data) => Some(data),

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
    pub enum ValueType {
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

    impl ValueType {

        
        

        #[allow(unused)]
        async fn write<W: AsyncWrite + Unpin>(& self, mut writer: W) -> Result<(), ParserError> {
            match self {
                ValueType::String(s) => {
                                writer.write(s.as_bytes()).await?;
                            }
                ValueType::SignedNumber64(num) => {
                                writer.write_i64(*num).await?;
                            }
                ValueType::UnSignedNumber64(num) => {
                                writer.write_u64(*num).await?;
                            }
                ValueType::UnSignedNumber32(num) => {
                    writer.write_u32(*num).await?;
                }
                ValueType::U8Vec(data) => {
                                writer.write_all(&data[..]).await?;
                            }
                ValueType::SignedNumber16(num) => {
                    writer.write_i16(*num).await?;
                }
                ValueType::UnSignedNumber16(num) => {
                    writer.write_u16(*num).await?;
                },

                

                ValueType::None => todo!(),
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

    pub trait InfoProvider: Default + Send + Sync {
        #[allow(unused)]
        fn get_info(&self, key: &String) -> Option<&ValueType>;

        #[allow(unused)]
        fn get_info_mut(&mut self, key: &String) -> Option<&mut ValueType>;

        #[allow(unused)]
        fn get_keys_by_group_name(&self, name: String) -> Option<Vec<& String>>;

        fn add_info(&mut self, key: String, value: ValueType);

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
        fn get_request_spec(&self) -> &Placeholder;
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
        fn get_response_spec(&self) -> &Placeholder;
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
            req: REQI,
            writer: W,
            spec: &Placeholder,
        ) -> Result<(), ParserError>
        where W: AsyncWrite + Unpin + Send + Sync;

        async fn deserialize_from<B>(
            &self,
            request_info: REQI,
            reader: &mut  B,
            spec: &Placeholder,
        ) -> Result<REQI, ParserError> where B:AsyncRead + Unpin + Send + Sync ;        
    }


    #[async_trait]
    pub trait ResponseSerializer<RSI>: Send + Sync 
    where RSI: ResponseInfo ,
        
    {
        async fn serialize_to<W>(
            &self,
            req: RSI,
            writer: W,
            spec: &Placeholder,
        ) -> Result<(), ParserError>
        where W: AsyncWrite + Unpin + Send + Sync;

        #[allow(unused)]
        async fn deserialize_from<R>(&self,  
            response_info: &mut RSI,
            reader: &mut BufReader<R>,
            spec: &Placeholder) -> Result<(), ParserError>
        where R:AsyncRead + Unpin + Send + Sync;
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
            request_info: REQI,
            writer: W,
            spec: &Placeholder,
        ) -> Result<(), ParserError> 
        where W: AsyncWrite + Unpin + Send + Sync {
            let mut protocol_writer = ProtocolBuffWriter::new(writer);
            protocol_writer
                .write_composite(spec, &request_info, None)
                .await?;
            Ok(())
        }

        async fn deserialize_from< B>(
            &self,
            mut request_info:  REQI,
            reader: &mut B,
            spec: &Placeholder,
        )  -> Result<REQI, ParserError> 
        where B:AsyncRead + Unpin + Send + Sync {
            let mut protocol_reader = ProtocolBuffReader::new( BufReader::new(reader), 1024);
            let result = protocol_reader
            .parse_composite(&mut request_info, spec).await;
            
            if let Err(parser_error) = result{
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
            spec: &Placeholder,
        ) -> Result<(), ParserError> where W: AsyncWrite + Unpin + Send + Sync {
            let mut protocol_writer = ProtocolBuffWriter::new(writer);
            protocol_writer
                .write_composite(spec, &response_info, None)
                .await?;
            Ok(())
        }

        //(&self, mut response_info: RSI,reader: R, spec: &Placeholder)
        //async fn deserialize_from(&self,  response_info: &mut RSI,reader: &mut BufReader<&mut R>, spec: &Placeholder) -> Result<RSI, ParserError>;

        async fn deserialize_from<R>(
            &self,
            response_info:&mut RESI,
            reader: &mut BufReader< R>,
            spec: &Placeholder,
        ) -> Result<(), ParserError> 
        where R:AsyncRead + Unpin + Send + Sync {
            let mut protocol_reader = ProtocolBuffReader::new(reader, 1024);
            protocol_reader
                .parse_composite(response_info, spec).await?;
                
            Ok(())
        }        
    }

    #[allow(unused)]
    pub struct Protocol {
        name: ProtocolVersion,
        transport: Transport,
        format: ProtocolFormat,
        request_place_holder: Placeholder,
        response_place_holder: Placeholder,
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
            let req_info = self.request_factory.create_request_info();
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
                    req_info,
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

    #[derive(Default)]
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
    }

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

    #[derive(Default)]
    pub struct Placeholder {
        #[allow(dead_code)]
        pub name: PlaceHolderIdentifier,
        pub place_holder_type: PlaceHolderType,
        pub constituents: Option<Vec<Placeholder>>,
        #[allow(dead_code)]
        pub optional: bool,
    }

    impl Placeholder {
        pub fn new(
            place_holder_identifier: PlaceHolderIdentifier,
            constituents: Option<Vec<Placeholder>>,
            place_holder_type: PlaceHolderType,
            optional: bool,
        ) -> Self {
            Placeholder {
                name: place_holder_identifier,
                place_holder_type,
                constituents,
                optional,
            }
        }

        #[allow(unused)]
        pub fn new_key_placeholder(
            name: String,
            constituents: Option<Vec<Placeholder>>,
            place_holder_type: PlaceHolderType,
            optional: bool,
        ) -> Self {
            Placeholder {
                name: PlaceHolderIdentifier::Key,
                place_holder_type,
                constituents,
                optional,
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
                name: PlaceHolderIdentifier::InlineKeyWithValue(name),
                place_holder_type,
                constituents,
                optional,
            }
        }

        #[allow(unused)]
        pub fn new_value_placeholder(
            constituents: Option<Vec<Placeholder>>,
            place_holder_type: PlaceHolderType,
            optional: bool,
        ) -> Self {
            Placeholder {
                name: PlaceHolderIdentifier::Value,
                place_holder_type,
                constituents,
                optional,
            }
        }
    }

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
        pub fn parse(place_holder_type: &PlaceHolderType, value: &[u8]) -> ValueType {
            match place_holder_type {
                PlaceHolderType::AnyString => {
                                            ValueType::String(String::from_utf8(value.to_vec()).unwrap())
                                        }
                PlaceHolderType::ExactString(input) => {
                                            ValueType::String(String::from_utf8(value.to_vec()).unwrap())
                                            //todo!("Implement ExactString")
                                        }
                PlaceHolderType::OneOf(_) => {
                                            ValueType::String(String::from_utf8(value.to_vec()).unwrap())
                                        }
                PlaceHolderType::Delimiter(_) => {
                                            ValueType::String(String::from_utf8(value.to_vec()).unwrap())
                                        }
                PlaceHolderType::Space => ValueType::String(" ".to_string()),
                PlaceHolderType::NewLine => ValueType::String("\r\n".to_string()),
                PlaceHolderType::Composite => todo!(),
                PlaceHolderType::RepeatMany => todo!(),
                PlaceHolderType::RepeatN(_) => todo!(),
                OneOf(items) => todo!(),
                PlaceHolderType::StreamValue(data) => todo!(),
                PlaceHolderType::BytesOfSizeFromHeader(_) => ValueType::U8Vec(value.to_vec()),
                PlaceHolderType::BytesOfSizeN(_) => ValueType::U8Vec(value.to_vec()),
                PlaceHolderType::Bytes => {
                    ValueType::U8Vec(value.to_vec())                
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

    #[allow(unused)]
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
    }

    /* impl RequestParse for HttpParser {
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

    #[allow(dead_code)]
    pub trait ProtocolSpecBuilder {
        fn expect_newline(&mut self) -> &mut Self;
        fn expect_delimiter(&mut self, delimiter: String) -> &mut Self;
        fn expect_space(&mut self) -> &mut Self;

        fn expect_composite(&mut self, place_holder: Placeholder, name: String) -> &mut Self;
        fn expect_string(&mut self, identifier: PlaceHolderIdentifier, optional: bool)
            -> &mut Self;
        fn expect_exact_string(
            &mut self,
            identifier: PlaceHolderIdentifier,
            input: String,
            optional: bool,
        ) -> &mut Self;

        fn expect_bytes_of_size(&mut self, id: PlaceHolderIdentifier, size:u32, optional: bool) -> &mut Self;
        fn expect_bytes_of_size_from_header(&mut self, id: PlaceHolderIdentifier, header:String, optional: bool) -> &mut Self;

        fn expect_one_of_string(
            &mut self,
            one_of: Vec<String>,
            identifier: PlaceHolderIdentifier,
            optional: bool,
        ) -> &mut Self;

        fn expect_repeat_many(&mut self, placeholder: Placeholder, name: String) -> &mut Self;
        fn expect_repeat_n(
            &mut self,
            repeat_count: u8,
            placeholder: Placeholder,
            name: String,
        ) -> &mut Self;

        fn expect_stream(
            &mut self,
            identifier: PlaceHolderIdentifier,
            name: String,
            optional: bool,
        ) -> &mut Self;

        //fn expect_key_string(&mut self, identifier: PlaceHolderIdentifier) -> &mut Self;

        fn expect_value_string(&mut self, optional: bool) -> &mut Self;

        fn build(&mut self) -> Placeholder;
    }

    #[allow(dead_code, unused)]
    fn parse<R>(placeholder: Placeholder, _buf_reader: BufReader<R>)
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

    impl Placeholder {
        pub fn add_place_holder(&mut self, place_holder: Placeholder) {
            match self.constituents {
                None => {
                    let mut vec = vec![];
                    vec.push(place_holder);
                    self.constituents = Some(vec);
                }
                Some(ref mut place_holders) => {
                    place_holders.push(place_holder);
                }
            }
        }
    }

    #[derive(Default)]
    pub struct SpecBuilder(pub Placeholder);

    #[allow(dead_code)]
    impl SpecBuilder {
        pub fn new(name: String) -> Self {
            SpecBuilder(Placeholder {
                name: PlaceHolderIdentifier::Name(name),
                place_holder_type: PlaceHolderType::Composite,
                constituents: Some(vec![]),
                optional: false,
            })
        }

        pub fn new_composite(name: String, optional: bool) -> Self {
            SpecBuilder(Placeholder {
                name: PlaceHolderIdentifier::Name(name),
                place_holder_type: PlaceHolderType::Composite,
                constituents: Some(vec![]),
                optional,
            })
        }
    }

    impl ProtocolSpecBuilder for SpecBuilder {
        fn expect_newline(&mut self) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                PlaceHolderIdentifier::Name(String::new()),
                None,
                PlaceHolderType::NewLine,
                false,
            ));
            return self;
        }

        fn expect_delimiter(&mut self, delimiter: String) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                Name(String::new()),
                None,
                PlaceHolderType::Delimiter(delimiter),
                false,
            ));
            return self;
        }

        fn expect_space(&mut self) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                Name("space".to_string()),
                None,
                PlaceHolderType::Space,
                false,
            ));
            return self;
        }

        fn expect_composite(&mut self, place_holder: Placeholder, name: String) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                Name(name),
                Option::from(vec![place_holder]),
                PlaceHolderType::Composite,
                false,
            ));
            return self;
        }

        fn expect_string(&mut self, id: PlaceHolderIdentifier, optional: bool) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                id,
                None,
                PlaceHolderType::AnyString,
                optional,
            ));
            return self;
        }

        fn expect_exact_string(
            &mut self,
            identifier: PlaceHolderIdentifier,
            input: String,
            optional: bool,
        ) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                identifier,
                None,
                PlaceHolderType::ExactString(input),
                optional,
            ));
            return self;
        }

        /* fn expect_key_string(&mut self, id: PlaceHolderIdentifier) -> &mut Self {
            self.0.add_place_holder(Placeholder::new_key_placeholder(
                id,
                None,
                PlaceHolderType::AnyString,
            ));
            return self;
        } */

        fn expect_value_string(&mut self, optional: bool) -> &mut Self {
            self.0.add_place_holder(Placeholder::new_value_placeholder(
                None,
                PlaceHolderType::AnyString,
                optional,
            ));
            return self;
        }

        fn expect_one_of_string(
            &mut self,
            one_of: Vec<String>,
            id: PlaceHolderIdentifier,
            optional: bool,
        ) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                id,
                None,
                PlaceHolderType::OneOf(one_of),
                optional,
            ));
            return self;
        }

        fn expect_stream(
            &mut self,
            id: PlaceHolderIdentifier,
            name: String,
            optional: bool,
        ) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                id,
                None,
                PlaceHolderType::StreamValue(name),
                optional,
            ));
            return self;
        }

        fn expect_repeat_many(&mut self, placeholder: Placeholder, name: String) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                Name(name),
                Some(vec![placeholder]),
                PlaceHolderType::RepeatMany,
                false,
            ));
            return self;
        }

        fn expect_repeat_n(
            &mut self,
            repeat_count: u8,
            placeholder: Placeholder,
            name: String,
        ) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                Name(name),
                Some(vec![placeholder]),
                PlaceHolderType::RepeatN(repeat_count),
                false,
            ));
            return self;
        }

        fn build(&mut self) -> Placeholder {
            return mem::take(&mut self.0);
        }

        fn expect_bytes_of_size(&mut self, id: PlaceHolderIdentifier, size:u32, optional: bool) -> &mut Self {
            self.0
                .add_place_holder(Placeholder::new(id, None, PlaceHolderType::BytesOfSizeN(size), optional));
            return self;
        }
        
        fn expect_bytes_of_size_from_header(&mut self, id: PlaceHolderIdentifier, header:String, optional:bool) -> &mut Self {
            self.0
                .add_place_holder(Placeholder::new(id, None, PlaceHolderType::BytesOfSizeFromHeader(header), optional));
            return self;
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

    #[test]
    fn test_protocol_spec_builder() {
        let mut spec_builder = SpecBuilder(Placeholder::new(
            Name("Request".to_string()),
            Some(vec![]),
            PlaceHolderType::Composite,
            false,
        ));

        spec_builder.expect_string(InlineKeyWithValue("request_method".to_string()), false);
    }
}

#[cfg(test)]
mod test_utils {
    use std::collections::HashMap;

    use crate::core::{InfoProvider, ValueType};

    pub fn assert_result_has_string(
        result: Result<Option<crate::core::ValueType>, crate::core::ParserError>,
        data: String,
    ) {
        match result {
            Ok(Some(crate::core::ValueType::String(value))) => {
                assert!(value == data);
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[derive(Default)]
    pub struct TestRequestInfo(HashMap<String, ValueType>);

    impl TestRequestInfo {
        pub fn new() -> Self {
            TestRequestInfo(HashMap::new())
        }
    }

    impl InfoProvider for TestRequestInfo {
        fn add_info(&mut self, key: String, value: ValueType) {
            self.0.insert(key, value);
        }

        fn get_info(&self, key: &String) -> Option<&crate::core::ValueType> {
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

        fn get_info_mut(&mut self, key: &String) -> Option<&mut ValueType> {
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
