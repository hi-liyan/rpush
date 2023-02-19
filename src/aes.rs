use base64::{Engine, engine::general_purpose};
use crypto::{symmetriccipher, buffer, aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use mac_address::get_mac_address;

/// # 加密函数
/// 返回 base64 字符串
pub fn encrypt(data: &str) -> Result<String, symmetriccipher::SymmetricCipherError> {

    let key = gen_key();

    let mut encryptor = aes::ecb_encryptor(
        aes::KeySize::KeySize256,
        &key,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    let base64_str = general_purpose::STANDARD.encode(final_result);
    Ok(base64_str)
}

/// # 解密函数
/// encrypted_data 参数为加密后的 base64 字符串
pub fn decrypt(encrypted_data: &str) -> Result<String, symmetriccipher::SymmetricCipherError> {
    let key = gen_key();
    let encrypted_data = general_purpose::STANDARD.decode(encrypted_data).unwrap();

    let mut decryptor = aes::ecb_decryptor(
        aes::KeySize::KeySize256,
        &key,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data.as_slice());
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(String::from_utf8(final_result).unwrap())
}

/// # 生成 key
/// key 长度 32 个字节，前 6 个字节使用本地 Mac 地址，其余位置用 0 占位
fn gen_key() -> [u8; 32] {
    let mac_addr_bytes = get_mac_address()
        .unwrap()
        .unwrap()
        .bytes();
    let mut result = [0u8; 32];

    for (i, elem) in mac_addr_bytes.iter().enumerate() {
        result[i] = *elem;
    }

    result
}

#[test]
fn test() {
    let message = "Hello, World";

    let encrypted_data = encrypt(message).unwrap();
    println!("加密：{:?}", encrypted_data);

    let decrypted_data = decrypt(&encrypted_data).ok().unwrap();
    println!("解密：{:?}", decrypted_data);

    assert_eq!(message, decrypted_data)
}
