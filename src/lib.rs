pub mod flag;
pub mod header;
pub mod question;
use crate::header::DNSMessageHeader;
use crate::question::DNSQuestion;

#[derive(Debug, PartialEq, Default)]
pub struct DNSMessage {
    pub header: DNSMessageHeader,
    pub question: DNSQuestion,
}
