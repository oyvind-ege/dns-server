use codecrafters_dns_server::*;
use deku::DekuContainerWrite;
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
                        qr: true,
                        opcode: OpCode::QUERY,
                        authoritative_answer: false,
                        is_truncated: false,
                        is_recursion_desired: false,
                        is_recursion_available: false,
                        z: Z::Default,
                        response_code: RCode::NoError,
                        question_count: 0,
                        answer_count: 0,
                        name_server_resource_count: 0,
                        resource_records_count: 0,
                    },
                };

                response.print();
                println!("Bytes: {:0x?}", &DNSMessage::to_bytes(&response).unwrap());
                udp_socket
                    .send_to(&DNSMessage::to_bytes(&response).unwrap(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
