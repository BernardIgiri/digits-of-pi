use num_bigint::BigInt;
use num_traits::{cast::ToPrimitive, One, Zero};

type Quad = (BigInt, BigInt, BigInt, BigInt);

fn comp(a: Quad, b: Quad) -> Quad {
    let (q, r, s, t) = a;
    let (u, v, w, x) = b;

    (
        q.clone() * u.clone() + r.clone() * w.clone(),
        q * v.clone() + r * x.clone(),
        s.clone() * u + t.clone() * w,
        s * v + t * x,
    )
}

fn extr(z: Quad, x: u32) -> (BigInt, BigInt) {
    let (q, r, s, t) = z;
    (q * x + r, s * x + t)
}

fn prod(z: Quad, n: u32) -> Quad {
    let ten = BigInt::from(10);
    let neg = BigInt::from(-10i32) * n;
    let matrix = (ten, neg, BigInt::zero(), BigInt::one());
    comp(matrix, z)
}

fn safe(z: Quad, n: BigInt) -> bool {
    let (num, den) = extr(z, 4);
    (num / den) == n
}

fn cons(z: Quad, z1: Quad) -> Quad {
    comp(z, z1)
}

fn next(z: Quad) -> BigInt {
    let (num, den) = extr(z, 3);
    num / den
}

fn lfts(k: u32) -> Quad {
    (k.into(), (4 * k + 2).into(), 0.into(), (2 * k + 1).into())
}

struct PiGen {
    z: Quad,
    k: u32,
}

impl Iterator for PiGen {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let lft = lfts(self.k);
            let n = next(self.z.clone());
            if safe(self.z.clone(), n.clone()) {
                let n_int = n.to_u32().expect("Digit too large for u32");
                self.z = prod(self.z.clone(), n_int);
                return Some(n);
            } else {
                self.z = cons(self.z.clone(), lft);
                self.k += 1;
            }
        }
    }
}

pub fn generator() -> impl Iterator<Item = BigInt> {
    PiGen {
        z: (1.into(), 0.into(), 0.into(), 1.into()),
        k: 1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first100digits() {
        let expected: [u32; 100] = [
            3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2,
            7, 9, 5, 0, 2, 8, 8, 4, 1, 9, 7, 1, 6, 9, 3, 9, 9, 3, 7, 5, 1, 0, 5, 8, 2, 0, 9, 7, 4,
            9, 4, 4, 5, 9, 2, 3, 0, 7, 8, 1, 6, 4, 0, 6, 2, 8, 6, 2, 0, 8, 9, 9, 8, 6, 2, 8, 0, 3,
            4, 8, 2, 5, 3, 4, 2, 1, 1, 7, 0, 6, 7,
        ];
        let generated: Vec<u32> = generator().take(100).map(|d| d.to_u32().unwrap()).collect();
        assert_eq!(generated, expected);
    }
}
