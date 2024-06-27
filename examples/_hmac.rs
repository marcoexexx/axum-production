use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::Sha512;

fn encode(content: String, salt: String) -> Result<String> {
  let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(&[0u8; 64])?;
  hmac_sha512.update(content.as_bytes());
  hmac_sha512.update(salt.as_bytes());

  let hmac_result = hmac_sha512.finalize();
  let b64u = base64_url::encode(&hmac_result.into_bytes());

  Ok(b64u)
}

fn main() -> Result<()> {
  let content = String::from("hello");
  let b64u = encode(content, String::from("pepper"))?;

  println!("{}", b64u);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Result;

  #[test]
  fn test_reverse_b64u() -> Result<()> {
    let fx_content = String::from("hello");
    let fx_encoded = encode(fx_content, String::from("pepper"))?;

    let encoded =
      "FmxBF0ip-kdVF2F9wRE_tPJnj_h6BxmQsurqZ8Ot_0KMI-cNZithdfuAUPdOIGngw_ACZROU_iS1sgthRDqHWA";

    println!("fx: {}\n{}", fx_encoded, encoded);

    assert_eq!(fx_encoded, encoded);

    Ok(())
  }
}
