use ark_bls12_381::{Fr, G1Affine, G2Affine};
use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_ff::fields::PrimeField;
use ark_std::{
    fs::File,
    io::{BufReader, Read, Cursor},
    UniformRand, ops::Neg, One
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

fn main() {
    let g2 = {
        let file_in = File::open("g2_2.dat").unwrap();
        let mut reader = BufReader::new(file_in);
        let len =  G2Affine::prime_subgroup_generator().uncompressed_size();

        let mut g2_str = Vec::<Vec<u8>>::new();
        loop {
            let mut serialized = vec![0u8; len];
            let res = reader.read_exact(&mut serialized[..]);
            if res.is_err() {
                break;
            }
            g2_str.push(serialized);
        }

        let mut g2 = Vec::with_capacity(2);
        for i in 0..=1 {
            let mut cursor = Cursor::new(&g2_str[i][..]);
            g2.push(G2Affine::deserialize_uncompressed(&mut cursor).unwrap());
        }
        g2_str.clear();
        g2
    };
        
    println!("Read G2 elements: {}", g2.len());
    assert_eq!(g2.len(), 2);

    let g1 = {
        let file_in = File::open("g1_2_21_plus_3.dat").unwrap();
        let mut reader = BufReader::new(file_in);
        let len =  G1Affine::prime_subgroup_generator().uncompressed_size();

        let mut g1_str = Vec::<Vec<u8>>::new();
        let mut counter = 0;
        loop {
            let mut serialized = vec![0u8; len];
            let res = reader.read_exact(&mut serialized[..]);
            if res.is_err() {
                break;
            }
            g1_str.push(serialized);

            counter += 1;
            if counter % 10000 == 0 {
                println!("{}", counter);
            }
        }

        let c = 10000;
        let idx: Vec<_> = (0..g1_str.len()).step_by(c).collect();

        let g1: Vec<G1Affine> = ark_std::cfg_into_iter!(idx).
            map(|i| {
                let mut g1_local = Vec::with_capacity(c);
                for j in i..std::cmp::min(i+c, g1_str.len()) {
                    let mut cursor = Cursor::new(&g1_str[j][..]);
                    g1_local.push(G1Affine::deserialize_uncompressed(&mut cursor).unwrap());
                }
                g1_local
            }).flatten().collect();

        g1_str.clear();
        g1
    };
    
    println!("Read G1 elements: {}", g1.len());
    assert_eq!(g1.len(), (1 << 21) + 3);

    let rand = {
        let c = 10000;
        let idx: Vec<_> = (0..g1.len()).step_by(c).collect();

        let rand: Vec<_> = ark_std::cfg_into_iter!(idx).
        map(|i| {
            let r = Fr::rand(&mut rand::thread_rng());
            let mut cur = r.clone();
            let mut rand_local = Vec::with_capacity(c);
            for _ in i..std::cmp::min(i+c, g1.len()) {
                rand_local.push(cur.into_repr());
                cur *= &r;
            }
            rand_local
        }).flatten().collect();

        rand
    };

    let accumulated_g1 = ark_ec::msm::VariableBase::msm(&g1[..g1.len() - 1], &rand[0..g1.len() - 1]);
    let accumulated_g1_shifted = ark_ec::msm::VariableBase::msm(&g1[1..g1.len()], &rand[0..g1.len() - 1]);

    let miller_loop_result = ark_bls12_381::Bls12_381::miller_loop(
        &[
            (accumulated_g1.into_affine().into(), g2[1].into()),
            (accumulated_g1_shifted.neg().into_affine().into(), g2[0].into())
        ]
    );

    let pairing_result = ark_bls12_381::Bls12_381::final_exponentiation(&miller_loop_result).unwrap();
    println!("{}", pairing_result);

    assert!(pairing_result.is_one());
}