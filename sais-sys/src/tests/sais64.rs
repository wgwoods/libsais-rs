use lazy_static::lazy_static;

use crate::sais64::*;
use crate::tests::common::*;

lazy_static! {
    static ref TEXTS: Vec<Vec<u8>> = {
        let mut samples = vec![
            b"".to_vec(),
            b"_".to_vec(),
            b"\x00\xff".to_vec(),
            b"mississippi".to_vec(),
            b"the quick brown fox jumps over the lazy dog".to_vec(),
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_vec(),
            b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
            Egestas egestas fringilla phasellus faucibus scelerisque eleifend donec pretium vulputate. Feugiat in fermentum posuere urna. \
            Amet nisl purus in mollis nunc. Tellus orci ac auctor augue mauris augue. Dolor morbi non arcu risus quis varius quam quisque id. \
            Et malesuada fames ac turpis egestas sed tempus. Eget mi proin sed libero enim sed faucibus. Turpis massa sed elementum tempus. \
            Congue eu consequat ac felis donec."
                .to_vec(),
        ];
        samples.push(random_text(100..=200, 0..=4));
        samples.push(random_text(100..=200, 0..=16));
        samples.push(random_text(100..=200, 0..=64));
        samples.push(random_text(100..=200, 128..=255));
        samples.push(random_text(100000..=200000, 0..=4));
        samples.push(random_text(100000..=200000, 0..=16));
        samples.push(random_text(100000..=200000, 0..=64));
        samples.push(random_text(100000..=200000, 128..=255));
        samples
    };
}

#[test]
fn test_sais() {
    let texts: Vec<&[u8]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut sa in allocate_suffix_arrays(t.len()) {
            // sais
            sais(t, sa.as_mut_slice(), None).expect("sais failed");
            check_suffix_array(t, sa.as_slice());

            // sais, w/ output symbol frequency table
            let mut freq = [0i64; FREQ_TABLE_SIZE];
            sais(t, sa.as_mut_slice(), Some(&mut freq)).expect("sais failed");
            check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
            check_suffix_array(t, sa.as_slice());
        }
    }
}

#[test]
#[cfg(feature = "openmp")]
fn test_sais_openmp() {
    let texts: Vec<&[u8]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut sa in allocate_suffix_arrays(t.len()) {
            // openmp::sais
            openmp::sais(t, sa.as_mut_slice(), None, 0).expect("sais failed");
            check_suffix_array(t, sa.as_slice());

            // openmp::sais, w/ output symbol frequency table
            let mut freq = [0i64; FREQ_TABLE_SIZE];
            openmp::sais(t, sa.as_mut_slice(), Some(&mut freq), 0).expect("sais failed");
            check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
            check_suffix_array(t, sa.as_slice());
        }
    }
}

#[test]
fn test_bwt_unbwt() {
    let texts: Vec<&[u8]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut a in allocate_suffix_arrays(t.len()) {
            let mut u = vec![0u8; t.len()];
            let mut s = vec![0u8; t.len()];

            // bwt + unbwt
            let i = bwt(t, u.as_mut_slice(), a.as_mut_slice(), None).expect("bwt failed");
            unbwt(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), None, i).expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // bwt + unbwt, w/ output symbol frequency table
            let mut freq = [0i64; FREQ_TABLE_SIZE];
            bwt(t, u.as_mut_slice(), a.as_mut_slice(), Some(&mut freq)).expect("bwt failed");
            check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
            unbwt(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), Some(&freq), i).expect("unbwt failed");
            assert_eq!(t, s.as_slice());
        }
    }
}

#[test]
#[cfg(feature = "openmp")]
fn test_bwt_unbwt_openmp() {
    let texts: Vec<&[u8]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut a in allocate_suffix_arrays(t.len()) {
            let mut u = vec![0u8; t.len()];
            let mut s = vec![0u8; t.len()];

            // openmp::bwt + openmp::unbwt
            let i = openmp::bwt(t, u.as_mut_slice(), a.as_mut_slice(), None, 0).expect("bwt failed");
            openmp::unbwt(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), None, i, 0).expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // openmp::bwt + openmp::unbwt, w/ output symbol frequency table
            let mut freq = [0i64; FREQ_TABLE_SIZE];
            openmp::bwt(t, u.as_mut_slice(), a.as_mut_slice(), Some(&mut freq), 0).expect("bwt failed");
            check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
            openmp::unbwt(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), Some(&freq), i, 0).expect("unbwt failed");
            assert_eq!(t, s.as_slice());
        }
    }
}

#[test]
fn test_bwt_unbwt_aux() {
    let texts: Vec<&[u8]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut a in allocate_suffix_arrays(t.len()) {
            let mut u = vec![0u8; t.len()];
            let mut s = vec![0u8; t.len()];
            let mut i = vec![0i64; Ord::max(t.len() / 4, 1)];

            // bwt_aux + unbwt_aux
            bwt_aux(t, u.as_mut_slice(), a.as_mut_slice(), None, i.as_mut_slice()).expect("bwt failed");
            unbwt_aux(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), None, i.as_slice()).expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // bwt_aux + unbwt_aux, w/ output symbol frequency table
            let mut freq = [0i64; FREQ_TABLE_SIZE];
            bwt_aux(t, u.as_mut_slice(), a.as_mut_slice(), Some(&mut freq), i.as_mut_slice()).expect("bwt failed");
            check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
            unbwt_aux(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), Some(&freq), i.as_slice()).expect("unbwt failed");
            assert_eq!(t, s.as_slice());
        }
    }
}

#[test]
#[cfg(feature = "openmp")]
fn test_bwt_unbwt_aux_openmp() {
    let texts: Vec<&[u8]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut a in allocate_suffix_arrays(t.len()) {
            let mut u = vec![0u8; t.len()];
            let mut s = vec![0u8; t.len()];
            let mut i = vec![0i64; Ord::max(t.len() / 4, 1)];

            // openmp::bwt_aux + openmp::unbwt_aux
            openmp::bwt_aux(t, u.as_mut_slice(), a.as_mut_slice(), None, i.as_mut_slice(), 0).expect("bwt failed");
            openmp::unbwt_aux(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), None, i.as_slice(), 0).expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // openmp::bwt_aux + openmp::unbwt_aux, w/ output symbol frequency table
            let mut freq = [0i64; FREQ_TABLE_SIZE];
            openmp::bwt_aux(t, u.as_mut_slice(), a.as_mut_slice(), Some(&mut freq), i.as_mut_slice(), 0).expect("bwt failed");
            check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
            openmp::unbwt_aux(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), Some(&freq), i.as_slice(), 0).expect("unbwt failed");
            assert_eq!(t, s.as_slice());
        }
    }
}

#[test]
fn test_plcp_lcp() {
    let texts: Vec<&[u8]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut sa = vec![0i64; t.len()];
        let mut plcp_array = vec![0i64; t.len()];
        let mut lcp_array = vec![0i64; t.len()];

        sais(t, sa.as_mut_slice(), None).expect("sais failed");

        // plcp + lcp
        plcp(t, sa.as_slice(), plcp_array.as_mut_slice()).expect("plcp failed");
        lcp(plcp_array.as_slice(), sa.as_slice(), lcp_array.as_mut_slice()).expect("lcp failed");
        check_lcp_array(t, sa.as_slice(), lcp_array.as_slice());
    }
}

#[test]
#[cfg(feature = "openmp")]
fn test_plcp_lcp_openmp() {
    let texts: Vec<&[u8]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut sa = vec![0i64; t.len()];
        let mut plcp_array = vec![0i64; t.len()];
        let mut lcp_array = vec![0i64; t.len()];

        openmp::sais(t, sa.as_mut_slice(), None, 0).expect("sais failed");

        // openmp::plcp + openmp::lcp
        openmp::plcp(t, sa.as_slice(), plcp_array.as_mut_slice(), 0).expect("plcp failed");
        openmp::lcp(plcp_array.as_slice(), sa.as_slice(), lcp_array.as_mut_slice(), 0).expect("lcp failed");
        check_lcp_array(t, sa.as_slice(), lcp_array.as_slice());
    }
}
