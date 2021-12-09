use ark_bls12_381::{G1Affine, Fq};
use ark_ec::AffineCurve;
use ark_serialize::CanonicalSerialize;
use num_bigint::BigUint;
use std::{
    fs::File,
    io::{BufReader, BufRead, Cursor, Write}
};
use text_io::scan;

fn main() {
    let file_in = File::open("g1_coeffs.dat").unwrap();
    let mut reader = BufReader::new(file_in);

    let mut line = String::new();

    let mut g1 = Vec::<G1Affine>::new();

    let mut counter = 0;

    loop {
        let _ = reader.read_line(&mut line).unwrap();
        if line.trim().len() == 0 {
            break;
        }

        let x_str: String;
        let y_str: String;

        scan!(line.bytes() => "{} {}", x_str, y_str);

        assert!(x_str.starts_with("0x"));
        assert!(y_str.starts_with("0x"));

        let x = BigUint::parse_bytes(&x_str.as_bytes()[2..], 16).unwrap();
        let y = BigUint::parse_bytes(&y_str.as_bytes()[2..], 16).unwrap();

        let x_field_elem: Fq = x.clone().into();
        let y_field_elem: Fq = y.clone().into();

        g1.push(G1Affine::new(x_field_elem, y_field_elem, false));
        counter = counter + 1;
        if counter % 10000 == 0 {
            println!("{}", counter);
        }

        line.clear();
    }

    for i in 10..=21 {
        let size = 1 << i;

        let mut file_out = File::create(format!("g1_2_{}_plus_3.dat", i)).unwrap();

        let mut serialized = vec![0u8; G1Affine::prime_subgroup_generator().serialized_size()];

        for elem in g1.iter().take(size + 3) {
            let mut cursor = Cursor::new(&mut serialized[..]);
            elem.serialize(&mut cursor).unwrap();

            file_out.write_all(&serialized[..]).unwrap();
        }
    }
}