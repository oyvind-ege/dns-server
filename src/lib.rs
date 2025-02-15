pub mod flag;
use crate::flag::Flag;

#[derive(Debug, PartialEq)]
pub struct DNSMessage {
    pub header: DNSMessageHeader,
}

#[derive(Debug, PartialEq)]
pub enum OpCode {
    QUERY = 0,
    IQUERY = 1,
    STATUS = 2,
}

#[derive(Debug, PartialEq)]
pub enum RCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

#[derive(Debug, PartialEq)]
pub enum Z {
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
#[derive(Debug, PartialEq)]
pub struct DNSMessageHeader {
    pub id: u16,
    pub flags: Flag,
    pub question_count: u16,
    pub answer_count: u16,
    pub name_server_resource_count: u16,
    pub resource_records_count: u16,
}

impl DNSMessageHeader {
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut bytes: [u8; 12] = [0; 12];
        let id_bytes = self.id.to_be_bytes();
        bytes[0] = id_bytes[0];
        bytes[1] = id_bytes[1];

        let flag_bytes = self.flags.get_flag().to_be_bytes();

        bytes[2] = flag_bytes[0];
        bytes[3] = flag_bytes[1];

        let qd_count_bytes = self.question_count.to_be_bytes();
        bytes[4] = qd_count_bytes[0];
        bytes[5] = qd_count_bytes[1];

        let an_count_bytes = self.answer_count.to_be_bytes();
        bytes[6] = an_count_bytes[0];
        bytes[7] = an_count_bytes[1];

        let ns_count_bytes = self.name_server_resource_count.to_be_bytes();
        bytes[8] = ns_count_bytes[0];
        bytes[9] = ns_count_bytes[1];

        let ar_count_bytes = self.resource_records_count.to_be_bytes();
        bytes[10] = ar_count_bytes[0];
        bytes[11] = ar_count_bytes[1];

        bytes
    }
}
