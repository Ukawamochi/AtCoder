# バイナリヒープの使い方
- 標準ライブラリ(std::collections::BinaryHeap)は、ルートに最大値が来る最大ヒープ
- メモリの「ヒープとスタック」とは関係ない

## 最大ヒープのサンプルコード
```rust
use std::collections::BinaryHeap;

let mut heap: BinaryHeap<i64> = BinaryHeap::new();

//追加（push）
heap.push(5);

// 最大値を見る。
let max = *heap.peek().unwrap(); // Option<&T>

// 最大値を取り出して削除する。
let top = heap.pop().unwrap();// Option<T>

//要素数
println!("{}", heap.len());  // 3

// 空か判定
println!("{}", heap.is_empty());  // false

```

## 最小ヒープにしたいとき
デフォルトでは最大ヒープのみ存在する。値をstd::cmp::Reverseでラップする

```rust
use std::colections::BinaryHeap;
use std::cmp::Reverse;

let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();

heap.push(Reverse(5));
heap.push(Reverse(6));
heap.push(Reverse(9));

// 取り出すときは Reverse(値) という形で出てくるので、
// パターンマッチで中身を取り出す
let Reverse(min) = heap.pop().unwrap();//popはOption型を返す
println!("{}", min);
```

## 補足

### パターンマッチ

```rust
let Some(x) = Some(42);     // x = 42
let Reverse(y) = Reverse(7); // y = 7
let (a, b) = (1, 2);         // a = 1, b = 2
```

### ResultとOption
- Rersult => Ok(T)、Err(E)
- Option => Some(T)、None

### Unwrap()
Ok(T)、Some(T) => T
Err(E)、None => パニック