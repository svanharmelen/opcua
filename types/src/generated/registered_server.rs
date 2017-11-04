// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use string::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use attribute::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use generated::node_ids::*;
#[allow(unused_imports)]
use generated::status_codes::StatusCode;
#[allow(unused_imports)]
use generated::status_codes::StatusCode::*;

/// The information required to register a server with a discovery server.
#[derive(Debug, Clone, PartialEq)]
pub struct RegisteredServer {
    pub server_uri: UAString,
    pub product_uri: UAString,
    pub server_names: Option<Vec<LocalizedText>>,
    pub server_type: ApplicationType,
    pub gateway_server_uri: UAString,
    pub discovery_urls: Option<Vec<UAString>>,
    pub semaphore_file_path: UAString,
    pub is_online: Boolean,
}

impl MessageInfo for RegisteredServer {
    fn object_id(&self) -> ObjectId {
        ObjectId::RegisteredServer_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RegisteredServer> for RegisteredServer {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.server_uri.byte_len();
        size += self.product_uri.byte_len();
        size += byte_len_array(&self.server_names);
        size += self.server_type.byte_len();
        size += self.gateway_server_uri.byte_len();
        size += byte_len_array(&self.discovery_urls);
        size += self.semaphore_file_path.byte_len();
        size += self.is_online.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.server_uri.encode(stream)?;
        size += self.product_uri.encode(stream)?;
        size += write_array(stream, &self.server_names)?;
        size += self.server_type.encode(stream)?;
        size += self.gateway_server_uri.encode(stream)?;
        size += write_array(stream, &self.discovery_urls)?;
        size += self.semaphore_file_path.encode(stream)?;
        size += self.is_online.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let server_uri = UAString::decode(stream)?;
        let product_uri = UAString::decode(stream)?;
        let server_names: Option<Vec<LocalizedText>> = read_array(stream)?;
        let server_type = ApplicationType::decode(stream)?;
        let gateway_server_uri = UAString::decode(stream)?;
        let discovery_urls: Option<Vec<UAString>> = read_array(stream)?;
        let semaphore_file_path = UAString::decode(stream)?;
        let is_online = Boolean::decode(stream)?;
        Ok(RegisteredServer {
            server_uri,
            product_uri,
            server_names,
            server_type,
            gateway_server_uri,
            discovery_urls,
            semaphore_file_path,
            is_online,
        })
    }
}
