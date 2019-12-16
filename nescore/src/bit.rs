//
// bit.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 15 2019
//

#[macro_export]
macro_rules! bv {
    ($b:expr) => {
        1 << $b
    };
}

#[macro_export]
macro_rules! mask_is_set {
    ($x:expr, $y:expr) => {
        $x & $y != 0
    };
}

#[macro_export]
macro_rules! mask_is_clear {
    ($x:expr, $y:expr) => {
        $x & $y == 0
    };
}

#[macro_export]
macro_rules! bit_is_set {
    ($x:expr, $y:expr) => {
        mask_is_set!($x, bv!($y))
    };
}

#[macro_export]
macro_rules! bit_is_clear {
    ($x:expr, $y:expr) => {
        mask_is_clear!($x, bv!($y))
    };
}

#[macro_export]
macro_rules! mask_set {
    ($x:expr, $y:expr) => {
        $x |= $y
    };
}

#[macro_export]
macro_rules! mask_clear {
    ($x:expr, $y:expr) => {
        $x &= !$y
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mask_is_set() {
        let x = 0x0F;
        let m = 0x01;

        assert_eq!(mask_is_set!(x, m), true);
    }

    #[test]
    fn test_mask_is_clear() {
        let x = 0x0E;
        let m = 0x01;

        assert_eq!(mask_is_clear!(x, m), true);
    }

    #[test]
    fn test_bit_is_set() {
        let x = 0x80;
        let b = 7;

        assert_eq!(bit_is_set!(x, b), true);
    }

    #[test]
    fn test_mask_set() {
        let mut x = 0x00;
        mask_set!(x, 0x0F);

        assert_eq!(x, 0x0F);
    }

    #[test]
    fn test_mask_clear() {
        let mut x = 0x0F;
        mask_clear!(x, 0x0F);

        assert_eq!(x, 0x00);
    }

    #[test]
    fn test_bit_value() {
        assert_eq!(bv!(0), 0x01);
        assert_eq!(bv!(1), 0x02);
        assert_eq!(bv!(2), 0x04);
        assert_eq!(bv!(3), 0x08);
        assert_eq!(bv!(4), 0x10);
        assert_eq!(bv!(5), 0x20);
        assert_eq!(bv!(6), 0x40);
        assert_eq!(bv!(7), 0x80);
    }
}
