use async_trait::async_trait;
use memchr::memmem::Finder;
use pin_project::pin_project;
use std::{
     fmt::{Display}, future::Future, io::{self}, pin::Pin, task::{Context, Poll}, time::Duration
};
use tokio::{io::{AsyncBufRead, AsyncBufReadExt, AsyncRead, ReadBuf}, time::timeout};
use tokio_stream::Stream;

use crate::core::{
    ParserError, SpecRead
};



impl<R> Stream for ProtoStream<R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    type Item = io::Result<u8>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        
        if self.buff_reader.pos < self.buff_reader.buf.len() {
            let value = self.buff_reader.buf[self.buff_reader.pos];
            self.buff_reader.consume(1);
            return Poll::Ready(Some(Ok(value)));
        } else {
            match self.buff_reader.fill_buffer(cx) {
                Poll::Ready(Ok(len)) => {
                    if len == 0 {
                        return Poll::Ready(None);
                    }
                    let value = self.buff_reader.buf[self.buff_reader.pos];
                    self.buff_reader.consume(1);
                    return Poll::Ready(Some(Ok(value)));
                }
                Poll::Ready(Err(err)) => {
                    return Poll::Ready(Some(Err(err)));
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}

#[async_trait]
pub trait PlaceHolderRead {
    #[allow(unused)]
    async fn read_placeholder_as_string(
        self: & mut Self,
        input: String,
        
    ) -> Result<Option<Vec<u8>>, ParserError>;

    #[allow(unused)]
    async fn read_placeholder_until(
        self: &mut Self,        
        delimiter: String,        
        
    ) -> Result<Option<Vec<u8>>, ParserError>;


    #[allow(unused)]
    async fn read_bytes(
        self: & mut Self,        
        size: ReadBytesSize,
        
    ) -> Result<Option<Vec<u8>>, ParserError>;
}


pub trait MarkAndRead:AsyncBufRead + Unpin + Send + Sync {
    fn mark(&mut self) -> Marker;
    fn reset(&mut self, mark: &Marker) -> Result<(), ParserError>;
    fn unmark(&mut self, mark: &Marker) -> Result<(), ParserError>;
    fn is_valid_marker(&self, marker: &Marker) -> Result<(), ParserError>;
    fn has_markers(&self) -> bool;
}



#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Marker{
    pos:usize,
}

impl Marker {
    fn new(pos: usize) -> Self {
        Marker { pos }
    }
}





#[pin_project]
pub(super) struct ProtocolBuffReader<R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    #[pin]
    inner: R,
    cap: usize,
    pos: usize,
    buf: Vec<u8>,
    markers: Vec<Marker>,
    marked_pos: usize,
    marked: bool,
    line_index: usize,
    char_index: usize,
    char_index_in_line: usize,
}

impl <R> SpecRead for ProtocolBuffReader<R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    
}

pub struct ProtoStream<R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    buff_reader: ProtocolBuffReader<R>,
}

impl<R> From<ProtocolBuffReader<R>> for ProtoStream<R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    fn from(buff_reader: ProtocolBuffReader<R>) -> Self {
        ProtoStream { buff_reader }
    }
}

impl<R> ProtoStream<R>
where
    R: AsyncBufRead + Send + Sync +  Unpin,
{
    #[allow(unused)]
    fn new(buff_reader: ProtocolBuffReader<R>) -> Self {
        ProtoStream { buff_reader }
    }

    #[allow(unused)]
    fn as_buff_reader(self) -> ProtocolBuffReader<R> {
        self.into()
    }
}

impl<R> From<ProtoStream<R>> for ProtocolBuffReader<R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    fn from(value: ProtoStream<R>) -> Self {
        value.buff_reader
    }
}

impl<R> AsyncRead for ProtocolBuffReader<R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf,
    ) -> Poll<io::Result<()>> {
        let mut pinned_self = self.project();
        let pinned_reader = Pin::new(&mut pinned_self.inner);
        pinned_reader.poll_read(cx, buf)
    }
}

impl <B> MarkAndRead for ProtocolBuffReader<B> 
    where B: AsyncBufRead +  Send+ Sync + Unpin, {

    fn is_valid_marker(&self, marker: &Marker) -> Result<(), ParserError> {
        if self.markers.is_empty() || marker.pos != self.markers.last().unwrap().pos {
            return Err(ParserError::InvalidMarker {
                line_index: self.line_index,
                char_index: self.char_index_in_line + 1,
                message: "Invalid marker received during reset/unmark operation".to_string(),
            });
        }
        Ok(())
    }
    fn mark(&mut self) -> Marker {
        let marker = Marker::new(self.pos);
        self.markers.push(marker);
        *self.markers.last().unwrap()
    }

    fn reset(&mut self, marker: &Marker) -> Result<(), ParserError> {
        self.is_valid_marker(marker)?;
        let marker = self.markers.pop().unwrap();
        self.pos = marker.pos;        
        Ok(())
    }

    fn unmark(&mut self, marker: &Marker) -> Result<(), ParserError> {
        self.is_valid_marker(marker)?;        
        self.markers.pop().unwrap();            
        Ok(())
    }
    
    fn has_markers(&self) -> bool {
        self.markers.len() > 0
    }
}



impl<R> AsyncBufRead for ProtocolBuffReader<R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        //let mut pinned_self = self.project();
        let me = self.get_mut();
        //let  pinned_reader = Pin::new(&mut pinned_self.inner);
        match me.fill_buffer(cx) {
            Poll::Ready(Ok(_len)) => {
                let data = &((me).buf[me.pos..]);
                Poll::Ready(Ok(data))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let mut pinned_self = self.project();
        let pinned_reader = Pin::new(&mut pinned_self.inner);
        pinned_reader.consume(amt);
    }

}





impl <R>ProtocolBuffReader<R>
    where R: AsyncBufRead + Send + Sync + Unpin,
{
    fn mark(&mut self) {
        self.marked_pos = self.pos;
        self.marked = true;
    }

    fn reset(&mut self) {
        self.pos = self.marked_pos;
        self.marked = false;
    }

    fn unmark(&mut self) {
        self.marked = false;
    }

    fn fill_buffer(self: &mut Self, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        //let mut pinned_self = self.project();
        let pinned_reader = Pin::new(&mut self.inner);
        let result = pinned_reader.poll_fill_buf(cx);
        let mut len = 0;
        let result = match result {
            Poll::Ready(Ok(buf)) => {
                println!("len {}, cap {}", self.buf.len(), self.cap);
                self.buf.extend_from_slice(buf);
                len = buf.len();
                Poll::Ready(Ok(buf.len()))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        };
        if len > 0 {
            Pin::new(&mut self.inner).consume(len);
        }
        return result;
    }

    #[allow(unused)]
    fn get_buf_unread_size(&self) -> usize {
        self.buf.len() - self.pos + 1
    }

    fn buf_has_enough_data(&self, bytes_size: &ReadBytesSize) -> bool {
            (self.buf.len() - self.pos + 1) as u32 >= bytes_size.get_value()
    }

    #[allow(unused)]
    pub(super) fn new(reader: R, cap: usize) -> Self {
        ProtocolBuffReader {
            inner: reader,
            cap,
            pos: 0,
            buf: Vec::with_capacity(cap),
            marked_pos: 0,
            markers: Vec::new(),
            marked: false,
            line_index: 0,
            char_index: 0,
            char_index_in_line: 0,
        }
    }

    fn increment_line_index(&mut self) {
        self.line_index += 1;
        self.char_index_in_line = 0;
        self.char_index += 1;
    }
    fn increment_char_index(&mut self) {        
        self.char_index_in_line += 1;
        self.char_index += 1;
    }

    fn get_error_char_index(&self) -> usize {
        self.char_index_in_line + 1
    }

    fn increment_char_index_by(&mut self, count:usize) {        
        self.char_index_in_line += count;
        self.char_index += count;
    }

    #[allow(unused)]
    fn get_buffer_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buf
    }

    fn get_buffer(&self) -> &Vec<u8> {
        &self.buf
    }

    #[allow(unused)]
    fn get_current_buffer(&self) -> &[u8] {
        if self.buf.is_empty() {
            return &[];
        }
        let data = &self.buf[self.pos..];
        println!("-{}-", String::from_utf8_lossy(data));
        data
        
    }

    #[allow(unused)]
    fn as_stream<'a>(self) -> ProtoStream<R>
    where
        R: AsyncBufRead + Unpin,
    {
        self.into()
    }
}


pub enum ReadBytesSize{
    Fixed(u32),
    Full,
}

impl ReadBytesSize {
    fn get_value(&self) -> u32 {
        match self {
            ReadBytesSize::Fixed(size) => *size,
            ReadBytesSize::Full => u32::MAX, // or usize::MAX, depending on your use case
        }
    }

    fn is_full(&self) -> bool {
        matches!(self, ReadBytesSize::Full)
    }
}

impl Display for ReadBytesSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadBytesSize::Fixed(size) => write!(f, "Fixed({})", size),
            ReadBytesSize::Full => write!(f, "Full"),
        }
    }
}


#[pin_project]
struct ReadBytes<'a, R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    protocol_reader: &'a mut ProtocolBuffReader<R>,
    size: ReadBytesSize,
    
    
}

impl <'a, R> ReadBytes<'a, R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    fn new(
        protocol_reader: &'a mut ProtocolBuffReader<R>,
        
        size: ReadBytesSize,
    ) -> Self {
        ReadBytes {
            protocol_reader,
        
            size,
        }
    }
}

impl<'a, R> Future for ReadBytes<'a,  R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    type Output = Result<Option<Vec<u8>>, ParserError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut this = self.project();
        let read_bytes_expected_size = this.size;
        let protocol_reader = &mut this.protocol_reader;
                
        loop {
            let buf = protocol_reader.get_buffer();
            if buf.len() == 0 || !protocol_reader.buf_has_enough_data(read_bytes_expected_size) {
                let result = protocol_reader.fill_buffer(cx);
                match result {
                    Poll::Ready(Ok(read_length)) => {
                        if read_length > 0 {
                            continue;
                        } else {
                            if let ReadBytesSize::Fixed(_) = read_bytes_expected_size {                                
                                    return Poll::Ready(Ok(None));
                            }
                            
                        }
                    }
                    Poll::Pending => {
                        //todo: check if pending needs to be handled differently
                        if let ReadBytesSize::Fixed(_) = read_bytes_expected_size {
                            return Poll::Pending;
                        }
                    }
                    Poll::Ready(Err(e)) => {
                        return Poll::Ready(Err(ParserError::IOError { error: e }));
                    }
                };
            }
            let buf = protocol_reader.get_buffer();
            let pos = protocol_reader.pos;
            if let ReadBytesSize::Fixed(size) = read_bytes_expected_size {
                let size = *size as usize;
                if pos + size -1 < buf.len() {
                    return Poll::Ready(Ok(Some(buf[pos..pos + size].to_vec())));
                } else {
                    return Poll::Ready(Err(ParserError::TokenExpected {
                        line_index: protocol_reader.line_index,
                        char_index: protocol_reader.get_error_char_index(),
                        message: "Expected token not found, EOF reached".to_string(),
                    }));
                }

            }else {
                return Poll::Ready(Ok(Some(
                        buf[pos..buf.len()].to_vec())
                ));
            }            
        }
    }
}

#[pin_project]
struct ReadPlaceHolderUntil<'a, R>
where
    R: AsyncBufRead + Send + Sync + Unpin,    
{
    protocol_reader: &'a mut ProtocolBuffReader<R>,
    //placeholder: &'a Placeholder,
    delimiter: String,    
}

#[pin_project]
struct ReadString<'a, R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    protocol_reader: &'a mut ProtocolBuffReader<R>,
    input: String,
}

impl <'a, R> ReadPlaceHolderUntil<'a, R> where
R: AsyncBufRead + Send + Sync + Unpin,
{
    fn new(
        protocol_reader: &'a mut ProtocolBuffReader<R>,
        delimiter: String,        
    ) -> Self {
        ReadPlaceHolderUntil {
            protocol_reader,
            delimiter,            
        }
    }
}

impl<'a, R> Future for ReadPlaceHolderUntil<'a, R >
where
    R: AsyncBufRead + Send + Sync + Unpin,{
    type Output = Result<Option<Vec<u8>>, ParserError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut this = self.project();
        //let placeholder = this.placeholder;
        let delimiter = this.delimiter;
        let protocol_reader = &mut this.protocol_reader;
        //if protocol_reader.pos < (protocol_reader.cap - 1) {
            //protocol_reader.mark_if_optional(placeholder);
            //let pinned_reader = Pin::new(&mut protocol_reader.inner);
            if let Some(value) = perform_search(cx, delimiter, protocol_reader) {
                match value {
                    Poll::Ready(result) => match result {
                        Ok(index) => {
                            let matched_portion =
                                &protocol_reader.get_buffer()[protocol_reader.pos..index];
                            /* let place_holder_value = PlaceHolderValue::parse(
                                &placeholder.place_holder_type,
                                matched_portion,
                            ); */

                            //protocol_reader.consume(matched_portion.len() + delimiter.len());
                            //protocol_reader.unmark_if_optional(placeholder);
                            return Poll::Ready(Ok(Some(matched_portion.to_vec())));
                        }
                        Err(e) => {
                            //protocol_reader.reset_if_optional(placeholder);
                            /* if placeholder.optional {
                                return Poll::Ready(Ok(None));
                            } else { */
                                return Poll::Ready(Err(e));
                            /* } */
                        }
                    },
                    Poll::Pending => { 
                        /* if this.info_provider.has_all_data() {
                            protocol_reader.unmark_if_optional(placeholder);
                            return Poll::Ready(Err(ParserError::EndOfStream));
                        } */
                        return Poll::Pending;
                    }
                }
            }
            Poll::Ready(Err(protocol_reader.error_token_expected_eof_reached()))
        /* } else {
            Poll::Ready(Err(protocol_reader.error_token_expected_eof_reached()))
        } */
    }
}

impl<'a, R> ReadString<'a, R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    fn new(
        protocol_reader: &'a mut ProtocolBuffReader<R>,
        
        input: String,
    ) -> Self {
        ReadString {
            protocol_reader,
        
            input,
        }
    }
}

impl<'a, R> Future for ReadString<'a, R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    type Output = Result<Option<Vec<u8>>, ParserError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut this = self.project();

        //let placeholder = this.placeholder;
        let input = this.input;
        let protocol_reader = &mut this.protocol_reader;
        //if !protocol_reader.buf_has_enough_data(input.len()) {
        //let pinned_reader = Pin::new(&mut protocol_reader.inner);
        //protocol_reader.mark_if_optional(placeholder);
        if let Some(value) = read_string(cx, input, protocol_reader) {
            match value {
                Poll::Ready(result) => match result {
                    Ok(index) => {
                        let matched_portion =
                            &protocol_reader.get_buffer()[index..index + input.len()];
                        /* let place_holder_value = PlaceHolderValue::parse(
                            &placeholder.place_holder_type,
                            matched_portion,
                        );

                        protocol_reader.consume(input.len());
                        protocol_reader.unmark_if_optional(placeholder); */
                        return Poll::Ready(Ok(Some(matched_portion.to_vec())));
                    }
                    Err(e) => {
                        /* protocol_reader.reset_if_optional(placeholder);
                            if placeholder.optional {
                                return Poll::Ready(Ok(None));
                            } else { */
                                return Poll::Ready(Err(e));
                            /* } */
                    }
                },
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
        Poll::Ready(Err(ParserError::TokenExpected {
            line_index: protocol_reader.line_index,
            char_index: protocol_reader.char_index_in_line + 1,
            message: "Expected token not found, EOF reached".to_string(),
        }))
    }
}

#[allow(unused)]
fn token_expected_error(line_index:usize, line_char_pos:usize) -> ParserError {
    ParserError::TokenExpected {
        line_index,
            char_index: line_char_pos,
        message: "Expected token not found, EOF reached".to_string(),
    }
}

fn perform_search<R>(
    cx: &mut Context<'_>,
    delimiter: &mut String,
    protocol_reader: &mut ProtocolBuffReader<R>,
) -> Option<Poll<Result<usize, ParserError>>>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    let finder = Finder::new(delimiter.as_bytes());
    loop {
        let result = finder.find(protocol_reader.get_current_buffer());

        match result {
            Some(match_index) => {
                //protocol_reader.pos = index + 1;
                return Some(Poll::Ready(Ok(match_index + protocol_reader.pos)));
            }

            None => {
                let result = protocol_reader.fill_buffer(cx);
                match result {
                    Poll::Ready(Ok(read_length)) => {
                        if read_length > 0 {
                            continue;
                        } else {
                            return Some(Poll::Ready(Err(ParserError::TokenExpected {
                                line_index: protocol_reader.line_index,
                                char_index: protocol_reader.char_index_in_line + 1,
                                message: "Expected token not found, EOF reached".to_string(),
                            })));
                        }
                    }
                    Poll::Pending => return Some(Poll::Pending),
                    Poll::Ready(Err(e)) => {
                        return Some(Poll::Ready(Err(ParserError::IOError { error: e })));
                    }
                };
            }
        }
    }
}

fn read_string<R>(
    cx: &mut Context<'_>,
    input: &mut String,
    protocol_reader: &mut ProtocolBuffReader<R>,
) -> Option<Poll<Result<usize, ParserError>>>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    loop {
        let buf = protocol_reader.get_buffer();
        if buf.len() == 0 || !protocol_reader.buf_has_enough_data(&ReadBytesSize::Fixed(input.len() as u32)) {
            let result = protocol_reader.fill_buffer(cx);
            match result {
                Poll::Ready(Ok(read_length)) => {
                    if read_length > 0 {
                        continue;
                    } else {
                        return Some(Poll::Ready(Err(ParserError::TokenExpected {
                            line_index: protocol_reader.line_index,
                            char_index: protocol_reader.char_index_in_line,
                            message: "Expected token not found, EOF reached".to_string(),
                        })));
                    }
                }
                Poll::Pending => return Some(Poll::Pending),
                Poll::Ready(Err(e)) => {
                    return Some(Poll::Ready(Err(ParserError::IOError { error: e })));
                }
            };
        }
        let buf = protocol_reader.get_buffer();
        let pos = protocol_reader.pos;
        println!("input get bytes {:?}, len {}", input.as_bytes(), input.as_bytes().len());
        println!("buf  get byets {:?}, len {}", &buf[pos..pos + input.len()], &buf[pos..pos + input.len()].len());
        if &buf[pos..pos + input.len()] == input.as_bytes() {
            return Some(Poll::Ready(Ok(pos)));
        } else {
            return Some(Poll::Ready(Err(ParserError::TokenExpected {
                line_index: protocol_reader.line_index,
                char_index: protocol_reader.get_error_char_index(),
                message: "Expected token not found, EOF reached".to_string(),
            })));
        }
    }
}

#[async_trait]
impl<T> PlaceHolderRead for ProtocolBuffReader<T>
where
    T: AsyncBufRead + Send + Sync + Unpin,
{
    async fn read_placeholder_until(
        self: &mut Self,        
        delimiter: String,        
        
    ) -> Result<Option<Vec<u8>>, ParserError>{
        let data = timeout(Duration::from_secs(1), ReadPlaceHolderUntil::new(self, delimiter)).await;
        match data {
            Ok(Ok(data)) => Ok(data),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(ParserError::EndOfStream),
        }
    }

    async fn read_placeholder_as_string(
        self: &mut Self,
        input: String,
    ) -> Result<Option<Vec<u8>>, ParserError>
    {
        let data = timeout(Duration::from_secs(1),ReadString::new(self, input)).await;
        match data {
            Ok(Ok(data)) => Ok(data),
            Ok(Err(e)) => Err(e),
            Err(_e) => Err(ParserError::EndOfStream),
        }
    }
    
    async fn read_bytes(
        self: &mut Self,
        size: ReadBytesSize,
    ) -> Result<Option<Vec<u8>>, ParserError>
    {
            let data = timeout(Duration::from_secs(1), ReadBytes::new(self, size)).await;
            match data {
                Ok(Ok(data)) => Ok(data),
                Ok(Err(e)) => Err(e),
                Err(_e) => Err(ParserError::EndOfStream),
            }
    }
}

fn is_eof_error(parse_error: &ParserError) -> bool {
    match parse_error {
        ParserError::IOError { error } => {
            if error.kind() == io::ErrorKind::UnexpectedEof {
                true
            } else {
                false
            }
        },
        ParserError::EndOfStream => true,
        _ => false,
    }
}

/* fn update_key(key: &mut Option<String>, placeholder: &Placeholder, input_key_data: Option<String>) {
    match &placeholder.name {
        crate::core::PlaceHolderIdentifier::Key => {
            *key = Some(input_key_data.unwrap().to_owned());
        }
        crate::core::PlaceHolderIdentifier::InlineKeyWithValue(key_name) => {
            *key = Some(key_name.to_owned());
        }
        _ => {
            println!("unknown placeholder identifier type");
        }
    }
} */

impl<R> ProtocolBuffReader<R>
where
    R: AsyncBufRead + Send + Sync + Unpin,
{
    /* #[allow(unused)]
    async fn parse<'a, I>(&mut self, placeholder: &Placeholder, request_info:&mut I) -> Result<PlaceHolderValue, ParserError> where I:RequestInfo<'a>{
        match placeholder.place_holder_type {
            PlaceHolderType::Composite => {
                self.parse_composite(placeholder,request_info).await?;
                todo!()
            }
            _ => {
                todo!()
            }
        }
    } */

   /* pub fn mark_if_optional(&mut self, placeholder: &Placeholder){
        if placeholder.optional {
            self.mark();
        }
   }

   pub fn reset_if_optional(&mut self, placeholder: &Placeholder){
        if placeholder.optional {
            self.reset();
        }
    }

    pub fn unmark_if_optional(&mut self, placeholder: &Placeholder){
        if placeholder.optional {
            self.unmark();
        }
    } */


    #[allow(unused)]
    /* pub(super) async fn parse_composite<RI>(
        &mut self,
        request_info: &mut RI,
        placeholder: &Placeholder,
        
    ) -> Result<(), ParserError>
    where
        RI: InfoProvider,
    {
        match &placeholder.constituents {
            None => {
                panic!("Constituents not found for composite type");
            }
            Some(constituents) => {
                //for i in 0..constituents.len() {
                let mut i = 0;
                let mut key: Option<String> = None;
                loop {
                    let mut value: Option<String> = None;
                    if i >= constituents.len() {
                        break;
                    }
                    let constituent = &constituents[i];
                    match &constituent.place_holder_type {
                        PlaceHolderType::Composite => {
                                                Box::pin(self.parse_composite( request_info, constituent,)).await?;
                                            }
                        PlaceHolderType::RepeatN(n) => {
                                                Box::pin(self.parse_composite(request_info, constituent, )).await?;
                                            }
                        PlaceHolderType::RepeatMany => {
                                                let mut count = 0;
                                                let header_name = match &constituent.name {
                                                    crate::core::PlaceHolderIdentifier::Name(name) => {
                                                        Some(name.to_owned())    
                                                    }
                                
                                                    _ => None,
                                                };
                                                loop {
                                                    self.mark();
                                                    let result =
                                                        Box::pin(self.parse_composite(request_info, constituent, )).await;
                                                    if result.is_err() && is_eof_error(result.as_ref().err().unwrap()) {
                                                        if count > 0 {
                                                            self.reset();
                                        
                                                            break;
                                                        } else {
                                                            return result;
                                                        }
                                                    } else if result.is_err()
                                                        && !is_eof_error(result.as_ref().err().unwrap())
                                                    {
                                                        if count > 0 {
                                                            self.reset();
                                                            break;
                                                        } else {
                                                            return result;
                                                        }
                                                    } else if result.is_ok() {
                                                        count += 1;
                                                        self.unmark();
                                                    }
                                                }
                                            }
                        PlaceHolderType::ExactString(input) => {
                                                //self.mark_if_optional(constituent);
                                                let value_type = self
                                                    .read_placeholder_as_string(constituent, input.to_string())
                                                    .await;
                                                /* if value_type.is_err() {
                                self.reset_if_optional(placeholder);
                            }else */{
                                                    match value_type {
                                                        Ok(Some(v)) => {
                                                            self.increment_char_index_by(input.len());
                                                            update_key_value(
                                                                request_info,
                                                                &mut key,                               
                                                                constituent,
                                                                v,
                                                                self.line_index,
                                                                self.char_index_in_line,
                                                            )?;
                                                        }

                                                        Ok(None) => {
                                                        }
                                                        Err(e) => {
                                                            if !placeholder.optional{
                                                                return Err(e);
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                        PlaceHolderType::AnyString => {
                                                //Self::update_key(&mut key, constituent, None);
                                                let value_type_option = self
                                                    .read_delimited_string(constituent, constituents, &mut i, request_info)
                                                    .await?;


                                                match &value_type_option {
                                                    Some(Value::String(data)) => {
                                                        self.increment_char_index_by(data.len());
                                                        update_key_value(
                                                            request_info,
                                                            &mut key,                                
                                                            constituent,
                                                            Value::String(data.to_owned()),
                                                            self.line_index,
                                                            self.char_index_in_line,
                                                        )?;
                                                    }
                                                    None => {
                                                    }
                                                    _ => {}
                                                }
                            
                             
                                                i += 1;
                                            }
                        PlaceHolderType::StreamValue(name) => {
                                                // let x = Box::pin(self.as_stream());
                                                todo!()
                                            }
                        PlaceHolderType::OneOf(items) => {
                                                let value_type_option = self
                                                    .read_delimited_string(constituent, constituents, &mut i, request_info)
                                                    .await?;
                                                i += 1;

                                                match &value_type_option {
                                                    Some(Value::String(str)) => {
                                                        update_key(&mut key, constituent, Some(str.to_owned()));
                                                        if items.contains(str) {
                                                            self.increment_char_index_by(str.len());
                                                            request_info.add_info(key.unwrap(), Value::String(str.to_owned()));
                                                            key = None;
                                        
                                                            //return Ok(());
                                                        } else {
                                                            return Err(self.error_token_expected_eof_reached());
                                                        }
                                                    }
                                                    None => {}
                                                    _ => {
                                                        return Err(self.error_token_expected_eof_reached());
                                                    }
                                                }
                                            },
                   
                        PlaceHolderType::BytesOfSizeN(size) => {
                            let size = *size as usize;
                            let value_type_option = Box::pin(
                                self.read_bytes( constituent,
                                     ReadBytesSize::Fixed(size as u32))).await?;
                            match value_type_option {
                                Some(value_type) => {
                                    self.increment_char_index_by(size);
                                    update_key_value(
                                        request_info,
                                        &mut key,                                
                                        constituent,
                                        value_type,
                                        self.line_index,
                                        self.char_index_in_line,
                                    )?;
                                }
                                None => {}
                            }
                        },
                        PlaceHolderType::BytesOfSizeFromHeader(header_name, ) => {
                            if let Some(header_value) =  request_info.get_info(header_name){
                                if let Some(size) = header_value.get_unsigned_num_32_value(){
                                    
                                    let value_type_option = Box::pin(ReadBytes::new(self, constituent, ReadBytesSize::Fixed(size))).await?;
                                    //self.increment_char_index_by(size);
                                    if let Some(value_type) = value_type_option {
                                        update_key_value(
                                            request_info,
                                            &mut key,                                
                                            constituent,
                                            value_type,
                                            self.line_index,
                                            self.char_index_in_line,
                                        )?;
                                    }
                                }
                            }
                        },
                        PlaceHolderType::Space => {
                                                let result = self
                                                    .read_placeholder_as_string(constituent, " ".to_string())
                                                    .await?;
                                                self.increment_char_index();
                                            }
                        PlaceHolderType::NewLine => {
                                                let result = self
                                                    .read_placeholder_as_string(constituent, "\r\n".to_string())
                                                    .await?;
                                                self.increment_line_index();

                                            }
                        PlaceHolderType::Delimiter(delim) => {
                                                let result = self
                                                    .read_placeholder_as_string(constituent, delim.to_string())
                                                    .await?;
                                                self.increment_char_index();
                                            }
                        PlaceHolderType::Bytes => todo!(),
                    }
                    i += 1;
                }
            }
        }
        Ok(())
    } */

    fn error_token_expected_eof_reached(&mut self) -> ParserError {
        ParserError::TokenExpected {
            line_index: self.line_index,
            char_index: self.get_error_char_index(),
            message: "Expected token not found, EOF reached"
                .to_string(),
        }
    }

    #[allow(unused)]
    fn error_unexpected_token_found(&mut self, unexpected_token: String) -> ParserError {
        ParserError::TokenExpected {
            line_index: self.line_index,
            char_index: self.get_error_char_index(),
            message: format!("Expected token not found, instead found token {}", unexpected_token)
                .to_string(),
        }
    }

    
    
    /* async fn read_delimited_string<RI:InfoProvider>(
        &mut self,
        placeholder: &Placeholder,
        constituents: &Vec<Placeholder>,
        i: &mut usize,
        info_provider: &RI,
    ) -> Result<Option<Value>, ParserError> {
        let delimiter = self.get_delimiter(constituents, i).await?;
        let result = self
        .read_placeholder_until(placeholder, delimiter.to_owned(), info_provider)
        .await?;
    Ok(result)
        
    } */

    /* async fn get_delimiter(
        &mut self,
        constituents: &Vec<Placeholder>,
        i: &mut usize,
    ) -> Result<String, ParserError> {
        if constituents.len() > *i + 1 {
            let delimiter = match &constituents[*i + 1].place_holder_type {
                PlaceHolderType::Delimiter(delimiter) => delimiter,
                PlaceHolderType::NewLine => "\r\n",

                PlaceHolderType::Space => " ",
                PlaceHolderType::ExactString(input) => input,
                
                _ => {
                    return Err(ParserError::InvalidPlaceHolderTypeFound {
                        line_index: self.line_index,
                        char_index: self.get_error_char_index(),
                        message: "Expected one of the delimiter type or known string, but found PlaceHolderType of Composite".to_string(),
                    });
                }
            };
            return Ok(delimiter.to_owned());
            /*
            let result = self
                .read_placeholder_until(placeholder, delimiter.to_owned())
                .await?;
            *i += 1;
            return Ok(());
             */
        } else {
            return Err(ParserError::InvalidPlaceHolderTypeFound { 
                line_index: self.line_index,
                char_index: self.get_error_char_index(),
                message: "Expected one of the delimiter type or known string, but reached end of child placeholders".to_string(),
            });
        }
    }
} */

/* fn handle_type<RI>(
    request_info: &mut RI,
    key: &mut Option<String>,    
    constituent: &Placeholder,
    result: Value,
    data: Option<String>,
    line_pos: usize,
    char_pos: usize,
) -> Result<(), ParserError>
where RI: InfoProvider {
    match &constituent.name {
        Key => {
            update_key(key, constituent, data);
            return Ok(());
        },
        InlineKeyWithValue(key_name) => {
            update_key(key, constituent, Some(key_name.to_string()));
            if key.is_none() {
                return Err(ParserError::MissingKey);
            }
            //value = Some(key_name.to_owned());
            request_info.add_info(key.as_ref().unwrap().to_owned(), result);
            return Ok(());
        }
        Value => {
            //update_key(key, constituent, Some(data.to_string()));
            if key.is_none() {
                return Err(ParserError::MissingKey);
            }
            //value = Some(data.to_owned());
            request_info.add_info(key.as_ref().unwrap().to_owned(), result);
            return Ok(());
        }

    _ => {
        Ok(())
    }


}*/
} 

/* fn update_key_value1<RI>(
    request_info: &mut RI,
    key: &mut Option<String>,    
    constituent: &Placeholder,
    result: Value,
    line_pos: usize,
    char_pos: usize,
) -> Result<(), ParserError>
where
    RI: InfoProvider,
{
    let result = match &result {
        Value::String(data) => match &constituent.name {
            Key => {
                update_key(key, constituent, Some(data.to_string()));
            }
            InlineKeyWithValue(key_name) => {
                update_key(key, constituent, Some(key_name.to_string()));
                if key.is_none() {
                    return Err(ParserError::MissingKey);
                }
                //value = Some(key_name.to_owned());
                request_info.add_info(key.as_ref().unwrap().to_owned(), result);
            }
            Value => {
                update_key(key, constituent, Some(data.to_string()));
                if key.is_none() {
                    return Err(ParserError::MissingKey);
                }
                //value = Some(data.to_owned());
                request_info.add_info(key.as_ref().unwrap().to_owned(), result);
            }

            _ => {}
        },
        _ => {
            Err(
                ParserError::TokenExpected {
                line_index: line_pos,
                char_index: char_pos,
                message: "Expected String token not found".to_string(),
            })?;
        }
    };
    Ok(result)
} */


/* fn update_key_value<RI>(
    request_info: &mut RI,
    key: &mut Option<String>,    
    constituent: &Placeholder,
    result: Value,
    line_pos: usize,
    char_pos: usize,
) -> Result<(), ParserError>
where
    RI: InfoProvider,
{
    match &result {
        Value::String(data) => {
            let data = data.to_owned();
            handle_type(request_info, key, constituent, result, Some(data), line_pos, char_pos)
        },
        Value::U8Vec(data) => {
            handle_type(request_info, key, constituent, result, None, line_pos, char_pos)
        },
        _ => {
            Err(
                ParserError::TokenExpected {
                line_index: line_pos,
                char_index: char_pos,
                message: "Expected String token not found".to_string(),
            })?
        }
    }
    
} */

#[cfg(test)]
mod tests {

    
    use tokio::io::BufReader;
    use tokio_stream::StreamExt;

    use crate::core::protocol_reader::ProtoStream;
    
    use crate::test_utils::assert_result_has_string;
    

    
    use super::{PlaceHolderRead, ProtocolBuffReader};
    

    #[tokio::test]
    async fn test_read_string_until() {
        let data = b"Hello World::";
        //let request_info = TestRequestInfo::new();
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);
        let result = protocol_reader
            .read_placeholder_until(                
                "::".to_string())
            .await;
        let bytes = result.unwrap().unwrap();        
        assert_eq!(String::from_utf8(bytes).unwrap(), "Hello World".to_string());

        //assert_result_has_string(result, "Hello World".to_string());
    }

    #[tokio::test]
    async fn test_read_string_until_delimiter_missing() {
        let data = b"Hello World";
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);        
        let result = protocol_reader
            .read_placeholder_until(
                
                "::".to_string(),
            )
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_read_string_until_delimiter_as_prefix() {
        let data = b"::Hello World";
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);

        let result = protocol_reader
            .read_placeholder_until(                
                "::".to_string(),
            )
            .await;
        assert_result_has_string(result, "".to_string());
    }

    #[tokio::test]
    async fn test_read_string_until_space_asdelimiter() {
        let data = b"Hello World\n";
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);
        let result = protocol_reader
            .read_placeholder_until(
                
                " ".to_string(),
            )
            .await;
        assert_result_has_string(result, "Hello".to_string());

        let result = protocol_reader
            .read_placeholder_until(                
                "\n".to_string(),
            )
            .await;
        assert_result_has_string(result, "World".to_string());
    }

    #[tokio::test]
    async fn test_read_string() {
        let data = b"Hello World\n";
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);
        let result = protocol_reader
            .read_placeholder_as_string(
                "Hello".to_string(),
            )
            .await;
        assert_result_has_string(result, "Hello".to_string());

        let result = protocol_reader
            .read_placeholder_as_string(                
                " ".to_string(),
            )
            .await;
        assert_result_has_string(result, " ".to_string());

        let result = protocol_reader
            .read_placeholder_as_string(                
                "World".to_string(),
            )
            .await;
        assert_result_has_string(result, "World".to_string());

        let result = protocol_reader
            .read_placeholder_as_string(                
                "\n".to_string(),
            )
            .await;
        assert_result_has_string(result, "\n".to_string());
    }

    #[tokio::test]
    async fn test_proto_reader_mark_reset() {
        let data = b"Hello World\n";
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);
        protocol_reader.mark();
        protocol_reader
            .read_placeholder_as_string(
                
                "Hello".to_string(),
            )
            .await
            .unwrap();
        protocol_reader.reset();

        let value = protocol_reader
            .read_placeholder_as_string(
                "Hello".to_string(),
            )
            .await
            .unwrap();
        protocol_reader.reset();
        
        match value {
            Some(value) => {
                let str_value = String::from_utf8(value).unwrap();    
                assert!(str_value == "Hello".to_string());
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_proto_reader_stream_conversion() {
        let data = b"Hello World\n";
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);

        protocol_reader
            .read_placeholder_as_string(                
                "Hello".to_string(),
            )
            .await
            .unwrap();

        let mut stream: ProtoStream<_> = protocol_reader.into();
        let space = stream.next().await.unwrap().unwrap();
        assert_eq!(space, b' ');

        let w = stream.next().await.unwrap().unwrap();
        assert_eq!(w, b'W');

        let o = stream.next().await.unwrap().unwrap();
        assert_eq!(o, b'o');

        let r = stream.next().await.unwrap().unwrap();
        assert_eq!(r, b'r');

        let l = stream.next().await.unwrap().unwrap();
        assert_eq!(l, b'l');

        let d = stream.next().await.unwrap().unwrap();
        assert_eq!(d, b'd');

        let new_line = stream.next().await.unwrap().unwrap();
        assert_eq!(new_line, b'\n');

        let eof = stream.next().await;
        assert!(eof.is_none());
    }

    #[tokio::test]
    async fn test_stream_to_proto_reader_conversion() {
        let data = b"Hello World\n";
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);
        protocol_reader.mark();
        protocol_reader
            .read_placeholder_as_string(                
                "Hello".to_string(),
            )
            .await
            .unwrap();

        let mut stream: ProtoStream<_> = protocol_reader.into();
        let space = stream.next().await.unwrap().unwrap();
        assert_eq!(space, b' ');

        let w = stream.next().await.unwrap().unwrap();
        assert_eq!(w, b'W');

        let o = stream.next().await.unwrap().unwrap();
        assert_eq!(o, b'o');

        let r = stream.next().await.unwrap().unwrap();
        assert_eq!(r, b'r');
        let mut proco_reader: ProtocolBuffReader<_> = stream.into();

        let data = proco_reader
            .read_placeholder_as_string(                
                "ld\n".to_string(),
            )
            .await
            .unwrap();
        match data {
            Some(value) => {
                assert!(String::from_utf8(value).unwrap() == "ld\n".to_string());
            }
            _ => {
                assert!(false);
            }
        }
    }

    /* #[tokio::test]
    async fn parse_composite_test() {
        let root_placeholder =
            Placeholder::new(Name("root".to_string()), None, PlaceHolderType::Composite, false);

        let mut spec_builder = SpecBuilder(root_placeholder);

        let request_line_placeholder = SpecBuilder::new_composite("request_line".to_string(), false)
            .expect_one_of_string(
                vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "DELETE".to_string(),
                    "PUT".to_string(),
                    "OPTIONS".to_string(),
                ],
                InlineKeyWithValue("request_method".to_string()),
                false
            )
            .expect_space()
            .expect_string(InlineKeyWithValue("request_uri".to_string()),false)
            .expect_space()
            .expect_string(InlineKeyWithValue("protocol_version".to_string()), false)
            .expect_newline()
            .build();

        let mut header_placeholder_builder = SpecBuilder::new_composite("header".to_string(), false);
        let header_place_holder = header_placeholder_builder
            .expect_string(crate::core::PlaceHolderIdentifier::Key, false)
            .expect_delimiter(": ".to_string())
            .expect_string(crate::core::PlaceHolderIdentifier::Value, false)
            .expect_newline()
            .build();

        spec_builder.expect_composite(request_line_placeholder, "first_line".to_owned());
        spec_builder.expect_newline();
        spec_builder.expect_repeat_many(header_place_holder, "headers".to_owned());
        spec_builder.expect_exact_string(InlineKeyWithValue("data".to_string()), "test123".to_string(), false);
        spec_builder.expect_newline();

        let placehoder = spec_builder.build();
        let mut protocol_reader = ProtocolBuffReader::new(
            BufReader::new(
                b"GET /index.html HTTP/1.1\r\n\r\nname: value\r\nname2: value2\r\ntest123\r\n".as_ref(),
            ),
            1024,
        );
        let mut request_info = TestRequestInfo::default();
        let result = protocol_reader
            .parse_composite(&mut request_info, &placehoder, )
            .await;
        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let request_method = request_info.get_info(&"request_method".to_string()).unwrap();
        match request_method {
            crate::core::Value::String(value) => {
                assert!(*value == "GET".to_string());
            }
            _ => {
                assert!(false);
            }
        }

        match request_info.get_info(&"name".to_owned()).unwrap() {
            crate::core::Value::String(value) => {
                assert!(*value == "value".to_string());
            }
            _ => {
                assert!(false);
            }
        }

        match request_info.get_info(&"name2".to_owned()).unwrap() {
            crate::core::Value::String(value) => {
                assert!(*value == "value2".to_string());
            }
            _ => {
                assert!(false);
            }
        }

        match request_info.get_info(&"data".to_owned()).unwrap() {
            crate::core::Value::String(value) => {
                assert!(*value == "test123".to_string());
            }
            _ => {
                assert!(false);
            }
        }
    } */


    
    /* #[tokio::test]
    async fn test_read_unexpected_token_error() {
        let data = b"Hello World\n";
        let mut spec = SpecBuilder::new("root".to_owned());
        let root = spec.expect_string(crate::core::PlaceHolderIdentifier::InlineKeyWithValue("first_word".to_string()), false)
            .expect_space()
            .expect_exact_string(InlineKeyWithValue("second_word".to_string()), "World".to_string(), false)
            .expect_space()
            .build();
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);

        let mut request_info = TestRequestInfo::new();
        let result = protocol_reader.parse_composite(&mut request_info, &root).await;
        

        
        match result{
            Ok(_) => {
                assert!(false, "expected unexpected token error, but got success");
            }
            Err(e   ) => {
                match e {
                    crate::core::ParserError::TokenExpected { line_index, char_index, message } => {
                        assert!(line_index == 0);
                        assert!(char_index == 11);
                        assert!(message.contains( "Expected token not found"));
                    }
                    _ => {
                        assert!(false, "expected unexpected token error, but got error");
                    }
                    
                }

            }
        }
    }

    #[tokio::test]
    async fn test_read_token_eof_error() {
        let data = b"Hello World\r\n";
        let mut spec = SpecBuilder::new("root".to_owned());
        let root = spec.expect_string(crate::core::PlaceHolderIdentifier::InlineKeyWithValue("first_word".to_string()), false)
            .expect_space()
            .expect_exact_string(InlineKeyWithValue("second_word".to_string()), "World".to_string(),false)
            .expect_newline()
            .expect_newline()
            .build();
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);

        let mut request_info = TestRequestInfo::new();
        let result = protocol_reader.parse_composite(&mut request_info, &root).await;
        

        
        match result{
            Ok(_) => {
                assert!(false, "expected unexpected token error, but got success");
            }
            Err(e   ) => {
                match e {
                    crate::core::ParserError::TokenExpected { line_index, char_index, message } => {
                        assert!(line_index == 1);
                        assert!(char_index == 0);
                        assert!(message.contains( "EOF reached"));
                    }
                    _ => {
                        assert!(false, "expected unexpected token error, but got error");
                    }
                    
                }

            }
        }
    } */


    /* #[tokio::test]
    async fn test_proto_optional_test() {
        let data = b"Hello \r\n";
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);
        let spec = SpecBuilder::new("root".to_owned())
            .expect_string(crate::core::PlaceHolderIdentifier::InlineKeyWithValue("first_word".to_string()), false)
            .expect_space()
            .expect_exact_string(InlineKeyWithValue("second_word".to_string()), "World".to_string(), true)
            .expect_newline()
            .build();

        let mut request_info = TestRequestInfo::new();
        let result = protocol_reader.parse_composite( &mut request_info, &spec).await;
        match result {
            Ok(_) => {
                let first_word = request_info.get_info(&"first_word".to_string()).unwrap();
                match first_word {
                    crate::core::Value::String(value) => {
                        assert!(*value == "Hello".to_string());
                    }
                    _ => {
                        assert!(false);
                    }
                }
                let second_word = request_info.get_info(&"second_word".to_string());
                
                assert!(second_word.is_none());
            }
            Err(e) => {
                assert!(false, "expected success, but got error: {:?}", e);
            }
        }

        
        
    } */


    
}
