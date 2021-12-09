use ark_bls12_381::{G2Affine, Fq, Fq2};
use ark_ec::AffineCurve;
use ark_serialize::CanonicalSerialize;
use num_bigint::BigUint;
use std::{
    fs::File,
    io::{BufReader, BufRead, Cursor, Write}
};
use text_io::scan;

fn main() {
    let file_in = File::open("g2_coeffs.dat").unwrap();
    let mut reader = BufReader::new(file_in);

    let mut line = String::new();

    let mut g2 = Vec::<G2Affine>::new();

    for _ in 0..2 {
        let _ = reader.read_line(&mut line).unwrap();
        if line.trim().len() == 0 {
            break;
        }

        let x_c0_str: String;
        let x_c1_str: String;
        let y_c0_str: String;
        let y_c1_str: String;

        scan!(line.bytes() => "{} {} {} {}", x_c0_str, x_c1_str, y_c0_str, y_c1_str);

        assert!(x_c0_str.starts_with("0x"));
        assert!(x_c1_str.starts_with("0x"));
        assert!(y_c0_str.starts_with("0x"));
        assert!(y_c1_str.starts_with("0x"));

        let x_c0 = BigUint::parse_bytes(&x_c0_str.as_bytes()[2..], 16).unwrap();
        let x_c1 = BigUint::parse_bytes(&x_c1_str.as_bytes()[2..], 16).unwrap();

        let y_c0 = BigUint::parse_bytes(&y_c0_str.as_bytes()[2..], 16).unwrap();
        let y_c1 = BigUint::parse_bytes(&y_c1_str.as_bytes()[2..], 16).unwrap();

        let x_c0_field_elem: Fq = x_c0.clone().into();
        let x_c1_field_elem: Fq = x_c1.clone().into();

        let y_c0_field_elem: Fq = y_c0.clone().into();
        let y_c1_field_elem: Fq = y_c1.clone().into();

        let x_field_elem = Fq2::new(x_c0_field_elem, x_c1_field_elem);
        let y_field_elem = Fq2::new(y_c0_field_elem, y_c1_field_elem);

        let elem = G2Affine::new(x_field_elem, y_field_elem, false);
        assert!(elem.is_on_curve());
        g2.push(elem);
        
        line.clear();
    }

    let mut file_out = File::create("g2_2.dat").unwrap();
    let mut serialized = vec![0u8; G2Affine::prime_subgroup_generator().uncompressed_size()];

    for elem in g2.iter() {
        let mut cursor = Cursor::new(&mut serialized[..]);
        elem.serialize_uncompressed(&mut cursor).unwrap();

        file_out.write_all(&serialized[..]).unwrap();
    }
}