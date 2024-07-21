use rsa::{RsaPublicKey, pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPrivateKey, pkcs1::{DecodeRsaPrivateKey}};
use wasm_bindgen::prelude::*;

use base64::prelude::*;
// 将该函数导出为 WebAssembly 模块
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
#[wasm_bindgen(start)]
fn run()  -> Result<(),  JsValue> {
    let window = web_sys::window().expect("no global  ");
    let document = window.document().expect("not found document");
    let body = document.body().expect("not found body");
    println!("{:?}", window);
    println!("{:?}", document);
    println!("{:?}", body);

    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello P!"));

    body.append_child(&val)?;
    Ok(())
}

#[wasm_bindgen] 
pub fn encrypt(val: &str) -> String {
    let pem = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApvudAcNxs5WQqbQCiYQ3
4EI2e86jAf+wCZQneVWSBKq5ck3DpCPby4UhGEoL4Fx7cDqzNgj1Kf2pBgllk08e
Xvwqh9VcXoxzRfYnx920JKYs/UPkhj9i4jYQ6kOSmqJ5UsT9PKqyUT+dNDURKcKR
Q5OmKyQ3go+E2Wld0JJLAd2Iixj2wa5vr4mMM4mq9AtzEvovo90M2eOT6vwFn94E
yeBYesYohkO2YXQJ6yEP6k7BZo4eGcid3iTJ7wZbTQXW/quILlVutSOWheoPLCSu
sRghVJt38UKtUMOGUfqMBlaeQTRb1BedlapYLFld/VucdUGxdM28KwMgS/f3UqI6
QwIDAQAB
-----END PUBLIC KEY-----";
    let pub_key = RsaPublicKey::from_public_key_pem(pem).unwrap();
    let mut rng = rand::thread_rng();

    let encry_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, val.as_bytes()).unwrap();
    // base64::encode(encry_date)
    BASE64_STANDARD.encode(encry_data)

}

#[wasm_bindgen]
pub fn decrypt(val: &str) -> String {
    let private_pem = "-----BEGIN RSA PRIVATE KEY-----
MIIEogIBAAKCAQEApvudAcNxs5WQqbQCiYQ34EI2e86jAf+wCZQneVWSBKq5ck3D
pCPby4UhGEoL4Fx7cDqzNgj1Kf2pBgllk08eXvwqh9VcXoxzRfYnx920JKYs/UPk
hj9i4jYQ6kOSmqJ5UsT9PKqyUT+dNDURKcKRQ5OmKyQ3go+E2Wld0JJLAd2Iixj2
wa5vr4mMM4mq9AtzEvovo90M2eOT6vwFn94EyeBYesYohkO2YXQJ6yEP6k7BZo4e
Gcid3iTJ7wZbTQXW/quILlVutSOWheoPLCSusRghVJt38UKtUMOGUfqMBlaeQTRb
1BedlapYLFld/VucdUGxdM28KwMgS/f3UqI6QwIDAQABAoIBAE/psqYZZtzPA0B0
CZLOV/9XMMPnjFN0jkbHRGzo6syY988gMDTphFLOD5yUM0LYf6qiLQn7F3lVEAL3
lEj/YEWM4L3I+j+1b6VXvIoGLCQt8vHx//fPdf9UaJv0YsDmaLGX3Kmf916y1wUZ
MHGPz/1LOAcFehtdm/KAXolmucbV/HkR1uR8XlsPc/ZXPKfuQcsl6DIgsobTBIv3
IQGvKKf0+uLsbiLgR/mbFjOizXvQaVDmXj9McyBKD1p3ZqwPcfXOcL4/BpG1m4go
bq6RzvVafj/ZDjIiZXXp3N+SnCX0fS/MYEIFYpu1yBw3aXILsb2ipFYAcCRng/vu
o6L8B2ECgYEA1T8k6zpr0vJhOVBSnGBfliqg4i3/0u7qZ5/GO+3rnMuhfreDgm6L
SKPORi5Lxzij/Pt4iw0VoAwbGEFvMWa47WqrY5ZiWHaCqQzldMjuRQ1XGpyj+7f0
1LCeqF/dFGfo5M9B2kPYjcyQrczTodMnyWo5mLFwaGaQQbceXlch1McCgYEAyHX8
eABtIW29mj5P4NewF9wsWbQV7pnLqmRikwBmngBrbQKSd3nHKacHjLO3RT6YzRea
hxYlNc6qVFz9ku99ejw0U+VtYgykR8Ph12qerkZEs9YsMlA4AeQoPV3ksW95a3HN
zuv58FXxxSEA8DpatgHjBi5AL9jCVW2NTPZpOqUCgYAflpb1cVX/jX/xaYCTQYNn
TzE9z+qRaGq4puZKglfghhmKF80XSSDAxeMMalc3T6CbHPUiQ/HsH170HN1HyBuk
r6RrwB8ZU5TPTeW/gvQorm00t/dilkFiMDeQYYLyPuI04q6yklJFzwOveyPuWMUQ
BPg7Mi2giXDdCy8ocmfEiQKBgAXFl+pgoMPIpB/v0V749uII/eHrVIJVPU7/qvLu
nQXJn045N7xm7jwdyhLcDdfZkI6/MjzXf2TIOZFKSCbV7Hf7icosnfSnxocmdbc9
I6qC7XF8JjOgqawsRJhXvsD5I7bbRQDDPKQV5Ws2ow9Hrtr0vPOtmz2FcTsCd4IS
yj7dAoGAAWprwbFNn9azjwXFidCoMalG4uv5qcquNzpFny6MZES2IsNNoPlu3gUg
mfIq3lnP7yM619SfL43ctp0pZEBm9ed3unjg4w+0g6oSLqHfmVhnS+JFPkydia4+
3aNaOccds+btVBD8FdwO+DUuunT3wcypwhvqbnbOzVRKXnGEHng=
-----END RSA PRIVATE KEY-----";
    let private_key = RsaPrivateKey::from_pkcs1_pem(private_pem).unwrap();
    let data = BASE64_STANDARD.decode(val).unwrap();
    let plain_data = private_key.decrypt(Pkcs1v15Encrypt, &data[..]).unwrap();
    String::from_utf8(plain_data).unwrap()
}

