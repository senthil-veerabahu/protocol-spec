use async_trait::async_trait;
use memchr::memmem::Finder;
use pin_project::pin_project;
use tracing::debug;
use std::{
     fmt::Display, future::Future, io::{self, ErrorKind}, pin::Pin, task::{Context, Poll}, time::Duration
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
            self.buff_reader.consume_and_drain(1);
            Poll::Ready(Some(Ok(value)))
        } else {
            match self.buff_reader.fill_buffer(cx) {
                Poll::Ready(Ok(len)) => {
                    if len == 0 {
                        return Poll::Ready(None);
                    }
                    let value = self.buff_reader.buf[self.buff_reader.pos];
                    self.buff_reader.consume(1);
                    Poll::Ready(Some(Ok(value)))
                }
                Poll::Ready(Err(err)) => {
                    Poll::Ready(Some(Err(err)))
                }
                Poll::Pending => {
                    Poll::Pending
                }
            }
        }
    }
}

#[async_trait]
pub trait PlaceHolderRead {
    #[allow(unused)]
    async fn read_placeholder_as_string(
        & mut self,
        input: String,
        
    ) -> Result<Option<Vec<u8>>, ParserError>;

    #[allow(unused)]
    async fn read_placeholder_until(
        &mut self,        
        delimiter: String,        
        
    ) -> Result<Option<Vec<u8>>, ParserError>;


    #[allow(unused)]
    async fn read_bytes(
        & mut self,
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
pub(crate) struct ProtocolBuffReader<R>
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
    fn into_buff_reader(self) -> ProtocolBuffReader<R> {
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
        !self.markers.is_empty()
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
    /* fn mark(&mut self) {
        self.marked_pos = self.pos;
        self.marked = true;
    }

    fn reset(&mut self) {
        self.pos = self.marked_pos;
        self.marked = false;
    }

    fn unmark(&mut self) {
        self.marked = false;
    } */

    fn fill_buffer(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        //let mut pinned_self = self.project();
        let pinned_reader = Pin::new(&mut self.inner);
        let result = pinned_reader.poll_fill_buf(cx);
        let mut len = 0;
        let result = match result {
            Poll::Ready(Ok(buf)) => {

                if buf.is_empty() {
                    return Poll::Ready(Err(io::Error::new(ErrorKind::UnexpectedEof, "End Of file reached")));
                }
                debug!("len {}, cap {}", self.buf.len(), self.cap);
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
        result
    }
    
    fn consume_and_drain(&mut self, amount: usize) {
        if self.pos >= self.buf.len() / 2 && !self.marked {
            self.buf.drain(0..self.pos + amount);
            self.pos = 0;
        } else {
            self.pos += amount;
        }
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

    #[allow(unused)]
    fn increment_line_index(&mut self) {
        self.line_index += 1;
        self.char_index_in_line = 0;
        self.char_index += 1;
    }
    
    #[allow(unused)]
    fn increment_char_index(&mut self) {        
        self.char_index_in_line += 1;
        self.char_index += 1;
    }

    fn get_error_char_index(&self) -> usize {
        self.char_index_in_line + 1
    }

    #[allow(unused)]
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
        debug!("-{}-", String::from_utf8_lossy(data));
        data
        
    }

    #[allow(unused)]
    fn into_stream(self) -> ProtoStream<R>
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

    #[allow(unused)]
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

fn convert_io_error(error: std::io::Error) -> ParserError{
    match error.kind(){
        ErrorKind::UnexpectedEof => {
            ParserError::EndOfStream
        },
        _ => {
            ParserError::IOError { error }
        }
    }
}

impl <R> Future for ReadBytes<'_,  R>
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
            if buf.is_empty() || !protocol_reader.buf_has_enough_data(read_bytes_expected_size) {
                let result = protocol_reader.fill_buffer(cx);
                match result {
                    Poll::Ready(Ok(read_length)) => {
                        if read_length > 0 {
                            continue;
                        } else if let ReadBytesSize::Fixed(_) = read_bytes_expected_size {                                
                                return Poll::Ready(Ok(None));
                        }
                    }
                    Poll::Pending => {
                        //todo: check if pending needs to be handled differently
                        if let ReadBytesSize::Fixed(_) = read_bytes_expected_size {
                            return Poll::Pending;
                        }
                    }
                    Poll::Ready(Err(e)) => {
                        return Poll::Ready(Err(convert_io_error(e)));
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

impl<R> Future for ReadPlaceHolderUntil<'_, R>
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
                                protocol_reader.get_buffer()[protocol_reader.pos..index].to_vec();
                            protocol_reader.consume_and_drain(matched_portion.len() + delimiter.len());                                
                            /* let place_holder_value = PlaceHolderValue::parse(
                                &placeholder.place_holder_type,
                                matched_portion,
                            ); */

                            
                            //protocol_reader.unmark_if_optional(placeholder);
                            return Poll::Ready(Ok(Some(matched_portion)));
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

impl<R> Future for ReadString<'_, R>
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
                            protocol_reader.get_buffer()[index..index + input.len()].to_vec();
                        /* let place_holder_value = PlaceHolderValue::parse(
                            &placeholder.place_holder_type,
                            matched_portion,
                        );*/

                        protocol_reader.consume_and_drain(input.len());
                        //protocol_reader.unmark_if_optional(placeholder);
                        return Poll::Ready(Ok(Some(matched_portion)));
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
        if buf.is_empty() || !protocol_reader.buf_has_enough_data(&ReadBytesSize::Fixed(input.len() as u32)) {
            let result = protocol_reader.fill_buffer(cx);
            match result {
                Poll::Ready(Ok(read_length)) => {
                    if read_length > 0 {
                        continue;
                    } else {
                        return Some(Poll::Ready(Err(ParserError::EndOfStream)));
                    }
                }
                Poll::Pending => return Some(Poll::Pending),
                Poll::Ready(Err(e)) => {
                    return Some(Poll::Ready(Err(convert_io_error(e))));
                }
            };
        }
        let buf = protocol_reader.get_buffer();
        let pos = protocol_reader.pos;
        debug!("pos is {}", pos);
        debug!("input get bytes {:?}, len {}", input.as_bytes(), input.len());
        debug!("buf  get byets {:?}, len {}", &buf[pos..pos + input.len()], &buf[pos..pos + input.len()].len());
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
        let data = timeout(Duration::from_millis(300), ReadPlaceHolderUntil::new(self, delimiter)).await;
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
        let data = timeout(Duration::from_millis(300),ReadString::new(self, input)).await;
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
            let data = timeout(Duration::from_millis(300), ReadBytes::new(self, size)).await;
            match data {
                Ok(Ok(data)) => Ok(data),
                Ok(Err(e)) => Err(e),
                Err(_e) => Err(ParserError::EndOfStream),
            }
    }
}

#[allow(unused)]
fn is_eof_error(parse_error: &ParserError) -> bool {
    match parse_error {
        ParserError::IOError { error } => {
            error.kind() == io::ErrorKind::UnexpectedEof
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
}

#[cfg(test)]
mod tests {
    use tokio::io::BufReader;
    use tokio_stream::StreamExt;
    use tracing::{debug, warn};
    use crate::core::builders::{new_spec_builder, CompositeBuilder, DelimitedStringSpecBuilder, DelimiterBuilder, InlineValueBuilder, KeySpecBuilder, ProtoSpecBuilder, RepeatBuilder,  ValueBuilder, StringSpecBuilder};
    use crate::core::{ DefaultSerializer, InfoProvider, RequestSerializer };
    use crate::core::{protocol_reader::ProtoStream, SpecName};
    
    use crate::mapping_extractor::{DefaultMapper, SpecTraverse};
    use crate::test_utils::{assert_result_has_string, TestRequestInfo};
    

    
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
        assert!(eof.is_some(), "Got None, but expected a value");
        if let Some(result) = eof {
            match result{
                Ok(_) => todo!(),
                Err(e) => {
                    let kind = e.kind();
                    assert!(kind == std::io::ErrorKind::UnexpectedEof, "Expected eof error, found {:?}", e);
                    /* match kind{
                        
                        std::io::ErrorKind::UnexpectedEof => {
                            assert!(true, "received eof as expected");
                        },
                        _ => {
                            assert!(false, "End of file excepted, found {}", e.to_string());
                        },
                    } */
                },
            }
        }
        
    }

    #[tokio::test]
    async fn test_stream_to_proto_reader_conversion() {
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
        let mut proco_reader: ProtocolBuffReader<_> = stream.into();

        let data = proco_reader
            .read_placeholder_as_string(                
                "ld\n".to_string(),
            )
            .await
            .unwrap();

        assert!(data.is_some());
        if let Some(value) = data {
            assert!(String::from_utf8(value).unwrap() == "ld\n");
        }

        
    }

    #[tokio::test]
    async fn parse_composite_test() {
        let spec_builder =
            new_spec_builder(SpecName::Name("root".to_string()));

        let request_line_placeholder = new_spec_builder(SpecName::NoName)
            .inline_value_follows(SpecName::NoName, false)
            .expect_one_of_string(
                SpecName::Name("request_method".to_owned()), false,
                vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "DELETE".to_string(),
                    "PUT".to_string(),
                    "OPTIONS".to_string(),
                ],                
            )
            .delimited_by_space()
            .inline_value_follows(SpecName::NoName, false)
            .expect_string(SpecName::Name("request_uri".to_string()),false)
            .delimited_by_space()
            .inline_value_follows(SpecName::NoName, false)
            .expect_string(SpecName::Name("protocol_version".to_string()),false)
            .delimited_by_newline()
            .build();

        let header_placeholder_builder = new_spec_builder(SpecName::Name("header".to_string()));
        let header_place_holder = header_placeholder_builder
            .key_follows(SpecName::Name("header_name".to_owned()), true)
            .expect_string(SpecName::NoName, false)
            .delimited_by(": ".to_string())
            .value_follows(SpecName::Name("header_value".to_owned()), false)
            .expect_string(SpecName::NoName, false)
            .delimited_by_newline()
            .build();

        let spec_builder = spec_builder.expect_composite(request_line_placeholder);
        let spec_builder = spec_builder.expect_newline();
        let spec_builder = spec_builder.repeat_many(SpecName::NoName, false, crate::core::Separator::Delimiter("\r\n".to_owned()), header_place_holder);

        let spec_builder = spec_builder.inline_value_follows(SpecName::NoName, false)
        .expect_exact_string(SpecName::Name("data".to_string()), "test123".to_string(), false);
        let spec_builder = spec_builder.expect_newline();

        
        let mut protocol_reader = ProtocolBuffReader::new(
            BufReader::new(
                b"GET /index.html HTTP/1.1\r\n\r\nname: value\r\nname2: value2\r\n\r\ntest123\r\n".as_ref(),
            ),
            1024,
        );

        let spec =spec_builder.build();
        
        let mut request_info = TestRequestInfo::new();

        let mut mapper = DefaultMapper::new();
        let result = spec.traverse(&mut mapper );
        assert!(result.is_ok());
        debug!("{:?}", &mut mapper);
        request_info.0 = mapper;

        

        let result = DefaultSerializer{}.deserialize_from(&mut request_info, &mut protocol_reader, &spec).await;
        /* let result = protocol_reader
            .parse_composite(&mut request_info, &placehoder, )
            .await; */
        debug!("Result: {:?}", result);
        assert!(result.is_ok());
        let request_method = request_info.get_info("request_method").unwrap();
        if let crate::core::Value::String(value) = request_method {
            assert!(value == "GET");
        }

        if let crate::core::Value::String(value) = request_info.get_key_value_info_by_spec_name("name".to_owned(), &"header_name".to_owned()).unwrap() {
            assert!(*value == "value");
        }

        if let crate::core::Value::String(value) = request_info.get_key_value_info_by_spec_name("name2".to_owned(), &"header_name".to_owned()).unwrap() {
            assert!(*value == "value2");
        } 

        if let crate::core::Value::String(value) = request_info.get_info("data").unwrap() {
            assert!(*value == "test123");
        }
    }


    
    #[tokio::test]
    async fn test_read_unexpected_token_error() {
        let data = b"Hello World\n";
        let spec = new_spec_builder(SpecName::NoName);
        let root = spec.inline_value_follows(SpecName::NoName, false)
            .expect_string(SpecName::Name("first_word".to_string()), false)
            .delimited_by_space()
            .inline_value_follows(SpecName::NoName, false)
            .expect_exact_string(SpecName::Name("second_word".to_string()), "World".to_string(), false)
            .expect_space()
            .build();
        let protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);

        let mut request_info = TestRequestInfo::new();
        let mut mapper = DefaultMapper::new();
        assert!(root.traverse(&mut mapper).is_ok());
        request_info.0 = mapper;
        //let result = protocol_reader.parse_composite( &mut request_info, &spec).await;
        let result = DefaultSerializer{}.deserialize_from(&mut request_info, protocol_reader, &root).await;
        
        assert!(result.is_err(), "expected unexpected token error, but got success");
        #[allow(unused)]        
        if let Err(e   ) = result {
            match e {
                crate::core::ParserError::TokenExpected { line_index, char_index, message } => {
                    /* assert!(line_index == 0);
                    assert!(char_index == 11); */
                    assert!(message.contains( "Expected token not found"));
                }
                _ => {
                    assert!(!matches!(e, crate::core::ParserError::TokenExpected { .. }), "expected unexpected token error, but got error");
                }
                
            }

        }
    }

    #[tokio::test]
    async fn test_read_token_eof_error() {
        let data = b"Hello World\r\n";
        let spec = new_spec_builder(SpecName::NoName);
        let root = spec
            .inline_value_follows(SpecName::NoName, false)
            .expect_string(SpecName::Name("first_word".to_string()), false)
            .delimited_by_space()
            .inline_value_follows(SpecName::NoName, false)
            .expect_exact_string(SpecName::Name("second_word".to_string()), "World".to_string(),false)
            .expect_newline()
            .expect_newline()
            .build();
        let protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);

        let mut request_info = TestRequestInfo::new();
        let mut mapper = DefaultMapper::new();
        assert!(root.traverse(&mut mapper ).is_ok());
        request_info.0 = mapper;
        //let result = protocol_reader.parse_composite( &mut request_info, &spec).await;
        let result = DefaultSerializer{}.deserialize_from(&mut request_info, protocol_reader, &root).await;
        

        assert!(result.is_err(), "expected unexpected token error, but got success");
        
            
        if let Err(e   ) = result {
            warn!("Error received {}", e);
            assert!(matches!(e, crate::core::ParserError::EndOfStream), "expected EndOfStream error, but got different error {:?}", e);
        }
        
    } 


    #[tokio::test]
    async fn test_proto_optional_test() {
        let data = b"Hello \r\n";
        let protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);
        let spec = new_spec_builder(SpecName::NoName)
        .inline_value_follows(SpecName::Name("first_word".to_string()), false)
        
            .expect_string(SpecName::NoName, false)
            .delimited_by_space()
            . inline_value_follows(SpecName::Name("second_word".to_string()), true)
            .expect_exact_string(SpecName::NoName, "World".to_string(), true)
            /* .expect_exact_string(SpecName::NoName, "World".to_string(), true) 
            .expect_newline() */
            .build(); 

        let mut request_info = TestRequestInfo::new();
        let mut mapper = DefaultMapper::new();
        assert!(spec.traverse(&mut mapper ).is_ok());
        request_info.0 = mapper;
        //let result = protocol_reader.parse_composite( &mut request_info, &spec).await;
        let result = DefaultSerializer{}.deserialize_from(&mut request_info, protocol_reader, &spec).await;
        assert!(result.is_ok(), "expected success, but got error {:?}", result.err());
        if result.is_ok() {
            let first_word = request_info.get_info("first_word").unwrap();
            assert!(matches!(first_word, crate::core::Value::String(_)), "expected a string value, but received {:?}", first_word);
            if let crate::core::Value::String(value) = first_word {
                assert!(*value == "Hello");
            }
                
            let second_word = request_info.get_info("second_word");
            assert!(second_word.is_none());
        }
    }
}
