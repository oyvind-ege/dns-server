use deku::bitvec::{BitRef, BitView, Lsb0, Msb0};
use deku::ctx::Endian;
use deku::prelude::*;

#[derive(Debug, PartialEq, DekuWrite, DekuRead)]
#[deku(id_type = "u8", bits = 4, endian = "endian", ctx = "endian: Endian")]
pub enum OpCode {
    #[deku(id = "0")]
    QUERY = 0,
    #[deku(id = "1")]
    IQUERY = 1,
    #[deku(id = "2")]
    STATUS = 2,
}

#[derive(Debug, PartialEq, DekuWrite, DekuRead)]
#[deku(id_type = "u8", bits = 4, endian = "endian", ctx = "endian: Endian")]
pub enum RCode {
    #[deku(id = "0")]
    NoError = 0,
    #[deku(id = "1")]
    FormatError = 1,
    #[deku(id = "2")]
    ServerFailure = 2,
    #[deku(id = "3")]
    NameError = 3,
    #[deku(id = "4")]
    NotImplemented = 4,
    #[deku(id = "5")]
    Refused = 5,
}

#[derive(Debug, PartialEq, DekuWrite, DekuRead)]
#[deku(id_type = "u8", bits = 3, endian = "endian", ctx = "endian: Endian")]
pub enum Z {
    #[deku(id = "0")]
    Default,
}

//                                     1  1  1  1  1  1
//       0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
//     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//     |                      ID                       |
//     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//     |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
//     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//     |                    QDCOUNT                    |
//     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//     |                    ANCOUNT                    |
//     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//     |                    NSCOUNT                    |
//     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//     |                    ARCOUNT                    |
//     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct DNSMessageHeader {
    pub id: u16,
    pub qr: bool,
    pub opcode: OpCode,
    pub authoritative_answer: bool,
    pub is_truncated: bool,
    pub is_recursion_desired: bool,
    pub is_recursion_available: bool,
    pub z: Z,
    pub response_code: RCode,
    pub question_count: u16,
    pub answer_count: u16,
    pub name_server_resource_count: u16,
    pub resource_records_count: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct DNSMessage {
    pub header: DNSMessageHeader,
}

impl DNSMessage {
    pub fn print(&self) {
        println!(
            "Bytes as binary: {:#b}",
            &self.to_bytes().unwrap().view_bits::<Msb0>()
        )
    }
}
