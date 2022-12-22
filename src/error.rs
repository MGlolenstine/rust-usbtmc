#[derive(Debug)]
pub enum UsbtmcError {
    #[cfg(feature = "rusb")]
    Rusb(rusb::Error),
    #[cfg(feature = "libusbk")]
    Libusbk(libusbk::Error),
    BulkIn,
    BulkOut,
    Exception,
}

#[cfg(feature = "rusb")]
impl From<rusb::Error> for UsbtmcError {
    fn from(item: rusb::Error) -> Self {
        UsbtmcError::Rusb(item)
    }
}

#[cfg(feature = "libusbk")]
impl From<libusbk::Error> for UsbtmcError {
    fn from(item: libusbk::Error) -> Self {
        UsbtmcError::Libusbk(item)
    }
}