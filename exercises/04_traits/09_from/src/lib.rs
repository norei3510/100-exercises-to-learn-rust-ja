// TODO: `example`をコンパイルできるように、`WrappingU32`型に`From`トレイトを実装してください。

pub struct WrappingU32 {
    value: u32,
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
