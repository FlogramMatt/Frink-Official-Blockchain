//use falcon sign for signing data: https://falcon-sign.info/index.htm
//will need to create a Rust FFI binding either manually or using something like this:
//https://github.com/rust-lang/rust-bindgen

//generate and return a new public key, private key pair
fn generate_key_pair() -> (Vec<u8>, Vec<u8>)
{

}

//produce a signature for data

//returns the signature
fn sign_data(data_in, private key) -> Vec<u8>
{

}

//verify a signature is valid against a chunk of data and a public key

fn verify_data(data_in, public key, signature) -> bool
{

}