use base64::{engine::general_purpose, DecodeError, Engine as _};

// go with url safe alphabet to allow GET request with query params
// see spec of RFC 4648 here: https://datatracker.ietf.org/doc/html/rfc4648#section-5
// also go with "no padding"
//
// rust package implementing RFC 4648: https://docs.rs/base64/latest/base64/alphabet/constant.URL_SAFE.html
// npm package implementing RFC 4648: https://www.npmjs.com/package/rfc4648
//
// notes to self:
// this is also what node:Buffer.from(str, "base64url") uses.
// (yes node:Buffer, base64url means "url safe alphabet without padding" even though docs links to wrong section of spec)
//
// keep in mind node:Buffer is not available in client side javascript and also react cant
// serialize node:Buffer when going from server component->client component etc
// so application code needs to use regular TypedArrays eg Uint8Array
//
// with https://www.npmjs.com/package/rfc4648 it looks like:
// base64url.stringify(uint8array, { pad: false })
// and
// base64url.parse(str, { loose: true })
//
// with node:Buffer it looks like
// buf.toString("base64url")
// and
// Buffer.from(str, "base64url")

pub fn base64string_to_vecu8(str: String) -> Result<Vec<u8>, DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(str)
}

pub fn vecu8_to_base64string(v: Vec<u8>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(v)
}
