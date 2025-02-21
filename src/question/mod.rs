#[derive(Debug, PartialEq, Default)]
#[repr(u16)]
pub enum QTYPE {
    #[default]
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct QName {
    labels: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct DNSQuestion {
    qname: QName,
    qtype: QTYPE,
    qclass: u16,
}

impl DNSQuestion {
    pub fn set_qname(&mut self, qn: String) -> &mut Self {
        self.qname.labels = qn;
        self
    }

    pub fn set_qtype(&mut self, qt: QTYPE) -> &mut Self {
        self.qtype = qt;
        self
    }

    pub fn set_qclass(&mut self, qc: u16) -> &mut Self {
        self.qclass = qc;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {}
}
