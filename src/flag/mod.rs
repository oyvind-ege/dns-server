use crate::OpCode;
use crate::RCode;

#[derive(Debug, PartialEq, Default)]
pub struct Flag {
    flag: u16,
}

impl Flag {
    pub fn new(fl: u16) -> Self {
        Self { flag: fl }
    }

    pub fn get_flag(&self) -> u16 {
        self.flag
    }
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_set_qr_to_true() {
        let expected: u16 = 0b1000000000000000;
        let mut flag_data = Flag::new(0b0000000000000000);
        flag_data.set_qr(true);
        dbg!(assert_eq!(expected, flag_data.get_flag()));
    }

    #[test]
    fn test_set_qr_false() {
        let expected: u16 = 0b0000000000000000;
        let mut flag_data = Flag::new(0b1000000000000000);
        flag_data.set_qr(false);
        dbg!(assert_eq!(expected, flag_data.get_flag()));
    }

    #[test]
    fn test_unaffected_fields_or() {
        let expected: u16 = 0b1100000000000000;
        let mut flag_data = Flag::new(0b0100000000000000);
        flag_data.set_qr(true);
        dbg!(assert_eq!(expected, flag_data.get_flag()));
    }

    #[test]
    fn test_unaffected_fields_and() {
        let expected: u16 = 0b0100000000000000;
        let mut flag_data = Flag::new(0b1100000000000000);
        flag_data.set_qr(false);
        dbg!(assert_eq!(expected, flag_data.get_flag()));
    }

    #[test]
    fn test_combined() {
        let expected: u16 = 0b1000100000000001;

        let mut flag_data = Flag::new(0b0000000000000000);
        flag_data.set_qr(true);
        flag_data.set_opcode(OpCode::IQUERY);
        flag_data.set_r_code(RCode::FormatError);

        dbg!(assert_eq!(expected, flag_data.get_flag()));
    }
}
