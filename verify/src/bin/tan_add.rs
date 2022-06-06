//! tan の加法定理を使う。

use anyhow::{ensure, Context};
use itertools::assert_equal;
use num_traits::{One as _, Signed as _, Zero as _};

const N: usize = 16; // 行数/列数

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/atan.txt"));

type Matrix = Vec<Vec<(i64, i64)>>;

fn main() -> anyhow::Result<()> {
    let mat = parse()?;

    verify_numbers(&mat);

    // 全ての行についてチェック。
    for r in 0..N {
        let ps = (0..N).map(|c| (r, c));
        verify_line(&mat, ps);
    }

    // 全ての列についてチェック。
    for c in 0..N {
        let ps = (0..N).map(|r| (r, c));
        verify_line(&mat, ps);
    }

    // 左上 -> 右下のラインをチェック。
    {
        let ps = (0..N).map(|i| (i, i));
        verify_line(&mat, ps);
    }

    // 右上 -> 左下のラインをチェック。
    {
        let ps = (0..N).map(|i| (i, N - 1 - i));
        verify_line(&mat, ps);
    }

    Ok(())
}

fn parse() -> anyhow::Result<Matrix> {
    let mut mat = vec![];

    for line in INPUT.lines() {
        let mut row = vec![];

        for token in line.split_ascii_whitespace() {
            let mut it = token.split('/');
            let numer: i64 = it.next().context("numer not found")?.parse()?;
            let denom: i64 = it.next().context("denom not found")?.parse()?;
            row.push((numer, denom));
        }
        ensure!(row.len() == N);

        mat.push(row);
    }
    ensure!(mat.len() == N);

    Ok(mat)
}

/// 以下の条件が満たされているか確認する:
///
///   * 全て既約真分数である
///   * 1 から 512 までの数値が 1 回ずつ現れる
fn verify_numbers(mat: &Matrix) {
    let mut xs = vec![];

    for &(numer, denom) in mat.iter().flatten() {
        assert_eq!(num_integer::gcd(numer, denom), 1);
        assert!(numer < denom);

        xs.push(numer);
        xs.push(denom);
    }

    xs.sort_unstable();
    assert_equal(xs, 1..=512);
}

// Ratio<i64> でも収まるっぽいけど一応。
type Rational = num_rational::BigRational;

/// atan の総和 (n * pi/2 + atan(x))
///
/// 不変条件: 0 <= atan(x) < pi/2
#[derive(Debug)]
struct AtanSum {
    n: u32,
    x: Rational,
}

impl AtanSum {
    /// 総和の初期値 0 を返す。
    fn new() -> Self {
        Self::default()
    }

    /// 総和が 2*pi かどうかを返す。
    fn is_tau(&self) -> bool {
        self.n == 4 && self.x.is_zero()
    }

    /// 総和に atan(y) を加える。
    fn add(&mut self, y: Rational) {
        // A = atan(x), B = atan(y) とおく。条件より 0 <= A < pi/2, 0 <= B < pi/2 である。
        //
        // x*y == 1 ならば、A+B == pi/2 なので、n を 1 増やし、x = 0 とする。
        //
        // x*y != 1 ならば、tan(A+B) = (x+y) / (1-x*y) の符号を調べる。
        // これが非負なら 0 <= A+B < pi/2 なので、単に x を更新する。
        // これが負なら pi/2 < A+B < pi なので、A+B == pi/2 + atan(A+B-pi/2) より、
        // n を 1 増やし、tan(A+B-pi/2) = -1/tan(A+B) により x を更新する。

        let prod = &self.x * &y;

        if prod == Rational::one() {
            self.n += 1;
            self.x = Rational::zero();
            return;
        }

        let tan_add = (&self.x + &y) / (Rational::one() - prod);

        if tan_add.is_negative() {
            self.n += 1;
            self.x = -tan_add.recip();
        } else {
            self.x = tan_add;
        }
    }
}

impl Default for AtanSum {
    fn default() -> Self {
        Self {
            n: 0,
            x: Rational::zero(),
        }
    }
}

/// 指定したラインの atan の総和が 2*pi になっているか確認する。
fn verify_line(mat: &Matrix, ps: impl IntoIterator<Item = (usize, usize)>) {
    let mut sum = AtanSum::new();

    let ys = ps.into_iter().map(|(r, c)| {
        let (numer, denom) = mat[r][c];
        Rational::new(numer.into(), denom.into())
    });

    for y in ys {
        sum.add(y);
    }

    assert!(sum.is_tau());
}
