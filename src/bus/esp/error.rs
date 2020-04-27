
/// Error types mapped to ESP-IDF failure values.
#[repr(u32)]
#[derive(Debug, failure::Fail)]
pub enum EspError {
    #[fail(display = "NO_MEM")]
    NoMem = esp_idf_sys::ESP_ERR_NO_MEM,
    #[fail(display = "INVALID_ARG")]
    InvalidArg = esp_idf_sys::ESP_ERR_INVALID_ARG,
    #[fail(display = "INVALID_STATE")]
    InvalidState = esp_idf_sys::ESP_ERR_INVALID_STATE,
    #[fail(display = "INVALID_SIZE")]
    InvalidSize = esp_idf_sys::ESP_ERR_INVALID_SIZE,
    #[fail(display = "NOT_FOUND")]
    NotFound = esp_idf_sys::ESP_ERR_NOT_FOUND,
    #[fail(display = "NOT_SUPPORTED")]
    NotSupported = esp_idf_sys::ESP_ERR_NOT_SUPPORTED,
    #[fail(display = "TIMEOUT")]
    Timeout = esp_idf_sys::ESP_ERR_TIMEOUT,
    #[fail(display = "INVALID_RESPONSE")]
    InvalidResponse = esp_idf_sys::ESP_ERR_INVALID_RESPONSE,
    #[fail(display = "INVALID_CRC")]
    InvalidCrc = esp_idf_sys::ESP_ERR_INVALID_CRC,
    #[fail(display = "INVALID_VERSION")]
    InvalidVersion = esp_idf_sys::ESP_ERR_INVALID_VERSION,
    #[fail(display = "INVALID_MAC")]
    InvalidMac = esp_idf_sys::ESP_ERR_INVALID_MAC,
    #[fail(display = "WIFI_BASE")]
    WifiBase = esp_idf_sys::ESP_ERR_WIFI_BASE,
    #[fail(display = "MESH_BASE")]
    MeshBase = esp_idf_sys::ESP_ERR_MESH_BASE,
}

/// Failed to convert an integer into an `enum` value
pub struct EnumFromIntError(u32);

/// Attempts to convert int returned from ESP-IDF into error `enum`
impl core::convert::TryFrom<i32> for EspError {
    type Error = EnumFromIntError;
    fn try_from(value: i32) -> core::result::Result<Self, Self::Error> {
        use EspError::*;
        match value as u32 {
            esp_idf_sys::ESP_ERR_NO_MEM => Ok(NoMem),
            esp_idf_sys::ESP_ERR_INVALID_ARG => Ok(InvalidArg),
            esp_idf_sys::ESP_ERR_INVALID_STATE => Ok(InvalidState),
            esp_idf_sys::ESP_ERR_INVALID_SIZE => Ok(InvalidSize),
            esp_idf_sys::ESP_ERR_NOT_FOUND => Ok(NotFound),
            esp_idf_sys::ESP_ERR_NOT_SUPPORTED => Ok(NotSupported),
            esp_idf_sys::ESP_ERR_TIMEOUT => Ok(Timeout),
            esp_idf_sys::ESP_ERR_INVALID_RESPONSE => Ok(InvalidResponse),
            esp_idf_sys::ESP_ERR_INVALID_CRC => Ok(InvalidCrc),
            esp_idf_sys::ESP_ERR_INVALID_VERSION => Ok(InvalidVersion),
            esp_idf_sys::ESP_ERR_INVALID_MAC => Ok(InvalidMac),
            esp_idf_sys::ESP_ERR_WIFI_BASE => Ok(WifiBase),
            esp_idf_sys::ESP_ERR_MESH_BASE => Ok(MeshBase),
            value => Err(EnumFromIntError(value)),
        }
    }
}