use types::*;

#[derive(Debug, Clone)]
pub struct SecureChannelInfo {
    pub security_policy: SecurityPolicy,
    pub secure_channel_id: UInt32,
    pub token_id: UInt32,
}

impl SecureChannelInfo {
    pub fn new() -> SecureChannelInfo {
        SecureChannelInfo {
            security_policy: SecurityPolicy::None,
            secure_channel_id: 0,
            token_id: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityPolicy {
    Unknown,
    None,
    Basic128Rsa15,
    Basic256,
    Basic256Sha256,
}

const SECURITY_POLICY_NONE: &'static str = "http://opcfoundation.org/UA/SecurityPolicy#None";
const SECURITY_POLICY_BASIC128RSA15: &'static str = "http://opcfoundation.org/UA/SecurityPolicy#Basic128Rsa15";
const SECURITY_POLICY_BASIC256: &'static str = "http://opcfoundation.org/UA/SecurityPolicy#Basic256";
const SECURITY_POLICY_BASIC256SHA256: &'static str = "http://opcfoundation.org/UA/SecurityPolicy#Basic256Sha256";

impl SecurityPolicy {
    pub fn to_string(&self) -> UAString {
        UAString::from_str(self.to_uri())
    }

    pub fn to_uri(&self) -> &'static str {
        match *self {
            SecurityPolicy::None => SECURITY_POLICY_NONE,
            SecurityPolicy::Basic128Rsa15 => SECURITY_POLICY_BASIC128RSA15,
            SecurityPolicy::Basic256 => SECURITY_POLICY_BASIC256,
            SecurityPolicy::Basic256Sha256 => SECURITY_POLICY_BASIC256SHA256,
            _ => {
                panic!("Shouldn't be turning an unknown policy into a uri");
            }
        }
    }

    pub fn from_uri(uri: &str) -> SecurityPolicy {
        match uri {
            SECURITY_POLICY_NONE => {
                SecurityPolicy::None
            },
            SECURITY_POLICY_BASIC128RSA15 => {
                SecurityPolicy::Basic128Rsa15
            },
            SECURITY_POLICY_BASIC256 => {
                SecurityPolicy::Basic256
            },
            SECURITY_POLICY_BASIC256SHA256 => {
                SecurityPolicy::Basic256Sha256
            }
            _ => {
                error!("Specified security policy {} is not recognized", uri);
                SecurityPolicy::Unknown
            }
        }
    }
}
