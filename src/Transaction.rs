//Transaction bytes
// (See Ethereum transactions: https://ethereum.stackexchange.com/questions/30175/what-is-the-size-bytes-of-a-simple-ethereum-transaction-versus-a-bitcoin-trans )
//8 bytes - Nonce
//1 byte - Transaction Type (For now always 0 = simple transactions)  Some transaction types will have a sub-transaction type
//         Different transaction types will have different lengths and content organizations
//32 bytes - Hash of Sender Public Key
//32 bytes - Transaction Amount (type: f32)
//32 bytes - Hash of Receiver Public Key

fn Make_Transaction(){

}

fn Process_Transaction(){

}

fn Verify_Transaction(){

}

//once verified to be a valid transaction, forward it to other miners you are connected to
fn Propogate_Transaction(){

}