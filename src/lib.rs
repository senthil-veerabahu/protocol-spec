mod core {
    use crate::core::PlaceHolderIdentifier::Name;
    use crate::core::PlaceHolderType::OneOf;
    use protocol_reader::ProtocolBuffReader;
    
    use std::{
        fmt::{Display, Formatter},
        mem::{self},
    };
    use tokio::{
        io::{AsyncRead, BufReader},
        net::TcpListener,
    };

    #[allow(dead_code)]
    pub trait ProtocolInfo {
        fn get_name() -> String;
        fn get_version() -> String;
        fn get_transport_type() -> Transport;
        fn get_format() -> ProtocolFormat;
    }

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
        fn get_string_value(&self) -> Option<String>;
        fn get_signed_num_64_value(&self) -> Option<i64>;
        fn get_unsigned_num_64_value(&self) -> Option<u64>;
        //fn get_unsigned_byte_slice(&self) -> Option<&'a [u8]>;

        fn get_u8_vec(&self) -> Option<&Vec<u8>>;
    }

    impl<'a> ValueExtractor<'a> for ValueType {
        fn get_string_value(&self) -> Option<String> {
            return match &self {
                ValueType::String(ref data) => Some(data.clone()),

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
    }

    //#[derive(Clone)]
    #[allow(unused)]
    pub enum ValueType {
        String(String),
        SignedNumber64(i64),
        UnSignedNumber64(u64),
        //UnSignedByteSlice([u8]),
        U8Vec(Vec<u8>),
        StreamData(protocol_reader::ProtoStream<BufReader<tokio::net::TcpStream>>),
        None,
    }

    #[allow(unused)]
    impl ValueType {
        fn is_none(&self) -> bool {
            match self {
                ValueType::None => {
                    return false;
                }
                _ => {
                    return true;
                }
            }
        }

        fn has_value(&self) -> bool {
            match self {
                ValueType::None => {
                    return false;
                }
                _ => {
                    return true;
                }
            }
        }

        /*fn get_value<T>(&self) -> impl ValueTypeExtractor<T> {
            match self {
                ValueType::String(s) => { return Value(s); }
                ValueType::SignedNumber64(data) => { return Value(data); }
                ValueType::UnSignedNumber64(data) => { return Value(data); }
                ValueType::UnSignedByteSlice(data) => { return Value(data) }
                ValueType::None => { Value(()) }
            }
        }*/
    }

    /* #[allow(unused)]
    trait ValueTypeExtractor<T> {
        fn get_value(&self) -> Value<T>;
    } */

    /*     #[allow(unused)]
    struct Value<T>(T); */

    /*impl <T> ValueExtractor<T> for Value<T>{
        fn get_value(&self) -> T {
            &self.0
        }
    }*/

    /*impl <'a, > ValueType<'a> where Value<&'a &'a str>: ValueExtractor<T>{
        fn get_value_extractor(&self) -> impl ValueExtractor<T> {
            match self {
                ValueType::String(s) => { return Value(s); }
                ValueType::SignedNumber64(data) => { return Value(data); }
                ValueType::UnSignedNumber64(data) => { return Value(data); }
                ValueType::UnSignedByteSlice(data) => { return Value(data); }
                ValueType::None => { return Value(()); }
            }
        }
    }*/

    /*impl <'a, T> ValueExtractor<T> for ValueType<'a>{
        fn get_value(&self) -> Value<T> {
            match self {
                ValueType::String(s) => { return Value(s); }
                ValueType::SignedNumber64(data) => { return Value(data); }
                ValueType::UnSignedNumber64(data) => { return Value(data); }
                ValueType::UnSignedByteSlice(data) => { return Value(data); }
                ValueType::None => { return Value(()); }
            }
        }
    }*/

    #[allow(unused)]
    pub trait RequestProcessorRegistrar {
        fn register_request_processor<'a, H, RI>(
            request_type: String,
            request_info: RI,
            request_handler: H,
        ) where
            H: RequestHandler,
            RI: RequestInfo;
    }

    pub trait RequestParser {
        #[allow(unused)]
        async fn parse_request<RI, RequestStream>(
            &self,
            reader: RequestStream,
        ) -> Result<RI, ParserError>
        where
            /*H: RequestHandler,*/
            RI: RequestInfo,
            RequestStream: AsyncRead + Unpin;
    }

    /*
    use tokio::stream::StreamExt;



    async fn process_stream(mut stream: impl Stream<Item = String>) {

    while let Some(value) = stream.next().await {

        println!("Value: {}", value);

    }

    }*/

    pub trait RequestInfo: Default {
        #[allow(unused)]
        fn get_info(&self, key: String) -> Option<&ValueType>;

        fn add_info(&mut self, key: String, value: ValueType);
    }

    pub trait ResponseInfo: Default {
        #[allow(unused)]
        fn get_info(&self, key: String) -> Option<&ValueType>;

        #[allow(unused)]
        fn add_info(&mut self, key: String, value: ValueType);
    }

    #[allow(unused)]
    pub trait RequestInfoBuilder<RQI>
    where
        RQI: RequestInfo,
    {
        fn build(&self) -> RQI;
    }

    #[allow(unused)]
    pub trait ResponseInfoBuilder<RSI>
    where
        RSI: ResponseInfo,
    {
        fn build(&self) -> RSI;
    }

    pub trait RequestHandler {}

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

    #[allow(unused)]
    pub enum ServerError {
        StartError(String),
        StopError,
        RequestError(ParserError),
        ResponseError(ParserError),
        IOError(std::io::Error),
    }

    pub trait Server<RQI, RSI>
    where
        RQI: RequestInfo,
        RSI: ResponseInfo,
    {
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

    
    struct ServerInstance<RQIB, RSIB, RQI, RSI>
    where
        RQIB: RequestInfoBuilder<RQI>,
        RSIB: ResponseInfoBuilder<RSI>,
        RQI: RequestInfo,
        RSI: ResponseInfo,
    {
        hosts: Vec<String>,
        
        #[allow(unused)]
        protocol: Protocol,
        request_spec: Placeholder,
        #[allow(unused)]
        response_spec: Placeholder,
        req_info_builder: RQIB,
        #[allow(unused)]
        res_info_builder: RSIB,
        listeners: Vec<TcpListener>,

        phantom_req_info: std::marker::PhantomData<RQI>,
        phantom_res_info: std::marker::PhantomData<RSI>,
    }

    impl<RQIB, RSIB, RQI, RSI> ServerInstance<RQIB, RSIB, RQI, RSI>
    where
        RQI: RequestInfo + Send + Sync,
        RSI: ResponseInfo + Send + Sync,
        RQIB: RequestInfoBuilder<RQI> + Send + Sync,
        RSIB: ResponseInfoBuilder<RSI> + Send + Sync,
    {

        #[allow(unused)]
        async fn create_listeners(&mut self) -> Result<(), ServerError> {
            for host in &self.hosts {
                let option = host.split_once(":");
                match option {
                    Some((host, port)) => {
                        let listener = TcpListener::bind(format!("{}:{}", host, port)).await.unwrap();
                        self.listeners.push(listener);                        
                    }
                    None => {
                        return Err(ServerError::StartError("Invalid host:port format".to_owned()));
                    }
                }
            }   
            Ok(())         
        }
        
        async fn accept_connections(&'static self, tcp_listener: &'static  TcpListener) {
            let spec = &self.request_spec;
            tokio::spawn(async move {
                loop {
                    let (mut socket, addr) = tcp_listener.accept().await.unwrap();
                    println!("Accepted connection from {}", addr);
                    let mut req_info = self.req_info_builder.build();
                    let _handle = tokio::spawn(async move {
                        let mut protocol_buffer =
                            ProtocolBuffReader::new(BufReader::new(&mut socket), 1024);

                        let _result: Result<(), ParserError> = protocol_buffer.parse_composite(spec, &mut req_info).await;
                    });

                    // Handle the connection
                    // ...
                }
            });
        }
    }

    impl<RQIB, RSIB, RQI, RSI> Server<RQI, RSI> for ServerInstance<RQIB, RSIB, RQI, RSI>
    where
        RQI: RequestInfo + Send + Sync,
        RSI: ResponseInfo + Send + Sync,
        RQIB: RequestInfoBuilder<RQI> + Send + Sync,
        RSIB: ResponseInfoBuilder<RSI> + Send + Sync,
    {
        async fn start(&'static mut self) -> Result<(), ServerError>{
            
            //let host = String::new();
            self.create_listeners().await?;
            for listener in &self.listeners {
                let _result = self.accept_connections(listener).await;
                //println!("{:?}", listener);
            }
            
            Ok(())
        }

        async fn stop(&mut self) -> Result<(), ServerError>{
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

    #[derive(Default)]
    #[allow(unused)]
    pub enum PlaceHolderType {
        #[default]
        AnyString,
        ExactString(String),
        OneOf(Vec<String>),
        Bytes,
        Space,
        NewLine,
        Delimiter(String),
        Composite,
        RepeatMany,
        RepeatN(u8),
        Stream,
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
                optional
            }
        }

        #[allow(unused)]
        pub fn new_key_placeholder(
            name: String,
            constituents: Option<Vec<Placeholder>>,
            place_holder_type: PlaceHolderType,
            optional: bool
        ) -> Self {
            Placeholder {
                name: PlaceHolderIdentifier::Key,
                place_holder_type,
                constituents,
                optional
            }
        }

        #[allow(unused)]
        pub fn new_placeholder_with_key(
            name: String,
            constituents: Option<Vec<Placeholder>>,
            place_holder_type: PlaceHolderType,
            optional: bool
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
            optional: bool
        ) -> Self {
            Placeholder {
                name: PlaceHolderIdentifier::Value,
                place_holder_type,
                constituents,
                optional
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
                PlaceHolderType::Bytes => ValueType::U8Vec(value.to_vec()),
                PlaceHolderType::Space => ValueType::String(" ".to_string()),
                PlaceHolderType::NewLine => ValueType::String("\n".to_string()),
                PlaceHolderType::Composite => todo!(),
                PlaceHolderType::RepeatMany => todo!(),
                PlaceHolderType::RepeatN(_) => todo!(),
                OneOf(items) => todo!(),
                PlaceHolderType::Stream => todo!(),
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
        fn expect_string(&mut self, identifier: PlaceHolderIdentifier, optional: bool) -> &mut Self;
        fn expect_exact_string(
            &mut self,
            identifier: PlaceHolderIdentifier,
            input: String,
            optional:bool,
        ) -> &mut Self;

        fn expect_bytes(&mut self, identifier: PlaceHolderIdentifier) -> &mut Self;

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

        fn expect_stream(&mut self, identifier: PlaceHolderIdentifier, optional:bool) -> &mut Self;

        //fn expect_key_string(&mut self, identifier: PlaceHolderIdentifier) -> &mut Self;

        fn expect_value_string(&mut self, optional:bool) -> &mut Self;

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
            PlaceHolderType::Bytes => {}
            PlaceHolderType::Space => {}
            PlaceHolderType::NewLine => {}
            PlaceHolderType::Delimiter(_) => {}
            PlaceHolderType::Composite => {}
            PlaceHolderType::RepeatMany => {}
            PlaceHolderType::RepeatN(_) => {}
            PlaceHolderType::Stream => todo!(),
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
            self.0
                .add_place_holder(Placeholder::new(id, None, PlaceHolderType::AnyString, optional));
            return self;
        }

        fn expect_exact_string(
            &mut self,
            identifier: PlaceHolderIdentifier,
            input: String,
            optional: bool
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
            self.0
                .add_place_holder(Placeholder::new(id, None, PlaceHolderType::OneOf(one_of), optional));
            return self;
        }

        fn expect_bytes(&mut self, id: PlaceHolderIdentifier) -> &mut Self {
            self.0
                .add_place_holder(Placeholder::new(id, None, PlaceHolderType::Bytes, false));
            return self;
        }

        fn expect_stream(&mut self, id: PlaceHolderIdentifier, optional: bool,) -> &mut Self {
            self.0
                .add_place_holder(Placeholder::new(id, None, PlaceHolderType::Stream, optional));
            return self;
        }

        fn expect_repeat_many(&mut self, placeholder: Placeholder, name: String) -> &mut Self {
            self.0.add_place_holder(Placeholder::new(
                Name(name),
                Some(vec![placeholder]),
                PlaceHolderType::RepeatMany, false,
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
    }

    pub mod parser {
        use super::*;
        use std::ops::Deref;

        /*pub trait Parser {
            fn parse<'a, T: RequestInfo<'a>>(&self, t: T) -> Result<(), ParserError>;
        }*/
        #[allow(unused)]
        async fn parse_request<P: RequestParser, T: RequestInfo, RequestStream>(
            request_stream: RequestStream,
            parser: P,
        ) -> Result<T, ParserError>
        where
            RequestStream: Unpin + AsyncRead,
        {
            let result = parser.parse_request(request_stream).await;
            return result;
        }

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
                PlaceHolderType::Bytes => {}
                PlaceHolderType::Space => {}
                PlaceHolderType::NewLine => {}
                PlaceHolderType::Delimiter(_) => {}
                PlaceHolderType::Composite => {}
                PlaceHolderType::RepeatMany => {}
                PlaceHolderType::RepeatN(_) => {}
                PlaceHolderType::Stream => todo!(),
            }
        }
    }

    mod protocol_reader;
}

mod http;
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
