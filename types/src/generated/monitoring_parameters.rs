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
pub struct MonitoringParameters {
    pub client_handle: UInt32,
    pub sampling_interval: Double,
    pub filter: ExtensionObject,
    pub queue_size: UInt32,
    pub discard_oldest: Boolean,
}

impl MessageInfo for MonitoringParameters {
    fn object_id(&self) -> ObjectId {
        ObjectId::MonitoringParameters_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<MonitoringParameters> for MonitoringParameters {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.client_handle.byte_len();
        size += self.sampling_interval.byte_len();
        size += self.filter.byte_len();
        size += self.queue_size.byte_len();
        size += self.discard_oldest.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.client_handle.encode(stream)?;
        size += self.sampling_interval.encode(stream)?;
        size += self.filter.encode(stream)?;
        size += self.queue_size.encode(stream)?;
        size += self.discard_oldest.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let client_handle = UInt32::decode(stream)?;
        let sampling_interval = Double::decode(stream)?;
        let filter = ExtensionObject::decode(stream)?;
        let queue_size = UInt32::decode(stream)?;
        let discard_oldest = Boolean::decode(stream)?;
        Ok(MonitoringParameters {
            client_handle,
            sampling_interval,
            filter,
            queue_size,
            discard_oldest,
        })
    }
}
