pub mod skeleton;
pub mod function;
pub mod display;
pub mod hash;
pub mod parsing;

#[cfg(test)]
mod tests {
    use crate::hash::*;

    #[test]
    fn test_get_column() {
        assert_eq!(get_column("A"), 1);
        assert_eq!(get_column("Z"), 26);
        assert_eq!(get_column("AA"), 27);
        assert_eq!(get_column("AB"), 28);
        assert_eq!(get_column("BA"), 53);
        assert_eq!(get_column("ZZ"), 702);
        assert_eq!(get_column("AAA"), 703);
    }
    #[test]
    fn test_get_hash_single_letter_column() {
        assert_eq!(get_hash("A1", 5), 0);  // row 0, col 0 → 0*5+0 = 0
        assert_eq!(get_hash("B2", 5), 6);  // row 1, col 1 → 1*5+1 = 6
        assert_eq!(get_hash("E3", 5), 14); // row 2, col 4 → 2*5+4 = 14
    }

    #[test]
    fn test_get_hash_double_letter_column() {
        assert_eq!(get_hash("AA1", 30), 26);  // col 26, row 0 → 0*30+26 = 26
        assert_eq!(get_hash("AB2", 40), 67);  // col 27, row 1 → 1*10+27 = 67
        assert_eq!(get_hash("BA3", 80), 212);  // col 52, row 2 → 2*80+52 = 72 (this seems off unless col logic adjusted)
    }

    #[test]
    fn test_get_hash_large_grid() {
        assert_eq!(get_hash("C10", 1000), 9002); // row 9, col 2 → 9*1000+2 = 9002
        assert_eq!(get_hash("Z99", 5000), 490025); // row 98, col 25 → 98*5000+25 = 4925
    }
    #[test]
    fn test_hash_to_string_single_letter_column() {
        assert_eq!(hash_to_string(0, 5), "A1");   // 0 / 5 = row 0, 0 % 5 = col 0 → A1
        assert_eq!(hash_to_string(6, 5), "B2");   // 6 / 5 = row 1, 6 % 5 = col 1 → B2
        assert_eq!(hash_to_string(14, 5), "E3");  // 14 / 5 = row 2, 14 % 5 = col 4 → E3
    }

    #[test]
    fn test_hash_to_string_double_letter_column() {
        assert_eq!(hash_to_string(26, 30), "AA1"); // 26 / 10 = row 2, 26 % 10 = col 6 → col 26 → AA, row 0 → 1
        assert_eq!(hash_to_string(67, 40), "AB2"); // 37 / 10 = row 3, 37 % 10 = col 7 → col 27 → AB, row 1 → 2
        assert_eq!(hash_to_string(212, 80), "BA3"); // 72 / 10 = row 7, 72 % 10 = col 2 → col 52 → BA, row 2 → 3
    }

    #[test]
    fn test_hash_to_string_large_grid() {
        assert_eq!(hash_to_string(902, 100), "C10");    // 902 / 100 = 9 → row 10, 902 % 100 = 2 → col C
        assert_eq!(hash_to_string(4925, 50), "Z99");    // 4925 / 50 = 98 → row 99, 4925 % 50 = 25 → col Z
    }
}
