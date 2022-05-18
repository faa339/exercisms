/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    return vec![];
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut buf = create_empty();
    for _ in 0..count{
        buf.push(0);
    }
    return buf;
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut buf = create_buffer(5);
    buf[0] = 1;
    buf[1] = 1;
    for n in 2..5{
        buf[n] = buf[n-2] + buf[n-1];
    }
    return buf;
}
