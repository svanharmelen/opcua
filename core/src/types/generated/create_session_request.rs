// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write, Result};

use types::*;
use services::*;

/// Creates a new session with the server.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSessionRequest {
    pub request_header: RequestHeader,
    pub client_description: ApplicationDescription,
    pub server_uri: UAString,
    pub endpoint_url: UAString,
    pub session_name: UAString,
    pub client_nonce: ByteString,
    pub client_certificate: ByteString,
    pub requested_session_timeout: Double,
    pub max_response_message_size: UInt32,
}

impl MessageInfo for CreateSessionRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::CreateSessionRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CreateSessionRequest> for CreateSessionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.client_description.byte_len();
        size += self.server_uri.byte_len();
        size += self.endpoint_url.byte_len();
        size += self.session_name.byte_len();
        size += self.client_nonce.byte_len();
        size += self.client_certificate.byte_len();
        size += self.requested_session_timeout.byte_len();
        size += self.max_response_message_size.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> Result<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.client_description.encode(stream)?;
        size += self.server_uri.encode(stream)?;
        size += self.endpoint_url.encode(stream)?;
        size += self.session_name.encode(stream)?;
        size += self.client_nonce.encode(stream)?;
        size += self.client_certificate.encode(stream)?;
        size += self.requested_session_timeout.encode(stream)?;
        size += self.max_response_message_size.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> Result<CreateSessionRequest> {
        let request_header = RequestHeader::decode(stream)?;
        let client_description = ApplicationDescription::decode(stream)?;
        let server_uri = UAString::decode(stream)?;
        let endpoint_url = UAString::decode(stream)?;
        let session_name = UAString::decode(stream)?;
        let client_nonce = ByteString::decode(stream)?;
        let client_certificate = ByteString::decode(stream)?;
        let requested_session_timeout = Double::decode(stream)?;
        let max_response_message_size = UInt32::decode(stream)?;
        Ok(CreateSessionRequest {
            request_header: request_header,
            client_description: client_description,
            server_uri: server_uri,
            endpoint_url: endpoint_url,
            session_name: session_name,
            client_nonce: client_nonce,
            client_certificate: client_certificate,
            requested_session_timeout: requested_session_timeout,
            max_response_message_size: max_response_message_size,
        })
    }
}
