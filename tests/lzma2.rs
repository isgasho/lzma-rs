#[cfg(feature = "enable_logging")]
extern crate env_logger;
#[macro_use]
extern crate lzma_rs;
#[cfg(feature = "enable_logging")]
#[macro_use]
extern crate log;

fn round_trip(x: &[u8]) {
    let mut compressed: Vec<u8> = Vec::new();
    lzma_rs::lzma2_compress(&mut std::io::BufReader::new(x), &mut compressed).unwrap();
    lzma_info!("Compressed {} -> {} bytes", x.len(), compressed.len());
    lzma_debug!("Compressed content: {:?}", compressed);
    let mut bf = std::io::BufReader::new(compressed.as_slice());
    let mut decomp: Vec<u8> = Vec::new();
    lzma_rs::lzma2_decompress(&mut bf, &mut decomp).unwrap();
    assert_eq!(decomp, x)
}

fn round_trip_file(filename: &str) {
    use std::io::Read;

    let mut x = Vec::new();
    std::fs::File::open(filename)
        .unwrap()
        .read_to_end(&mut x)
        .unwrap();
    round_trip(x.as_slice());
}

#[test]
fn round_trip_basics() {
    #[cfg(feature = "enable_logging")]
    let _ = env_logger::try_init();
    round_trip(b"");
    // Note: we use vec! to avoid storing the slice in the binary
    round_trip(vec![0x00; 1_000_000].as_slice());
    round_trip(vec![0xFF; 1_000_000].as_slice());
}

#[test]
fn round_trip_hello() {
    #[cfg(feature = "enable_logging")]
    let _ = env_logger::try_init();
    round_trip(b"Hello world");
}

#[test]
fn round_trip_files() {
    #[cfg(feature = "enable_logging")]
    let _ = env_logger::try_init();
    round_trip_file("tests/files/foo.txt");
}
