// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn main() {
    let first: PseudoRandom;
    let second: PseudoRandom;
    let hash: HashAlgorithm;
    let gzip_codec: CompressionCodec;
    let left: ByteResult;
    let right: ByteResult;
    let first_child: PseudoRandom;
    let second_child: PseudoRandom;
    let bounded: RandomIntResult;
    let secure: SecureRandom;
    let secure_data: ByteResult;
    let secure_number: RandomIntResult;
    let digest: DigestResult;
    let wide_hash: HashAlgorithm;
    let wide_digest: DigestResult;
    let key: SecretBuffer;
    let same_key: SecretBuffer;
    let mac: SignatureResult;
    let same_mac: SignatureResult;
    let wide_mac: SignatureResult;
    let same_wide_mac: SignatureResult;
    let destroyed: SignatureResult;
    let unsupported: HashAlgorithm;
    let failed_digest: DigestResult;
    let failed_mac: SignatureResult;
    let strict: DecodeResult;
    let unpadded_as_padded: DecodeResult;
    let unpadded: DecodeResult;
    let wrong_alphabet: DecodeResult;
    let malformed: DecodeResult;
    let options: CompressionOptions;
    let packed: CompressionResult;
    let limits: DecompressionLimits;
    let unpacked: CompressionResult;
    let zlib_codec: CompressionCodec;
    let zlib_packed: CompressionResult;
    let zlib_unpacked: CompressionResult;
    let raw_codec: CompressionCodec;
    let raw_packed: CompressionResult;
    let raw_unpacked: CompressionResult;
    let zstd_codec: CompressionCodec;
    let zstd_packed: CompressionResult;
    let zstd_limits: DecompressionLimits;
    let zstd_unpacked: CompressionResult;
    let bomb_limits: DecompressionLimits;
    let refused: CompressionResult;
    let parsed: UuidResult;
    let generated_v4: UuidResult;
    let reparsed_v4: UuidResult;
    let generated_v7: UuidResult;
    let reparsed_v7: UuidResult;
    let noncanonical: UuidResult;
    let invalid_time: UuidResult;
    first = PseudoRandom::terrane_construct(
        chacha20(),
        Vec::<u8>::from([115, 101, 101, 100]),
    );
    second = PseudoRandom::terrane_construct(
        chacha20(),
        Vec::<u8>::from([115, 101, 101, 100]),
    );
    hash = sha256();
    gzip_codec = gzip();
    left = pseudo_bytes(first.clone(), terrane_int_support::Int::from(32_i128));
    right = pseudo_bytes(second.clone(), terrane_int_support::Int::from(32_i128));
    println!("{}", terrane_scalar_support::scalar_text(&(left.value == right.value)));
    first_child = split_pseudo(first.clone());
    second_child = split_pseudo(second);
    println!(
        "{}", terrane_scalar_support::scalar_text(&(pseudo_bytes(first_child,
        terrane_int_support::Int::from(16_i128)).value == pseudo_bytes(second_child,
        terrane_int_support::Int::from(16_i128)).value))
    );
    bounded = pseudo_bounded_int(first, terrane_int_support::Int::from(17_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(bounded.value.clone() >=
        terrane_int_support::Int::from(0_i128) &&bounded.value.clone() <
        terrane_int_support::Int::from(17_i128)))
    );
    secure = SecureRandom::terrane_construct();
    secure_data = secure_bytes(secure.clone(), terrane_int_support::Int::from(16_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! secure_data.failed
        &&terrane_int_support::Int::from(secure_data.value.len() as i128) ==
        terrane_int_support::Int::from(16_i128)))
    );
    secure_number = secure_bounded_int(
        secure.clone(),
        terrane_int_support::Int::from(17_i128),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! secure_number.failed
        &&secure_number.value.clone() >= terrane_int_support::Int::from(0_i128)
        &&secure_number.value.clone() < terrane_int_support::Int::from(17_i128)))
    );
    digest = digest_bytes(hash.clone(), Vec::<u8>::from([97, 98, 99]));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_hex(digest.value.value
        .clone()))
    );
    wide_hash = sha512();
    wide_digest = digest_bytes(wide_hash.clone(), Vec::<u8>::from([97, 98, 99]));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_hex(wide_digest.value.value
        .clone()))
    );
    key = SecretBuffer::terrane_construct(Vec::<u8>::from([107, 101, 121]));
    same_key = SecretBuffer::terrane_construct(Vec::<u8>::from([107, 101, 121]));
    mac = sign_hmac(hash.clone(), key.clone(), Vec::<u8>::from([100, 97, 116, 97]));
    same_mac = sign_hmac(
        hash.clone(),
        same_key.clone(),
        Vec::<u8>::from([100, 97, 116, 97]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&signature_equals(mac.value, same_mac
        .value))
    );
    wide_mac = sign_hmac(
        wide_hash.clone(),
        key.clone(),
        Vec::<u8>::from([100, 97, 116, 97]),
    );
    same_wide_mac = sign_hmac(
        wide_hash,
        same_key.clone(),
        Vec::<u8>::from([100, 97, 116, 97]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&signature_equals(wide_mac.value,
        same_wide_mac.value))
    );
    destroy_secret(same_key.clone());
    destroyed = sign_hmac(hash, same_key, Vec::<u8>::from([100, 97, 116, 97]));
    println!("{}", terrane_scalar_support::scalar_text(&destroyed.failed));
    unsupported = HashAlgorithm::terrane_construct(String::from("unsupported"));
    failed_digest = digest_bytes(
        unsupported.clone(),
        Vec::<u8>::from([100, 97, 116, 97]),
    );
    println!("{}", terrane_scalar_support::scalar_text(&failed_digest.failed));
    failed_mac = sign_hmac(unsupported, key, Vec::<u8>::from([100, 97, 116, 97]));
    println!("{}", terrane_scalar_support::scalar_text(&failed_mac.failed));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_base64(Vec:: < u8 >
        ::from([104, 101, 108, 108, 111]), false, true))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_base64(Vec:: < u8 >
        ::from([104, 101, 108, 108, 111, 63]), true, false))
    );
    strict = decode_base64(String::from("aGVsbG8="), false, true);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&strict
        .value, terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:51:13-51:38 */))
    );
    unpadded_as_padded = decode_base64(String::from("aGVsbG8"), false, true);
    println!("{}", terrane_scalar_support::scalar_text(&unpadded_as_padded.failed));
    unpadded = decode_base64(String::from("aGVsbG8"), false, false);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&unpadded
        .value, terrane_string_support::Encoding::Utf8), 1 /* terrane-site: case.trn:55:13-55:40 */))
    );
    wrong_alphabet = decode_base64(String::from("aGVsbG8_"), false, false);
    println!("{}", terrane_scalar_support::scalar_text(&wrong_alphabet.failed));
    malformed = decode_hex(String::from("abc"));
    println!("{}", terrane_scalar_support::scalar_text(&malformed.failed));
    options = CompressionOptions::terrane_construct(
        terrane_int_support::Int::from(6_i128),
        true,
    );
    packed = gzip_codec
        .compress(
            Vec::<u8>::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options.clone(),
        );
    limits = DecompressionLimits::terrane_construct(
        terrane_int_support::Int::from(1024_i128),
        terrane_int_support::Int::from(100_i128),
        terrane_int_support::Int::from(4096_i128),
    );
    unpacked = gzip_codec.decompress(packed.value.clone(), limits.clone());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&unpacked
        .value, terrane_string_support::Encoding::Utf8), 2 /* terrane-site: case.trn:64:13-64:40 */))
    );
    zlib_codec = zlib();
    zlib_packed = zlib_codec
        .compress(
            Vec::<u8>::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options.clone(),
        );
    zlib_unpacked = zlib_codec.decompress(zlib_packed.value.clone(), limits.clone());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&zlib_unpacked
        .value, terrane_string_support::Encoding::Utf8), 3 /* terrane-site: case.trn:68:13-68:45 */))
    );
    raw_codec = deflate_raw();
    raw_packed = raw_codec
        .compress(
            Vec::<u8>::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options.clone(),
        );
    raw_unpacked = raw_codec.decompress(raw_packed.value.clone(), limits);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&raw_unpacked
        .value, terrane_string_support::Encoding::Utf8), 4 /* terrane-site: case.trn:72:13-72:44 */))
    );
    zstd_codec = zstd();
    zstd_packed = zstd_codec
        .compress(
            Vec::<u8>::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options,
        );
    zstd_limits = DecompressionLimits::terrane_construct(
        terrane_int_support::Int::from(1073741824_i128),
        terrane_int_support::Int::from(100_i128),
        terrane_int_support::Int::from(1073741824_i128),
    );
    zstd_unpacked = zstd_codec.decompress(zstd_packed.value.clone(), zstd_limits);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&zstd_unpacked
        .value, terrane_string_support::Encoding::Utf8), 5 /* terrane-site: case.trn:77:13-77:45 */))
    );
    bomb_limits = DecompressionLimits::terrane_construct(
        terrane_int_support::Int::from(4_i128),
        terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(8_i128),
    );
    refused = gzip_codec.decompress(packed.value.clone(), bomb_limits);
    println!("{}", terrane_scalar_support::scalar_text(&refused.resource_limit));
    parsed = parse_uuid(String::from("01890f3e-7b4d-7cc0-98c8-77e22c318a14"));
    println!("{}", terrane_scalar_support::scalar_text(&parsed.value.string));
    generated_v4 = random_uuid(secure.clone());
    reparsed_v4 = parse_uuid(generated_v4.value.string.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! generated_v4.failed &&! reparsed_v4
        .failed &&terrane_int_support::Int::from(generated_v4.value.bytes.len() as i128)
        == terrane_int_support::Int::from(16_i128)))
    );
    generated_v7 = time_uuid(
        secure.clone(),
        terrane_int_support::Int::from(1700000000000_i128),
    );
    reparsed_v7 = parse_uuid(generated_v7.value.string.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! generated_v7.failed &&! reparsed_v7
        .failed &&terrane_int_support::Int::from(generated_v7.value.bytes.len() as i128)
        == terrane_int_support::Int::from(16_i128)))
    );
    noncanonical = parse_uuid(String::from("01890F3E-7B4D-7CC0-98C8-77E22C318A14"));
    println!("{}", terrane_scalar_support::scalar_text(&noncanonical.failed));
    invalid_time = time_uuid(secure, terrane_int_support::Int::from(-1_i128));
    println!("{}", terrane_scalar_support::scalar_text(&invalid_time.failed));
}
