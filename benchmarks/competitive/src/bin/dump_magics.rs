fn main() {
    let t = gigachess::attacks::tables();
    println!("pub const ROOK_MAGICS: [u64; 64] = {:?};", t.rook.magic);
    println!("pub const BISHOP_MAGICS: [u64; 64] = {:?};", t.bishop.magic);
}
