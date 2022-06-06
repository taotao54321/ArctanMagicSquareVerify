//! 複素数とみなして積をとり、偏角が 0 かどうか調べる。

use anyhow::{ensure, Context};
use num_bigint::BigInt;
use num_traits::{One as _, Signed as _, Zero as _};

const N: usize = 16; // 行数/列数

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/atan.txt"));

type Complex = num_complex::Complex<num_bigint::BigInt>;

type Matrix = Vec<Vec<Complex>>;

fn main() -> anyhow::Result<()> {
    let mat = parse()?;

    // NOTE: verify_numbers() は省略。

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
            let im: BigInt = it.next().context("numer not found")?.parse()?;
            let re: BigInt = it.next().context("denom not found")?.parse()?;
            row.push(Complex::new(re, im));
        }
        ensure!(row.len() == N);

        mat.push(row);
    }
    ensure!(mat.len() == N);

    Ok(mat)
}

/// 指定したラインの atan の総和が 2*pi になっているか確認する。
fn verify_line(mat: &Matrix, ps: impl IntoIterator<Item = (usize, usize)>) {
    // 複素数とみなして総積をとった結果が正の実数であればよい。
    // ただし、ちょうど 1 周していることを確認する必要がある。

    let mut prod = Complex::one();

    for x in ps.into_iter().map(|(r, c)| &mat[r][c]) {
        // 掛ける前が第3象限で、掛けた後が第1象限ならば、1 周を超えているのでNG。
        let quad3_before = prod.re.is_positive() && prod.im.is_negative();
        prod *= x;
        let quad1_after = prod.re.is_positive() && prod.im.is_positive();
        assert!(!(quad3_before && quad1_after));
    }

    assert!(prod.re.is_positive());
    assert!(prod.im.is_zero());
}
