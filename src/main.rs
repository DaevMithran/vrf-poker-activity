use rand::Rng;
use schnorrkel::context::SigningContext;
use sha2::{Digest, Sha256};

fn main() {

    // generate key pairs
    let player1_key_pair = schnorrkel::Keypair::generate();
    let player2_key_pair =schnorrkel::Keypair::generate();


    // generate random inputs
    let player_1_input: u32 = rand::thread_rng().gen();
    let player_2_input: u32 = rand::thread_rng().gen();

    // generate hash proofs
    let mut input_1_hasher = Sha256::new();
    input_1_hasher.update(&player_1_input.to_le_bytes());
    let input_1_commitment = input_1_hasher.finalize();

    let mut input_2_hasher = Sha256::new();
    input_2_hasher.update(&player_2_input.to_le_bytes());
    let input_2_commitment = input_2_hasher.finalize();


    // Share commitments and inputs to other players


    // VRF
    let vrf_input = player_1_input + player_2_input;
    let signing_context = SigningContext::new(b"abc");

    let (output_1, proof_1, _) = player1_key_pair.vrf_sign(signing_context.bytes(&vrf_input.to_le_bytes()));
    let (output_2, proof_2, _) = player2_key_pair.vrf_sign(signing_context.bytes(&vrf_input.to_le_bytes()));

    let output_1_u32: u32 = output_1.as_output_bytes().iter().map(|&v| v as u32).sum();
    let card_1 = output_1_u32 % 52;

    let output_2_u32: u32 = output_2.as_output_bytes().iter().map(|&v| v as u32).sum();
    let card_2 = output_2_u32 % 52;

    if card_1 > card_2 {
        println!("Player 1 is the winner")
    } else {
        println!("Player 2 is the winner")
    }

}
