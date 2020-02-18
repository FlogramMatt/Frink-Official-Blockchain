//Save list of Peers to Mining_Peers.csv but always keep the most recent list of peers in memory and lock file for writing.

//Each peer has:
// ip address
// hash of user's public key (for Account lookup)
//Flag - string that may be used to make an account as blacklisted or whitelisted

//open port num 37465 (Frink translated from letters to numbers using telephone keypad) as a server
fn OpenServerPort(port: i16){
    //open an enet server port that accepts connections
    PunchThroughFireWall(port);
}

//
fn PunchThroughFireWall(port: i16){
    // use this to punch through firewall: https://docs.rs/rustun/0.3.5/rustun/
}

// load a list of known IP addresses from a file
// connect to each one treating it like it's a server running on port 37465
// store a list of peer connections
fn Connect_to_Peers(){

}

//open Mining_Peers.csv, a CSV file containing a list of peers and load them all into memory
fn Load_Peers(){

}

// Add peer to miners in memory and save back to Mining_Peers.csv
fn Add_Peer(){
    //Just to make sure that the miners in memory are up to date with miners in file
    Load_Peers();


}

//Remove a Peer from miners in memory and save back to file
fn Remove_Peer(){

}

fn Black_List_Peer(){

}