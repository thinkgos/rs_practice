use base64;
use base64::{engine::general_purpose, Engine};
use base64_url;
use data_encoding::{BASE64, BASE64URL_NOPAD};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value = b"Hello, world!";
    let base64_value = "SGVsbG8sIHdvcmxkIQ==";
    let base64_url_value = "SGVsbG8sIHdvcmxkIQ";

    println!("-----------------------  use base64 and base64_url crate --------------------");
    assert_eq!(general_purpose::STANDARD.encode(value), base64_value);
    assert_eq!(
        value,
        &general_purpose::STANDARD.decode(base64_value)?.as_ref()
    );

    assert_eq!(base64_url::encode(value), base64_url_value);
    assert_eq!(value, &base64_url::decode(base64_url_value)?.as_ref());

    println!("-----------------------  use data-encoding crate --------------------");
    assert_eq!(BASE64.encode(value), base64_value);
    assert_eq!(value, &BASE64.decode(base64_value.as_ref())?.as_ref());

    assert_eq!(BASE64URL_NOPAD.encode(value), base64_url_value);
    assert_eq!(
        value,
        &BASE64URL_NOPAD.decode(base64_url_value.as_ref())?.as_ref()
    );
    Ok(())
}
