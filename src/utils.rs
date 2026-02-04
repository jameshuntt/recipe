pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let bytes = hex::decode(hex)?;
    Ok(bytes)
}
