
pub mod kyc_status {
    #[derive(Clone, Copy, PartialEq)]
    pub enum KycStatus {
        NotVerified = 0,
        Verified = 1,
    }
}