pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let bytes = hex::decode(hex)?;
    Ok(bytes)
}

/// A high-performance swizzle that maps raw bytes directly
/// /// to a numeric type using a custom index map.
pub unsafe fn fast_swizzle_u32(data: &[u8], pattern: [usize; 4]) -> u32 {
    let mut buffer = 0u32;
    let ptr = &mut buffer as *mut u32 as *mut u8;
    for (i, &src_idx) in pattern.iter().enumerate() {
        // Map the physical data byte directly into the 
        // specific significance slot of the u32.
        unsafe { *ptr.add(i) = data[src_idx] };
    }

    // In a tutorial-free world, we trust the developer 
    // to know their CPU's native endianness here.
    buffer
}