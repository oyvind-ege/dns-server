use arbitrary_int::u96;
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

#[derive(Debug, PartialEq, Default)]
pub struct Flag {
    flag: u16,
}

impl Flag {
    pub fn set_qr(&mut self, state: bool) -> &mut Self {
        match state {
            true => self.flag |= 0b1000000000000000,
            false => self.flag &= 0b0111111111111111,
        };

        self
    }

    pub fn set_opcode(&mut self, op: OpCode) -> &mut Self {
        match op {
            OpCode::QUERY => self.flag |= 0b0000000000000000,
            OpCode::IQUERY => self.flag |= 0b0000100000000000,
            OpCode::STATUS => self.flag |= 0b0001000000000000,
        }
        self
    }

    pub fn set_aa(&mut self, state: bool) -> &mut Self {
        match state {
            true => self.flag |= 0b0000010000000000,
            false => self.flag &= 0b1111101111111111,
        };
        self
    }

    pub fn set_tc(&mut self, state: bool) -> &mut Self {
        match state {
            true => self.flag |= 0b0000001000000000,
            false => self.flag &= 0b1111110111111111,
        };
        self
    }

    pub fn set_rd(&mut self, state: bool) -> &mut Self {
        match state {
            true => self.flag |= 0b0000000100000000,
            false => self.flag &= 0b1111111011111111,
        };
        self
    }

    pub fn set_ra(&mut self, state: bool) -> &mut Self {
        match state {
            true => self.flag |= 0b0000000010000000,
            false => self.flag &= 0b1111111101111111,
        };
        self
    }

    pub fn set_z(&mut self, state: bool) -> &mut Self {
        match state {
            true => self.flag |= 0b0000000001110000,
            false => self.flag &= 0b1111111110001111,
        };
        self
    }

    pub fn set_r_code(&mut self, rc: RCode) -> &mut Self {
        match rc {
            RCode::NoError => self.flag |= 0b0000000000000000,
            RCode::FormatError => self.flag |= 0b0000000000000001,
            RCode::ServerFailure => self.flag |= 0b0000000000000010,
            RCode::NameError => self.flag |= 0b0000000000000011,
            RCode::NotImplemented => self.flag |= 0b0000000000000100,
            RCode::Refused => self.flag |= 0b0000000000000101,
        }
        self
    }

    pub fn collect(&mut self) -> Self {
        Self { flag: self.flag }
    }
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

#[derive(Debug, PartialEq)]
pub struct DNSMessage {
    pub header: DNSMessageHeader,
}

impl DNSMessageHeader {
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut bytes: [u8; 12] = [0; 12];
        let id_bytes = self.id.to_be_bytes();
        bytes[0] = id_bytes[0];
        bytes[1] = id_bytes[1];

        let flag_bytes = self.flags.flag.to_be_bytes();

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_qr_to_true() {
        let expected: u16 = 0b1000000000000000;
        let mut flag_data = Flag {
            flag: 0b0000000000000000,
        };
        flag_data.set_qr(true);
        dbg!(assert_eq!(expected, flag_data.flag));
    }

    #[test]
    fn test_set_qr_false() {
        let expected: u16 = 0b0000000000000000;
        let mut flag_data = Flag {
            flag: 0b1000000000000000,
        };
        flag_data.set_qr(false);
        dbg!(assert_eq!(expected, flag_data.flag));
    }

    #[test]
    fn test_unaffected_fields_or() {
        let expected: u16 = 0b1100000000000000;

        let mut flag_data = Flag {
            flag: 0b0100000000000000,
        };
        flag_data.set_qr(true);
        dbg!(assert_eq!(expected, flag_data.flag));
    }

    #[test]
    fn test_unaffected_fields_and() {
        let expected: u16 = 0b0100000000000000;

        let mut flag_data = Flag {
            flag: 0b1100000000000000,
        };
        flag_data.set_qr(false);
        dbg!(assert_eq!(expected, flag_data.flag));
    }

    #[test]
    fn test_combined() {
        let expected: u16 = 0b1000100000000001;

        let mut flag_data = Flag {
            flag: 0b0000000000000000,
        };
        flag_data.set_qr(true);
        flag_data.set_opcode(OpCode::IQUERY);
        flag_data.set_r_code(RCode::FormatError);

        dbg!(assert_eq!(expected, flag_data.flag));
    }
}
