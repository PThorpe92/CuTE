// This is the display auth not to be confused with the request auth

#[derive(Debug, Clone, PartialEq)]
pub enum AuthType {
    // OAuth looks impossible to implement
    Basic,
    Bearer,
    Digest,
    Hawk,
    AWSSignature,
    NTLM,
    Kerberos,
    SPNEGO,
}
