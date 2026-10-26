//! DNS packet parser for Luffy Router.
//! Parses minimal header and question records without external dependencies.

use luffy_core::CoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

pub struct DnsPacketParser;

impl DnsPacketParser {
    pub fn parse_header(buf: &[u8]) -> Result<DnsHeader, CoreError> {
        if buf.len() < 12 {
            return Err(CoreError::Validation("DNS packet too short for header".into()));
        }

        let id = u16::from_be_bytes([buf[0], buf[1]]);
        let flags = u16::from_be_bytes([buf[2], buf[3]]);
        let qdcount = u16::from_be_bytes([buf[4], buf[5]]);
        let ancount = u16::from_be_bytes([buf[6], buf[7]]);
        let nscount = u16::from_be_bytes([buf[8], buf[9]]);
        let arcount = u16::from_be_bytes([buf[10], buf[11]]);

        Ok(DnsHeader {
            id,
            flags,
            qdcount,
            ancount,
            nscount,
            arcount,
        })
    }

    pub fn parse_first_question(buf: &[u8]) -> Result<(DnsHeader, DnsQuestion, usize), CoreError> {
        let header = Self::parse_header(buf)?;
        if header.qdcount == 0 {
            return Err(CoreError::Validation("No questions in DNS packet".into()));
        }

        let mut offset = 12;
        let mut labels = Vec::new();

        loop {
            if offset >= buf.len() {
                return Err(CoreError::Validation("Truncated QNAME".into()));
            }

            let len = buf[offset] as usize;
            offset += 1;

            if len == 0 {
                break;
            }

            // Compression pointers (top 2 bits 11) not expected in primary request question
            if len & 0xC0 == 0xC0 {
                return Err(CoreError::Validation("Compression pointer in primary question".into()));
            }

            if offset + len > buf.len() {
                return Err(CoreError::Validation("Label extends beyond packet buffer".into()));
            }

            let label = std::str::from_utf8(&buf[offset..offset + len])
                .map_err(|_| CoreError::Validation("Invalid UTF-8 in QNAME label".into()))?;
            labels.push(label.to_string());
            offset += len;
        }

        if offset + 4 > buf.len() {
            return Err(CoreError::Validation("Truncated QTYPE / QCLASS".into()));
        }

        let qtype = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
        let qclass = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]);
        offset += 4;

        let qname = labels.join(".");

        Ok((
            header,
            DnsQuestion {
                qname,
                qtype,
                qclass,
            },
            offset,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dns_query() {
        // Standard DNS query for "test.example.com" A IN, ID 0x1234
        let raw = vec![
            0x12, 0x34, // ID
            0x01, 0x00, // Standard query, RD=1
            0x00, 0x01, // QDCOUNT = 1
            0x00, 0x00, // ANCOUNT = 0
            0x00, 0x00, // NSCOUNT = 0
            0x00, 0x00, // ARCOUNT = 0
            // QNAME: 4test 7example 3com 0
            4, b't', b'e', b's', b't',
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
            3, b'c', b'o', b'm',
            0,
            0x00, 0x01, // QTYPE = A (1)
            0x00, 0x01, // QCLASS = IN (1)
        ];

        let (header, question, consumed) = DnsPacketParser::parse_first_question(&raw).unwrap();
        assert_eq!(header.id, 0x1234);
        assert_eq!(question.qname, "test.example.com");
        assert_eq!(question.qtype, 1);
        assert_eq!(question.qclass, 1);
        assert_eq!(consumed, raw.len());
    }
}
