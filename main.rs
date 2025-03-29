#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Bytes;
// use std::collections::HashMap;
use rustc_hash::FxHashMap; // https://docs.rs/rustc-hash/latest/rustc_hash/index.html

type us = usize;
type is = isize;

// #[proconio::fastout]
fn main() {
    input! {
        s: String,

        // f: f32,
        // n: i32,

        // n: us,
        // arr: [i32; n],

        // n: us,
        // s: [char; n],
        // t: [char; n],

        // n: us,
        // s: Chars,
        // t: Chars,
    }
    debug!{
        p!("Hello world");
    }
    yn(true);
}






// 爆速 Yes / No

/// 標準出力に "Yes" を出力します。  
#[inline(always)]
pub fn y() {
    println!("Yes");
}

/// 標準出力に "No" を出力します。  
#[inline(always)]
pub fn n() {
    println!("No");
}

/// 条件 `condition` が真の場合、"Yes" を出力し、偽の場合 "No" を出力します。  
///
/// ## Examples
///
/// ```
/// yn(10 > 5); // "Yes" を出力
/// yn(3 > 5);  // "No" を出力
/// ```
#[inline(always)]
pub fn yn(condition: bool) {
    if condition { y(); } else { n(); }
}

// 楽してprint

/// 標準出力に式 `c` の値を文字列として出力します。  
/// 複数の式を渡すと、スペース区切りで出力します。  
///
/// ## Examples
///
/// ```
/// p!("Hello"); // "Hello" を出力
/// p!(1, 2, 3); // "1 2 3" を出力
/// ```
#[macro_export]
macro_rules! p {
    ($c:expr) => {
        println!("{}", $c);
    };
    ($($c:expr),*) => {
        $(print!("{} ", $c);)*
        println!();
    };
}

/// 標準出力に式 `c` の値をデバッグ形式で出力します。  
/// 複数の式を渡すと、スペース区切りで出力します。  
///
/// ## Examples
///
/// ```
/// pd!(vec![1, 2, 3]); // "[1, 2, 3]" を出力
/// pd!("world", 42);  // "\"world\" 42" を出力
/// ```
#[macro_export]
macro_rules! pd {
    ($c:expr) => {
        println!("{:?}", $c);
    };
    ($($c:expr),*) => {
        $(print!("{:?} ", $c);)*
        println!();
    };
}

// debug時のみ有効なやつ

/// デバッグビルド時のみ、指定されたコード `code` を実行します。  
/// リリースビルドでは無視されます。  
///
/// ## Examples
///
/// ```
/// debug! {
///     println!("This is a debug message.");
/// }
/// ```
#[macro_export]
macro_rules! debug {
    ($($code:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $($code)*
        }
    };
}

/// デバッグビルド時のみ、式 `c` の値を文字列として出力します。  
/// 複数の式を渡すと、スペース区切りで出力します。  
///
/// ## Examples
///
/// ```
/// dbgp!("Debug message");
/// dbgp!(10, "test");
/// ```
#[macro_export]
macro_rules! dbgp {
    ($c:expr) => {
        #[cfg(debug_assertions)]
        {
            println!("{}", $c);
        }
    };
    ($($c:expr),*) => {
        #[cfg(debug_assertions)]
        {
            $(print!("{} ", $c);)*
            println!();
        }
    };
}

/// デバッグビルド時のみ、式 `c` の値をデバッグ形式で出力します。  
/// 複数の式を渡すと、スペース区切りで出力します。  
///
/// ## Examples
///
/// ```
/// dbgpd!(vec![1, 2, 3]);
/// dbgpd!("debug", 123);
/// ```
#[macro_export]
macro_rules! dbgpd {
    ($c:expr) => {
        #[cfg(debug_assertions)]
        {
            println!("{:?}", $c);
        }
    };
    ($($c:expr),*) => {
        #[cfg(debug_assertions)]
        {
            $(print!("{:?} ", $c);)*
            println!();
        }
    };
}

/// デバッグビルド時のみ、変数名と値を文字列として出力します。  
///
/// ## Examples
///
/// ```
/// let x = 10;
/// dbgpi!(x); // "x: 10" を出力
/// ```
#[macro_export]
macro_rules! dbgpi {
    ($var:ident) => {
        #[cfg(debug_assertions)]
        {
            println!("{}: {}", stringify!($var), $var);
        }
    };
}

/// デバッグビルド時のみ、変数名と値をデバッグ形式で出力します。  
///
/// ## Examples
///
/// ```
/// let vec = vec![1, 2, 3];
/// dbgpid!(vec); // "vec: [1, 2, 3]" を出力
/// ```
#[macro_export]
macro_rules! dbgpid {
    ($var:ident) => {
        #[cfg(debug_assertions)]
        {
            println!("{}: {:?}", stringify!($var), $var);
        }
    };
}

// その他

#[macro_export]
macro_rules! rep {
    ($i:ident, $max:expr, $($code:tt)*) => {
        for $i in 0..$max {
            $($code)*
        }
    };
}

#[macro_export]
macro_rules! search_linear {
    // 最大回数指定なしの場合
    ($n:expr, $s:expr, $f:expr) => {{
        let mut i: i32 = 0;
        loop {
            // 指定された関数を計算値とともに呼び出す
            let result = $f($n + i * $s);
            // 条件が満たされた場合、結果と反復回数を返してループを抜ける
            if result.0 {
                break (result, i);
            }
            i += 1;
        }
    }};
    // 最大反復回数を指定する場合のブランチ
    ($n:expr, $s:expr, $max:expr, $f:expr) => {{
        let mut i: i32 = 0;
        // 成功した場合はSome、失敗した場合はNoneでラップする
        let mut found = None;
        // 現在の反復回数が最大値未満の間ループを実行
        while i < $max {
            let result = $f($n + i * $s);
            if result.0 {
                found = Some((result, i));
                break;
            }
            i += 1;
        }
        // Optionでラップした結果を返す
        found
    }};
}

#[macro_export]
macro_rules! nth { // 整数xのk桁目
    ($x:expr, $k:expr, $base:expr) => {{
        ($x / ($base as i32).pow((($k - 1) as u32))) % $base
    }};
    ($x:expr, $k:expr) => {{ // base省略したら10進数
        ($x / 10_i32.pow((($k - 1) as u32))) % 10
    }};
}

#[macro_export]
macro_rules! nth2 { // 2進数xのk桁目
    ($x:expr, $k:expr) => {
        ($x >> ($k - 1)) & 1
    };
}

#[macro_export]
macro_rules! collect_adjacent_pairs { // Vecの隣接する2項のタプルのVecを作成
    ($slice:expr) => {
        $slice.windows(2).map(|w| (w[0], w[1])).collect::<Vec<_>>()
    }
}

// タプルの最初の要素 (number) を比較してベクタをソート
macro_rules! sort {
    ($vec:expr) => {
        $vec.sort_by(|a, b| a.0.cmp(&b.0));
    };
}

// Vecのスライスの各要素に関数fを適用してfの値が最も大きい時の値を種類
#[macro_export]
macro_rules! vec_fn_max {
    ($vec:expr, $f:expr) => {
        // Initialize the maximum pair and maximum value as None.
        let mut max_pair = None;
        let mut max_value = None;
        // Iterate over the vector with an index.
        for (i, x) in $vec.iter().enumerate() {
            let value = $f(x, i);
            match max_value {
                // If no maximum is set yet, assign the current value.
                None => {
                    max_value = Some(value);
                    max_pair = Some((x, i, value));
                },
                // Update the maximum if the current value is greater.
                Some(ref current_max) if value > *current_max => {
                    max_value = Some(value);
                    max_pair = Some((x, i, value));
                },
                _ => {}
            }
        }
        max_pair
    };
}

/*

vec.sort_by_key(|x| x.0)
vec.binary_search_by_key( target, |x| x.0)

sort_unstable
sort_unstable_by_key
sort_unstable_by

*/