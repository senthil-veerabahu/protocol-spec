use memchr::memmem::Finder;
use pin_project::pin_project;
use std::{
     future::Future, io::{self}, pin::Pin, task::{Context, Poll}
};
use tokio::io::{AsyncBufRead, AsyncRead, ReadBuf};
use tokio_stream::Stream;

use crate::core::{
    ParserError,
    PlaceHolderIdentifier::{InlineKeyWithValue, Key, Value},
    PlaceHolderType, PlaceHolderValue, Placeholder, RequestInfo, ValueType,
};

/* fn poll_test<'a, R, F, T>(reader: &mut ProtocolBuffReader<R>, cx:&mut Context<'a>) -> F where R: AsyncBufRead + Unpin, F: FnMut(&mut Context<'_>) -> Poll<T>, {
    |cx| {
        let mut pinned_reader = Pin::new(reader);
        pinned_reader.poll_next(cx)
    }

} */

/*
#[pin_project]
 struct StreamIterator<R>(&mut ProtocolBuffReader<R>) where R: AsyncBufRead + Unpin;

impl <R> Future for StreamIterator<R> where  R: AsyncBufRead + Unpin {
    type Output = Option<io::Result<u8>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {

        let mut pinned_self = self.project();
        let mut pinned_reader = Pin::new(&mut pinned_self.0);
        let result = pinned_reader.poll_next(cx);
        match result {
            Poll::Ready(Some(Ok(value))) => {
                return Poll::Ready(Some(Ok(value)));
            },
            Poll::Ready(Some(Err(err))) => {
                return Poll::Ready(Some(Err(err)));
            },
            Poll::Ready(None) => {
                println!("EOF");
                return Poll::Ready(None);
            },
            Poll::Pending => {
                return Poll::Pending;
            }
        }
    }
} */

impl<R> Stream for ProtoStream<R>
where
    R: AsyncBufRead + Unpin,
{
    type Item = io::Result<u8>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        if self.buff_reader.pos < self.buff_reader.buf.len() {
            let value = self.buff_reader.buf[self.buff_reader.pos];
            self.buff_reader.consume(1);
            return Poll::Ready(Some(Ok(value)));
        } else {
            match self.buff_reader.fill_buf(cx) {
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

trait PlaceHolderRead {
    #[allow(unused)]
    async fn read_placeholder_as_string<'a, 'b>(
        self: &'a mut Self,
        placeholder: &'b Placeholder,
        input: String,
    ) -> Result<ValueType, ParserError>
    where
        'a: 'b;

    #[allow(unused)]
    async fn read_placeholder_until<'a, 'b>(
        self: &'a mut Self,
        placeholder: &'b Placeholder,
        delimiter: String,
    ) -> Result<ValueType, ParserError>
    where
        'a: 'b;
}

#[pin_project]
pub(super) struct ProtocolBuffReader<R>
where
    R: AsyncBufRead + Unpin,
{
    #[pin]
    inner: R,
    cap: usize,
    pos: usize,
    buf: Vec<u8>,
    marked_pos: usize,
    marked: bool,
    line_count: usize,
    char_pos: usize,
    char_pos_line: usize,
}

pub struct ProtoStream<R>
where
    R: AsyncBufRead + Unpin,
{
    buff_reader: ProtocolBuffReader<R>,
}

impl<R> From<ProtocolBuffReader<R>> for ProtoStream<R>
where
    R: AsyncBufRead + Unpin,
{
    fn from(buff_reader: ProtocolBuffReader<R>) -> Self {
        ProtoStream { buff_reader }
    }
}

impl<R> ProtoStream<R>
where
    R: AsyncBufRead + Unpin,
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
    R: AsyncBufRead + Unpin,
{
    fn from(value: ProtoStream<R>) -> Self {
        value.buff_reader
    }
}

impl<R> AsyncRead for ProtocolBuffReader<R>
where
    R: AsyncBufRead + Unpin,
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

impl<R> AsyncBufRead for ProtocolBuffReader<R>
where
    R: AsyncBufRead + Unpin,
{
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        //let mut pinned_self = self.project();
        let me = self.get_mut();
        //let  pinned_reader = Pin::new(&mut pinned_self.inner);
        match me.fill_buf(cx) {
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

impl<R> ProtocolBuffReader<R>
where
    R: AsyncBufRead + Unpin,
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

    fn consume(&mut self, amount: usize) {
        if self.pos + amount > self.cap {
            self.buf.clear();
            self.pos = 0;
        }
        if self.pos >= self.buf.len() / 2 && !self.marked {
            self.buf.drain(0..self.pos + amount);
            self.pos = 0;
        } else {
            self.pos += amount;
        }
    }

    fn fill_buf(self: &mut Self, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
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

    fn buf_has_enough_data(&self, size: usize) -> bool {
        self.buf.len() - self.pos + 1 >= size
    }

    #[allow(unused)]
    fn new(reader: R, cap: usize) -> Self {
        ProtocolBuffReader {
            inner: reader,
            cap,
            pos: 0,
            buf: Vec::with_capacity(cap),
            marked_pos: 0,
            marked: false,
            line_count: 0,
            char_pos: 0,
            char_pos_line: 0,
        }
    }

    fn increment_line(&mut self) {
        self.line_count += 1;
        self.char_pos_line += 1;
        self.char_pos += 1;
    }
    fn increment_char_pos(&mut self) {        
        self.char_pos_line += 1;
        self.char_pos += 1;
    }

    fn increment_char_pos_by(&mut self, count:usize) {        
        self.char_pos_line += count;
        self.char_pos += count;
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
        &self.buf[self.pos..]
    }

    #[allow(unused)]
    fn as_stream<'a>(self) -> ProtoStream<R>
    where
        R: AsyncBufRead + Unpin,
    {
        self.into()
    }
}

#[pin_project]
struct ReadPlaceHolderUntil<'a, R>
where
    R: AsyncBufRead + Unpin,
{
    protocol_reader: &'a mut ProtocolBuffReader<R>,
    placeholder: &'a Placeholder,
    delimiter: String,
}

#[pin_project]
struct ReadString<'a, R>
where
    R: AsyncBufRead + Unpin,
{
    protocol_reader: &'a mut ProtocolBuffReader<R>,
    placeholder: &'a Placeholder,
    input: String,
}

impl<'a, R> ReadPlaceHolderUntil<'a, R>
where
    R: AsyncBufRead + Unpin,
{
    fn new(
        protocol_reader: &'a mut ProtocolBuffReader<R>,
        placeholder: &'a Placeholder,
        delimiter: String,
    ) -> Self {
        ReadPlaceHolderUntil {
            protocol_reader,
            placeholder,
            delimiter,
        }
    }
}

impl<'a, R> Future for ReadPlaceHolderUntil<'a, R>
where
    R: AsyncBufRead + Unpin,
{
    type Output = Result<ValueType, ParserError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut this = self.project();
        let placeholder = this.placeholder;
        let delimiter = this.delimiter;
        let protocol_reader = &mut this.protocol_reader;
        if protocol_reader.pos < (protocol_reader.cap - 1) {
            //let pinned_reader = Pin::new(&mut protocol_reader.inner);
            if let Some(value) = perform_search(cx, placeholder, delimiter, protocol_reader) {
                match value {
                    Poll::Ready(result) => match result {
                        Ok(index) => {
                            let matched_portion =
                                &protocol_reader.get_buffer()[protocol_reader.pos..index];
                            let place_holder_value = PlaceHolderValue::parse(
                                &placeholder.place_holder_type,
                                matched_portion,
                            );

                            protocol_reader.consume(matched_portion.len() + delimiter.len());
                            return Poll::Ready(Ok(place_holder_value));
                        }
                        Err(e) => {
                            return Poll::Ready(Err(e));
                        }
                    },
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                }
            }
            Poll::Ready(Err(protocol_reader.error_token_expected_eof_reached()))
        } else {
            Poll::Ready(Err(protocol_reader.error_token_expected_eof_reached()))
        }
    }
}

impl<'a, R> ReadString<'a, R>
where
    R: AsyncBufRead + Unpin,
{
    fn new(
        protocol_reader: &'a mut ProtocolBuffReader<R>,
        placeholder: &'a Placeholder,
        input: String,
    ) -> Self {
        ReadString {
            protocol_reader,
            placeholder,
            input,
        }
    }
}

impl<'a, R> Future for ReadString<'a, R>
where
    R: AsyncBufRead + Unpin,
{
    type Output = Result<ValueType, ParserError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut this = self.project();

        let placeholder = this.placeholder;
        let input = this.input;
        let protocol_reader = &mut this.protocol_reader;
        //if !protocol_reader.buf_has_enough_data(input.len()) {
        //let pinned_reader = Pin::new(&mut protocol_reader.inner);
        if let Some(value) = read_string(cx, placeholder, input, protocol_reader) {
            match value {
                Poll::Ready(result) => match result {
                    Ok(index) => {
                        let matched_portion =
                            &protocol_reader.get_buffer()[index..index + input.len()];
                        let place_holder_value = PlaceHolderValue::parse(
                            &placeholder.place_holder_type,
                            matched_portion,
                        );

                        protocol_reader.consume(input.len());
                        return Poll::Ready(Ok(place_holder_value));
                    }
                    Err(e) => {
                        return Poll::Ready(Err(e));
                    }
                },
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
        Poll::Ready(Err(ParserError::TokenExpected {
            line_pos: protocol_reader.line_count,
            char_pos: protocol_reader.char_pos_line,
            message: "Expected token not found, EOF reached".to_string(),
        }))
    }
}

#[allow(unused)]
fn token_expected_error(line_index:usize, line_char_pos:usize) -> ParserError {
    ParserError::TokenExpected {
        line_pos: line_index,
            char_pos: line_char_pos,
        message: "Expected token not found, EOF reached".to_string(),
    }
}

fn perform_search<R>(
    cx: &mut Context<'_>,
    _placeholder: &Placeholder,
    delimiter: &mut String,
    protocol_reader: &mut ProtocolBuffReader<R>,
) -> Option<Poll<Result<usize, ParserError>>>
where
    R: AsyncBufRead + Unpin,
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
                let result = protocol_reader.fill_buf(cx);
                match result {
                    Poll::Ready(Ok(read_length)) => {
                        if read_length > 0 {
                            continue;
                        } else {
                            return Some(Poll::Ready(Err(ParserError::TokenExpected {
                                line_pos: protocol_reader.line_count,
                                char_pos: protocol_reader.char_pos_line,
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
    _placeholder: &Placeholder,
    input: &mut String,
    protocol_reader: &mut ProtocolBuffReader<R>,
) -> Option<Poll<Result<usize, ParserError>>>
where
    R: AsyncBufRead + Unpin,
{
    loop {
        let buf = protocol_reader.get_buffer();
        if buf.len() == 0 || !protocol_reader.buf_has_enough_data(input.len()) {
            let result = protocol_reader.fill_buf(cx);
            match result {
                Poll::Ready(Ok(read_length)) => {
                    if read_length > 0 {
                        continue;
                    } else {
                        return Some(Poll::Ready(Err(ParserError::TokenExpected {
                            line_pos: protocol_reader.line_count,
                            char_pos: protocol_reader.char_pos_line,
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
        if &buf[pos..pos + input.len()] == input.as_bytes() {
            return Some(Poll::Ready(Ok(pos)));
        } else {
            return Some(Poll::Ready(Err(ParserError::TokenExpected {
                line_pos: protocol_reader.line_count,
                char_pos: protocol_reader.char_pos_line,
                message: "Expected token not found, EOF reached".to_string(),
            })));
        }
    }
}

impl<T> PlaceHolderRead for ProtocolBuffReader<T>
where
    T: AsyncBufRead + Unpin,
{
    async fn read_placeholder_until<'a, 'b>(
        self: &'a mut Self,
        placeholder: &'b Placeholder,
        delimiter: String,
    ) -> Result<ValueType, ParserError>
    where
        'a: 'b,
    {
        let data = ReadPlaceHolderUntil::new(self, placeholder, delimiter).await?;
        Ok(data)
    }

    async fn read_placeholder_as_string<'a, 'b>(
        self: &'a mut Self,
        placeholder: &'b Placeholder,
        input: String,
    ) -> Result<ValueType, ParserError>
    where
        'a: 'b,
    {
        let data = ReadString::new(self, placeholder, input).await?;
        Ok(data)
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
        }
        _ => false,
    }
}

fn update_key(key: &mut Option<String>, placeholder: &Placeholder, input_key_data: Option<String>) {
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
}

impl<R> ProtocolBuffReader<R>
where
    R: AsyncBufRead + Unpin,
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

    #[allow(unused)]
    async fn parse_composite<RI>(
        &mut self,
        placeholder: &Placeholder,
        request_info: &mut RI,
    ) -> Result<(), ParserError>
    where
        RI: RequestInfo,
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
                            Box::pin(self.parse_composite(constituent, request_info)).await?;
                        }
                        PlaceHolderType::RepeatN(n) => {
                            Box::pin(self.parse_composite(constituent, request_info)).await?;
                        }
                        PlaceHolderType::RepeatMany => {
                            let mut count = 0;
                            loop {
                                self.mark();
                                let result =
                                    Box::pin(self.parse_composite(constituent, request_info)).await;
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
                            let value_type = self
                                .read_placeholder_as_string(constituent, input.to_string())
                                .await?;
                            match &value_type {
                                ValueType::String(data) => {
                                    self.increment_char_pos_by(input.len());
                                }
                                _ => {}
                            }
                            update_key_value(
                                request_info,
                                &mut key,                               
                                constituent,
                                value_type,
                                self.line_count,
                                self.char_pos_line,
                            )?;
                        }
                        PlaceHolderType::AnyString => {
                            //Self::update_key(&mut key, constituent, None);
                            let value_type = self
                                .read_delimited_string(constituent, constituents, &mut i)
                                .await?;
                            match &value_type {
                                ValueType::String(data) => {
                                    self.increment_char_pos_by(data.len());
                                }
                                _ => {}
                            }
                            update_key_value(
                                request_info,
                                &mut key,                                
                                constituent,
                                value_type,
                                self.line_count,
                                self.char_pos_line,
                            )?;
                             
                            i += 1;
                        }
                        PlaceHolderType::Stream => {
                            // let x = Box::pin(self.as_stream());
                            todo!()
                        }
                        PlaceHolderType::OneOf(items) => {
                            let value_type = self
                                .read_delimited_string(constituent, constituents, &mut i)
                                .await?;
                            i += 1;

                            match &value_type {
                                ValueType::String(str) => {
                                    update_key(&mut key, constituent, Some(str.to_owned()));
                                    if items.contains(str) {
                                        self.increment_char_pos_by(str.len());
                                        request_info.add_info(key.unwrap(), value_type);
                                        key = None;
                                        
                                        //return Ok(());
                                    } else {
                                        return Err(self.error_token_expected_eof_reached());
                                    }
                                }
                                _ => {
                                    return Err(self.error_token_expected_eof_reached());
                                }
                            }
                        }
                        PlaceHolderType::Bytes => todo!(),
                        PlaceHolderType::Space => {
                            let result = self
                                .read_placeholder_as_string(constituent, " ".to_string())
                                .await?;
                            self.increment_char_pos();
                        }
                        PlaceHolderType::NewLine => {
                            let result = self
                                .read_placeholder_as_string(constituent, "\n".to_string())
                                .await?;
                            self.increment_line();

                        }
                        PlaceHolderType::Delimiter(delim) => {
                            let result = self
                                .read_placeholder_as_string(constituent, delim.to_string())
                                .await?;
                            self.increment_char_pos();
                        }
                    }
                    i += 1;
                }
            }
        }
        Ok(())
    }

    fn error_token_expected_eof_reached(&mut self) -> ParserError {
        ParserError::TokenExpected {
            line_pos: self.line_count,
            char_pos: self.char_pos_line,
            message: "Expected token not found, EOF reached"
                .to_string(),
        }
    }

    #[allow(unused)]
    fn error_unexpected_token_found(&mut self, unexpected_token: String) -> ParserError {
        ParserError::TokenExpected {
            line_pos: self.line_count,
            char_pos: self.char_pos_line,
            message: format!("Expected token not found, instead found token {}", unexpected_token)
                .to_string(),
        }
    }

    
    
    async fn read_delimited_string(
        &mut self,
        placeholder: &Placeholder,
        constituents: &Vec<Placeholder>,
        i: &mut usize,
    ) -> Result<ValueType, ParserError> {
        let delimiter = self.get_delimiter(constituents, i).await?;
        Ok(self
            .read_placeholder_until(placeholder, delimiter.to_owned())
            .await?)
    }

    async fn get_delimiter(
        &mut self,
        constituents: &Vec<Placeholder>,
        i: &mut usize,
    ) -> Result<String, ParserError> {
        if constituents.len() > *i + 1 {
            let delimiter = match &constituents[*i + 1].place_holder_type {
                PlaceHolderType::Delimiter(delimiter) => delimiter,
                PlaceHolderType::NewLine => "\n",

                PlaceHolderType::Space => " ",
                PlaceHolderType::ExactString(input) => input,
                
                _ => {
                    return Err(ParserError::InvalidPlaceHolderTypeFound {
                        line_pos: self.line_count,
                        char_pos: self.char_pos_line,
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
                line_pos: self.line_count,
                char_pos: self.char_pos_line,
                message: "Expected one of the delimiter type or known string, but reached end of child placeholders".to_string(),
            });
        }
    }
}

fn update_key_value<RI>(
    request_info: &mut RI,
    key: &mut Option<String>,    
    constituent: &Placeholder,
    result: ValueType,
    line_pos: usize,
    char_pos: usize,
) -> Result<(), ParserError>
where
    RI: RequestInfo,
{
    let result = match &result {
        ValueType::String(data) => match &constituent.name {
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
                line_pos: line_pos,
                char_pos: char_pos,
                message: "Expected String token not found".to_string(),
            })?;
        }
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tokio::io::BufReader;
    use tokio_stream::StreamExt;

    use crate::core::protocol_reader::ProtoStream;
    use crate::core::PlaceHolderIdentifier::{InlineKeyWithValue, Name};
    use crate::core::{
        PlaceHolderType, Placeholder, ProtocolSpecBuilder, RequestInfo, SpecBuilder, ValueType,
    };

    use super::{PlaceHolderRead, ProtocolBuffReader};

    fn assert_result_has_string(
        result: Result<crate::core::ValueType, crate::core::ParserError>,
        data: String,
    ) {
        match result.unwrap() {
            crate::core::ValueType::String(value) => {
                assert!(value == data);
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_read_string_until() {
        let data = b"Hello World::";
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);
        let result = protocol_reader
            .read_placeholder_until(
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: crate::core::PlaceHolderIdentifier::Name("name".to_string()),
                    constituents: None,
                },
                "::".to_string(),
            )
            .await;

        assert_result_has_string(result, "Hello World".to_string());
    }

    #[tokio::test]
    async fn test_read_string_until_delimiter_missing() {
        let data = b"Hello World";
        let mut protocol_reader = ProtocolBuffReader::new(BufReader::new(&data[..]), 1024);
        let result = protocol_reader
            .read_placeholder_until(
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: crate::core::PlaceHolderIdentifier::Name("name".to_string()),
                    constituents: None,
                },
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
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: crate::core::PlaceHolderIdentifier::Name("name".to_string()),
                    constituents: None,
                },
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
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: Name("name".to_string()),
                    constituents: None,
                },
                " ".to_string(),
            )
            .await;
        assert_result_has_string(result, "Hello".to_string());

        let result = protocol_reader
            .read_placeholder_until(
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: Name("name".to_string()),
                    constituents: None,
                },
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
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: Name("name".to_string()),
                    constituents: None,
                },
                "Hello".to_string(),
            )
            .await;
        assert_result_has_string(result, "Hello".to_string());

        let result = protocol_reader
            .read_placeholder_as_string(
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: Name("name".to_string()),
                    constituents: None,
                },
                " ".to_string(),
            )
            .await;
        assert_result_has_string(result, " ".to_string());

        let result = protocol_reader
            .read_placeholder_as_string(
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: Name("name".to_string()),
                    constituents: None,
                },
                "World".to_string(),
            )
            .await;
        assert_result_has_string(result, "World".to_string());

        let result = protocol_reader
            .read_placeholder_as_string(
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: crate::core::PlaceHolderIdentifier::InlineKeyWithValue(
                        "name".to_string(),
                    ),
                    constituents: None,
                },
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
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: Name("name".to_string()),
                    constituents: None,
                },
                "Hello".to_string(),
            )
            .await
            .unwrap();
        protocol_reader.reset();

        let value = protocol_reader
            .read_placeholder_as_string(
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: Name("name".to_string()),
                    constituents: None,
                },
                "Hello".to_string(),
            )
            .await
            .unwrap();
        protocol_reader.reset();

        match value {
            crate::core::ValueType::String(value) => {
                assert!(value == "Hello".to_string());
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
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: Name("name".to_string()),
                    constituents: None,
                },
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
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: Name("name".to_string()),
                    constituents: None,
                },
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
                &Placeholder {
                    place_holder_type: PlaceHolderType::AnyString,
                    name: Name("name".to_string()),
                    constituents: None,
                },
                "ld\n".to_string(),
            )
            .await
            .unwrap();
        match data {
            crate::core::ValueType::String(value) => {
                assert!(value == "ld\n".to_string());
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn parse_composite_test() {
        let root_placeholder =
            Placeholder::new(Name("root".to_string()), None, PlaceHolderType::Composite);

        let mut spec_builder = SpecBuilder(root_placeholder);

        let request_line_placeholder = SpecBuilder::new_composite("request_line".to_string())
            .expect_one_of_string(
                vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "DELETE".to_string(),
                    "PUT".to_string(),
                    "OPTIONS".to_string(),
                ],
                InlineKeyWithValue("request_method".to_string()),
            )
            .expect_space()
            .expect_string(InlineKeyWithValue("request_uri".to_string()))
            .expect_space()
            .expect_string(InlineKeyWithValue("protocol_version".to_string()))
            .expect_newline()
            .build();

        let mut header_placeholder_builder = SpecBuilder::new_composite("header".to_string());
        let header_place_holder = header_placeholder_builder
            .expect_string(crate::core::PlaceHolderIdentifier::Key)
            .expect_delimiter(": ".to_string())
            .expect_string(crate::core::PlaceHolderIdentifier::Value)
            .expect_newline()
            .build();

        spec_builder.expect_composite(request_line_placeholder, "first_line".to_owned());
        spec_builder.expect_newline();
        spec_builder.expect_repeat_many(header_place_holder, "headers".to_owned());
        spec_builder.expect_exact_string(InlineKeyWithValue("data".to_string()));
        spec_builder.expect_newline();

        let placehoder = spec_builder.build();
        let mut protocol_reader = ProtocolBuffReader::new(
            BufReader::new(
                b"GET /index.html HTTP/1.1\n\nname: value\nname2: value2\ntest123\n".as_ref(),
            ),
            1024,
        );
        let mut request_info = TestRequestInfo::default();
        let result = protocol_reader
            .parse_composite(&placehoder, &mut request_info)
            .await;
        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let request_method = request_info.get_info("request_method".to_string()).unwrap();
        match request_method {
            crate::core::ValueType::String(value) => {
                assert!(*value == "GET".to_string());
            }
            _ => {
                assert!(false);
            }
        }

        match request_info.get_info("name".to_owned()).unwrap() {
            crate::core::ValueType::String(value) => {
                assert!(*value == "value".to_string());
            }
            _ => {
                assert!(false);
            }
        }

        match request_info.get_info("name2".to_owned()).unwrap() {
            crate::core::ValueType::String(value) => {
                assert!(*value == "value2".to_string());
            }
            _ => {
                assert!(false);
            }
        }

        match request_info.get_info("data".to_owned()).unwrap() {
            crate::core::ValueType::String(value) => {
                assert!(*value == "test123".to_string());
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[derive(Default)]
    struct TestRequestInfo(HashMap<String, ValueType>);

    impl RequestInfo for TestRequestInfo {
        fn add_info(&mut self, key: String, value: ValueType) {
            self.0.insert(key, value);
        }

        fn get_info(&self, key: String) -> Option<&crate::core::ValueType> {
            self.0.get(&key)
        }
    }
}
