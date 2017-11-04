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

#[derive(Debug, Clone, PartialEq)]
pub struct ServerDiagnosticsSummaryDataType {
    pub server_view_count: UInt32,
    pub current_session_count: UInt32,
    pub cumulated_session_count: UInt32,
    pub security_rejected_session_count: UInt32,
    pub rejected_session_count: UInt32,
    pub session_timeout_count: UInt32,
    pub session_abort_count: UInt32,
    pub current_subscription_count: UInt32,
    pub cumulated_subscription_count: UInt32,
    pub publishing_interval_count: UInt32,
    pub security_rejected_requests_count: UInt32,
    pub rejected_requests_count: UInt32,
}

impl MessageInfo for ServerDiagnosticsSummaryDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::ServerDiagnosticsSummaryDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ServerDiagnosticsSummaryDataType> for ServerDiagnosticsSummaryDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.server_view_count.byte_len();
        size += self.current_session_count.byte_len();
        size += self.cumulated_session_count.byte_len();
        size += self.security_rejected_session_count.byte_len();
        size += self.rejected_session_count.byte_len();
        size += self.session_timeout_count.byte_len();
        size += self.session_abort_count.byte_len();
        size += self.current_subscription_count.byte_len();
        size += self.cumulated_subscription_count.byte_len();
        size += self.publishing_interval_count.byte_len();
        size += self.security_rejected_requests_count.byte_len();
        size += self.rejected_requests_count.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.server_view_count.encode(stream)?;
        size += self.current_session_count.encode(stream)?;
        size += self.cumulated_session_count.encode(stream)?;
        size += self.security_rejected_session_count.encode(stream)?;
        size += self.rejected_session_count.encode(stream)?;
        size += self.session_timeout_count.encode(stream)?;
        size += self.session_abort_count.encode(stream)?;
        size += self.current_subscription_count.encode(stream)?;
        size += self.cumulated_subscription_count.encode(stream)?;
        size += self.publishing_interval_count.encode(stream)?;
        size += self.security_rejected_requests_count.encode(stream)?;
        size += self.rejected_requests_count.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let server_view_count = UInt32::decode(stream)?;
        let current_session_count = UInt32::decode(stream)?;
        let cumulated_session_count = UInt32::decode(stream)?;
        let security_rejected_session_count = UInt32::decode(stream)?;
        let rejected_session_count = UInt32::decode(stream)?;
        let session_timeout_count = UInt32::decode(stream)?;
        let session_abort_count = UInt32::decode(stream)?;
        let current_subscription_count = UInt32::decode(stream)?;
        let cumulated_subscription_count = UInt32::decode(stream)?;
        let publishing_interval_count = UInt32::decode(stream)?;
        let security_rejected_requests_count = UInt32::decode(stream)?;
        let rejected_requests_count = UInt32::decode(stream)?;
        Ok(ServerDiagnosticsSummaryDataType {
            server_view_count,
            current_session_count,
            cumulated_session_count,
            security_rejected_session_count,
            rejected_session_count,
            session_timeout_count,
            session_abort_count,
            current_subscription_count,
            cumulated_subscription_count,
            publishing_interval_count,
            security_rejected_requests_count,
            rejected_requests_count,
        })
    }
}
