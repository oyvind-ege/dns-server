use crate::flag::Flag;
use crate::header::DNSMessageHeader;
use codecrafters_dns_server::*;
use question::DNSQuestion;
#[allow(unused_imports)]
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512]; // The theoretical maximum packet size is 65,515 bytes. This only reserves a buffer for 512, which is the expected size of a DNS Message.

    println!("UDP set up.");
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = DNSMessage {
                    header: DNSMessageHeader {
                        id: 1234,
                        flags: Flag::default().set_qr(true).collect(),
                        question_count: 0,
                        answer_count: 0,
                        name_server_resource_count: 0,
                        resource_records_count: 0,
                    },
                    question: DNSQuestion::default(),
                };

                udp_socket
                    .send_to(&response.header.to_bytes(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
