use crate::core_api::math::Random;
use crate::crypto_api;

#[test]
fn test_encode_encrypt_x_decrypt() {
    // random settings
    let (min, max) = generate_random_interval!();
    let (precision, padding) = generate_precision_padding!(8, 8);

    // encoder
    let encoder = crypto_api::Encoder::new(min, max, precision, padding).unwrap();

    // generate a secret key
    let secret_key = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);

    // a list of messages
    let message: f64 = random_message!(min, max);

    // encode and encrypt
    let ciphertext = crypto_api::LWE::encode_encrypt(&secret_key, message, &encoder).unwrap();

    // decryption
    let decryption: f64 = ciphertext.decrypt_decode(&secret_key).unwrap();

    // test
    assert_eq_granularity!(message, decryption, ciphertext.encoder);
    assert_eq!(precision, ciphertext.encoder.nb_bit_precision);
}

#[test]
fn test_encode_encrypt_x_add_constant_static_encoder_inplace_x_decrypt() {
    // random settings
    let (min, max) = generate_random_interval!();
    let (precision, padding) = generate_precision_padding!(8, 8);

    // encoder
    let encoder = crypto_api::Encoder::new(min - 10., max + 10., precision, padding).unwrap();

    // generate a secret key
    let secret_key = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);

    // two lists of messages
    let message_1: f64 = random_message!(min, max);
    let message_2: f64 = random_message!(-10., 10.);

    // encode and encrypt
    let mut ciphertext = crypto_api::LWE::encode_encrypt(&secret_key, message_1, &encoder).unwrap();

    // addition between ciphertext and messages_2
    ciphertext
        .add_constant_static_encoder_inplace(message_2)
        .unwrap();

    // decryption
    let decryption: f64 = ciphertext.decrypt_decode(&secret_key).unwrap();

    // test
    assert_eq_granularity!(message_1 + message_2, decryption, ciphertext.encoder);
    assert_eq!(precision, ciphertext.encoder.nb_bit_precision);
}

#[test]
fn test_encode_encrypt_x_add_constant_dynamic_encoder_decrypt() {
    // random settings
    let (min1, max1) = generate_random_interval!();
    let (min2, max2) = generate_random_interval!();
    let (precision, padding) = generate_precision_padding!(8, 8);

    // encoder
    let encoder = crypto_api::Encoder::new(min1, max1, precision, padding).unwrap();

    // generate a secret key
    let secret_key = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);

    // two lists of messages
    let message_1: f64 = random_message!(min1, max1);
    let message_2: f64 = random_message!(min2, max2);

    // encode and encrypt
    let mut ciphertext = crypto_api::LWE::encode_encrypt(&secret_key, message_1, &encoder).unwrap();

    // addition between ciphertext and messages_2
    ciphertext
        .add_constant_dynamic_encoder_inplace(message_2)
        .unwrap();

    // decryption
    let decryption: f64 = ciphertext.decrypt_decode(&secret_key).unwrap();

    // test
    assert_eq_granularity!(message_1 + message_2, decryption, ciphertext.encoder);
    assert_eq!(precision, ciphertext.encoder.nb_bit_precision);
}

#[test]
fn test_encode_encrypt_x_opposite_inplace_x_decrypt() {
    // random settings
    let (min, max) = generate_random_interval!();
    let (precision, padding) = generate_precision_padding!(8, 8);

    // encoder
    let encoder = crypto_api::Encoder::new(min, max, precision, padding).unwrap();

    // generate a secret key
    let secret_key = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);

    // a list of messages
    let message: f64 = random_message!(min, max);

    // encode and encrypt
    let mut ciphertext = crypto_api::LWE::encode_encrypt(&secret_key, message, &encoder).unwrap();

    // compute the opposite of the second ciphertext
    ciphertext.opposite_inplace().unwrap();

    // decryption
    let decryption: f64 = ciphertext.decrypt_decode(&secret_key).unwrap();

    // test
    assert_eq_granularity!(-message, decryption, ciphertext.encoder);
}

#[test]
fn test_encode_encrypt_x_add_centered_inplace_x_decrypt() {
    // random settings
    let (min, max) = generate_random_interval!();
    let (precision, padding) = generate_precision_padding!(8, 8);

    // encoders
    let encoder1 = crypto_api::Encoder::new(min - 100., max + 100., precision, padding).unwrap();
    let encoder2 = crypto_api::Encoder::new(
        -(max - min) / 2. - 100.,
        (max - min) / 2. + 100.,
        precision,
        padding,
    )
    .unwrap();

    // generate a secret key
    let secret_key = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);

    // two lists of messages
    let message1: f64 = random_message!(min, max);
    let message2: f64 = random_message!(-100., 100.);

    // encode and encrypt
    let mut ciphertext1 =
        crypto_api::LWE::encode_encrypt(&secret_key, message1, &encoder1).unwrap();
    let ciphertext2 = crypto_api::LWE::encode_encrypt(&secret_key, message2, &encoder2).unwrap();

    // addition between ciphertext1 and ciphertext2
    ciphertext1.add_centered_inplace(&ciphertext2).unwrap();

    // decryption
    let decryption: f64 = ciphertext1.decrypt_decode(&secret_key).unwrap();

    // check the precision loss related to the encryption
    assert_eq_granularity!(message1 + message2, decryption, ciphertext1.encoder);
    assert_eq!(precision, ciphertext1.encoder.nb_bit_precision);
}

#[test]
fn test_encode_encrypt_x_add_with_padding_inplace_x_decrypt() {
    // random settings
    let (min1, max1) = generate_random_interval!();
    let (min2, _max2) = generate_random_interval!();
    let max2 = min2 + max1 - min1; // same interval size
    let (precision, mut padding) = generate_precision_padding!(8, 3);
    padding += 1; // at least one bit

    // encoders
    let encoder1 = crypto_api::Encoder::new(min1, max1, precision, padding).unwrap();
    let encoder2 = crypto_api::Encoder::new(min2, max2, precision, padding).unwrap();

    // generate a secret key
    let secret_key = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);

    // two messages
    let message1: f64 = random_message!(min1, max1);
    let message2: f64 = random_message!(min2, max2);

    // encode and encrypt
    let mut ciphertext1 =
        crypto_api::LWE::encode_encrypt(&secret_key, message1, &encoder1).unwrap();
    let ciphertext2 = crypto_api::LWE::encode_encrypt(&secret_key, message2, &encoder2).unwrap();

    // addition between ciphertext and message_2
    ciphertext1.add_with_padding_inplace(&ciphertext2).unwrap();

    // decryption
    let decryption: f64 = ciphertext1.decrypt_decode(&secret_key).unwrap();

    // check the precision loss related to the encryption
    assert_eq_granularity!(message1 + message2, decryption, ciphertext1.encoder);
    assert_eq!(precision, ciphertext1.encoder.nb_bit_precision);
}

#[test]
fn test_encode_encrypt_x_sub_with_padding_inplace_x_decrypt() {
    // random settings
    let (min1, max1) = generate_random_interval!();
    let (min2, _max2) = generate_random_interval!();
    let max2 = min2 + max1 - min1; // same interval size
    let (precision, mut padding) = generate_precision_padding!(8, 3);
    padding += 1; // at least one bit

    // encoders
    let encoder1 = crypto_api::Encoder::new(min1, max1, precision, padding).unwrap();
    let encoder2 = crypto_api::Encoder::new(min2, max2, precision, padding).unwrap();

    // generate a secret key
    let secret_key = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);

    // two lists of messages
    let message1: f64 = random_message!(min1, max1);
    let message2: f64 = random_message!(min2, max2);

    // encode and encrypt
    let mut ciphertext1 =
        crypto_api::LWE::encode_encrypt(&secret_key, message1, &encoder1).unwrap();
    let ciphertext2 = crypto_api::LWE::encode_encrypt(&secret_key, message2, &encoder2).unwrap();

    // subtraction between ciphertext and messages_2
    ciphertext1.sub_with_padding_inplace(&ciphertext2).unwrap();

    // decryption
    let decryption: f64 = ciphertext1.decrypt_decode(&secret_key).unwrap();

    // check the precision loss related to the encryption
    assert_eq_granularity!(message1 - message2, decryption, ciphertext1.encoder);
    assert_eq!(precision, ciphertext1.encoder.nb_bit_precision);
}

#[test]
fn test_encode_encrypt_x_mul_constant_static_encoder_inplace_x_decrypt() {
    // random settings
    let (min, max) = generate_random_centered_interval!();
    let (precision, padding) = generate_precision_padding!(6, 2);
    let b = min.abs().min(max.abs()) / 20.;

    // encoders
    let encoder = crypto_api::Encoder::new(min, max, precision, padding).unwrap();

    // generate a secret key
    let secret_key = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);

    // two lists of messages
    let message1: f64 = random_message!(-b, b);
    let message2_float: f64 = random_message!(-b, b);
    let message2: i32 = message2_float as i32;

    // encode and encrypt
    let mut ciphertext = crypto_api::LWE::encode_encrypt(&secret_key, message1, &encoder).unwrap();

    // multiplication between ciphertext and messages2
    ciphertext
        .mul_constant_static_encoder_inplace(message2)
        .unwrap();

    // decryption
    let decryption: f64 = ciphertext.decrypt_decode(&secret_key).unwrap();

    // check the precision loss related to the encryption
    assert_eq_granularity!(message1 * (message2 as f64), decryption, ciphertext.encoder);
    assert_eq!(precision, ciphertext.encoder.nb_bit_precision);
}

#[test]
fn test_encode_encrypt_x_mul_constant_with_padding_inplace_x_decrypt() {
    // random settings
    let (min, max) = generate_random_centered_interval!();
    let precision: usize = random_index!(5) + 3;
    let padding = random_index!(3) + precision;
    let nb_bit_padding_mult = precision;
    let b = (random_index!(300) + 3) as f64;

    // encoders
    let encoder = crypto_api::Encoder::new(min, max, precision, padding).unwrap();

    // generate a secret key
    let secret_key = crypto_api::LWESecretKey::new(&crypto_api::LWE128_2048);

    // two lists of messages
    let message1: f64 = random_message!(min, max);
    let message2: f64 = random_message!(-b, b);

    // encode and encrypt
    let mut ciphertext = crypto_api::LWE::encode_encrypt(&secret_key, message1, &encoder).unwrap();

    // multiplication between ciphertext and messages2
    ciphertext
        .mul_constant_with_padding_inplace(message2, b, nb_bit_padding_mult)
        .unwrap();

    // decryption
    let decryption: f64 = ciphertext.decrypt_decode(&secret_key).unwrap();

    // check the precision loss related to the encryption
    assert_eq_granularity!(message1 * message2, decryption, ciphertext.encoder);
}

#[test]
fn test_encode_encrypt_x_keyswitch_x_decrypt() {
    // random settings
    let (min, max) = generate_random_interval!();
    let (precision, padding) = generate_precision_padding!(12, 12);
    let base_log: usize = random_index!(9) + 1;
    let level: usize = random_index!(4) + 1;

    // encoder
    let encoder = crypto_api::Encoder::new(min, max, precision, padding).unwrap();

    // generate two secret keys
    let secret_key_before = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);
    let secret_key_after = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);

    // generate the key switching key
    let ksk = crypto_api::LWEKSK::new(&secret_key_before, &secret_key_after, base_log, level);

    // a list of messages that we encrypt
    let message: f64 = random_message!(min, max);
    let ciphertext_before =
        crypto_api::LWE::encode_encrypt(&secret_key_before, message, &encoder).unwrap();

    // key switch
    let ciphertext_after = ciphertext_before.keyswitch(&ksk).unwrap();

    // decryption
    let decryption: f64 = ciphertext_before
        .decrypt_decode(&secret_key_before)
        .unwrap();

    // test
    assert_eq_granularity!(message, decryption, ciphertext_after.encoder);
}

#[test]
fn test_encode_encrypt_x_bootstrap_x_decrypt() {
    // random settings
    let (min, max) = generate_random_interval!();
    let padding: usize = random_index!(3) + 1;
    let precision: usize = random_index!(3) + 1;
    let base_log: usize = random_index!(3) + 7;
    let level: usize = random_index!(1) + 3;

    // encoders
    let encoder_input = crypto_api::Encoder::new(min, max, precision, padding).unwrap();

    // secret keys
    let rlwe_secret_key = crypto_api::RLWESecretKey::new(&crypto_api::RLWE128_1024_1);
    let secret_key_input = crypto_api::LWESecretKey::new(&crypto_api::LWE128_630);
    let secret_key_output = rlwe_secret_key.to_lwe_secret_key();

    // bootstrapping key
    let bootstrapping_key =
        crypto_api::LWEBSK::new(&secret_key_input, &rlwe_secret_key, base_log, level);

    // messages
    let message: f64 = random_message!(min, max);

    // encode and encrypt
    let ciphertext_input =
        crypto_api::LWE::encode_encrypt(&secret_key_input, message, &encoder_input).unwrap();

    // bootstrap
    let ciphertext_output = ciphertext_input.bootstrap(&bootstrapping_key).unwrap();

    // decrypt
    let decryption2 = ciphertext_output
        .decrypt_decode(&secret_key_output)
        .unwrap();
    assert_eq_granularity!(message, decryption2, ciphertext_output.encoder);
}

#[test]
fn test_encode_encrypt_x_mul_from_bootstrap_x_decrypt() {
    // random settings for the first encoder and some messages
    let (min1, max1) = generate_random_interval!();
    let encoder_1 = crypto_api::Encoder::new(min1, max1, 5, 2).unwrap();
    let message_1: f64 = random_message!(min1, max1);

    // random settings for the second encoder and some messages
    let (min2, _max2) = generate_random_interval!();
    let max2 = min2 + max1 - min1;
    let encoder_2 = crypto_api::Encoder::new(min2, max2, 5, 2).unwrap();
    let message_2: f64 = random_message!(min2, max2);

    // generate a secret key
    let rlwe_secret_key = crypto_api::RLWESecretKey::new(&crypto_api::RLWE128_1024_1);
    let secret_key_input = crypto_api::LWESecretKey::new(&crypto_api::LWE128_630);
    let secret_key_output = rlwe_secret_key.to_lwe_secret_key();

    // bootstrapping key
    let bsk = crypto_api::LWEBSK::new(&secret_key_input, &rlwe_secret_key, 5, 3);

    // encode and encrypt
    let ciphertext_1 =
        crypto_api::LWE::encode_encrypt(&secret_key_input, message_1, &encoder_1).unwrap();
    let ciphertext_2 =
        crypto_api::LWE::encode_encrypt(&secret_key_input, message_2, &encoder_2).unwrap();

    // multiplication
    let ciphertext_res = ciphertext_1
        .mul_from_bootstrap(&ciphertext_2, &bsk)
        .unwrap();

    // decrypt
    let decryption = ciphertext_res.decrypt_decode(&secret_key_output).unwrap();

    // test
    assert_eq_granularity!(message_1 * message_2, decryption, ciphertext_res.encoder);
}

#[test]
fn test_encode_encrypt_x_add_with_new_min_inplace_x_decrypt() {
    // random number of messages
    let (precision, padding) = generate_precision_padding!(8, 8);

    // random settings for the first encoder and some messages
    let (min1, max1) = generate_random_interval!();
    let encoder_1 = crypto_api::Encoder::new(min1, max1, precision, padding).unwrap();
    let message_1: f64 = random_message!(min1 + encoder_1.get_size() / 2., max1);

    // random settings for the second encoder and some random messages
    let (min2, _max2) = generate_random_interval!();
    let encoder_2 =
        crypto_api::Encoder::new(min2, min2 + encoder_1.get_size(), precision, padding).unwrap();
    let message_2: f64 = random_message!(min2, min2 + encoder_1.get_size() / 2.);

    // generate a secret key
    let secret_key = crypto_api::LWESecretKey::new(&crypto_api::LWE128_1024);

    // encode and encrypt
    let mut ciphertext_1 =
        crypto_api::LWE::encode_encrypt(&secret_key, message_1, &encoder_1).unwrap();
    let ciphertext_2 = crypto_api::LWE::encode_encrypt(&secret_key, message_2, &encoder_2).unwrap();

    // new_min
    let new_min: f64 = min1 + min2 + encoder_1.get_size() / 2.;

    // addition between ciphertext_1 and ciphertext_2
    ciphertext_1
        .add_with_new_min_inplace(&ciphertext_2, new_min)
        .unwrap();

    // decryption
    let decryption = ciphertext_1.decrypt_decode(&secret_key).unwrap();

    // test
    assert_eq_granularity!(message_1 + message_2, decryption, ciphertext_1.encoder);
    assert_eq!(precision, ciphertext_1.encoder.nb_bit_precision);
}