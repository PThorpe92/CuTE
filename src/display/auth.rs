// This is the display auth not to be confused with the request auth

#[derive(Debug, Clone, PartialEq)]
pub enum AuthType {
    Basic,
    Bearer,
    Digest,
    Hawk,
    OAuth,
    AWSSignature,
    NTLM,
    Kerberos,
    SPNEGO,
    Custom,
}
