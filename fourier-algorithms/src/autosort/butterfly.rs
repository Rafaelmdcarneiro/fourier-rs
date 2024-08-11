#[macro_export]
#[doc(hidden)]
macro_rules! butterfly2 {
    { $type:ty, $input:tt, $forward:tt } => { [add!($input[0], $input[1]), sub!($input[0], $input[1])] };
}

#[macro_export]
#[doc(hidden)]
macro_rules! butterfly3 {
    { $type:ty, $input:tt, $forward:tt } => {
        {
            let t = crate::twiddle::compute_twiddle(1, 3, $forward);
            let twiddle = broadcast!(t);
            let twiddle_conj = broadcast!(t.conj());
            [
                add!($input[0], add!($input[1], $input[2])),
                add!($input[0], add!(mul!($input[1], twiddle), mul!($input[2], twiddle_conj))),
                add!($input[0], add!(mul!($input[1], twiddle_conj), mul!($input[2], twiddle))),
            ]
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! butterfly4 {
    { $type:ty, $input:tt, $forward:tt } => {
        {
            let mut a = {
                let a0 = butterfly2!($type, [$input[0], $input[2]], $forward);
                let a1 = butterfly2!($type, [$input[1], $input[3]], $forward);
                [a0[0], a0[1], a1[0], a1[1]]
            };
            a[3] = rotate!(a[3], $forward);
            let b = {
                let b0 = butterfly2!($type, [a[0], a[2]], $forward);
                let b1 = butterfly2!($type, [a[1], a[3]], $forward);
                [b0[0], b0[1], b1[0], b1[1]]
            };
            [b[0], b[3], b[1], b[2]]
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! butterfly8 {
    { $type:ty, $input:tt, $forward:tt } => {
        {
            let t = crate::twiddle::compute_twiddle::<$type>(1, 8, $forward);
            let twiddle = broadcast!(t);
            let twiddle_neg = broadcast!(Complex::new(-t.re, t.im));
            let a1 = butterfly4!($type, [$input[0], $input[2], $input[4], $input[6]], $forward);
            let mut b1 = butterfly4!($type, [$input[1], $input[3], $input[5], $input[7]], $forward);
            b1[1] = mul!(b1[1], twiddle);
            b1[2] = rotate!(b1[2], !$forward);
            b1[3] = mul!(b1[3], twiddle_neg);
            let a2 = butterfly2!($type, [a1[0], b1[0]], forward);
            let b2 = butterfly2!($type, [a1[1], b1[1]], forward);
            let c2 = butterfly2!($type, [a1[2], b1[2]], forward);
            let d2 = butterfly2!($type, [a1[3], b1[3]], forward);
            [a2[0], b2[0], c2[0], d2[0], a2[1], b2[1], c2[1], d2[1]]
        }
    }
}
