pub fn example() {
    // `std::mem::size_of`経由でstr（またはその他のDST）のサイズを取得しようとすると、コンパイルエラーが発生します。
    //
    // TODO: 次の行をコメントアウトして、次の演習に進んでください。
    std::mem::size_of::<str>();
}
