// SPDX-License-Identifier: MIT
// Standalone comparative executable; never a production dependency.
use chess::Board as J;
use cozy_chess::Board as C;
use ferrum::Board as F;
use gigachess::{fen::parse_fen, san, Board};
use shakmaty::zobrist::Zobrist64;
use shakmaty::Chess as S;
use shakmaty::{CastlingMode, EnPassantMode, Position};
use std::{hint::black_box as bb, time::Instant};
use ultra::Position as U;

const POS: [(&str, &str, u64); 6] = [
    (
        "start",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        197281,
    ),
    (
        "kiwipete",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        4085603,
    ),
    (
        "endgame",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        43238,
    ),
    (
        "promotion",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        422333,
    ),
    (
        "tactical",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        2103487,
    ),
    (
        "middlegame",
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        3894594,
    ),
];
fn shak(f: &str) -> S {
    shakmaty::fen::Fen::from_ascii(f.as_bytes())
        .unwrap()
        .into_position(CastlingMode::Standard)
        .unwrap()
}
fn cozy_count(b: &C) -> usize {
    let mut n = 0;
    b.generate_moves(|ms| {
        n += ms.len() as usize;
        false
    });
    n
}
fn cozy_list(b: &C) -> arrayvec::ArrayVec<cozy_chess::Move, 256> {
    let mut v = arrayvec::ArrayVec::new();
    b.generate_moves(|ms| {
        v.extend(ms);
        false
    });
    v
}
fn ultra_list(b: &U) -> ultra::MoveList {
    let mut v = ultra::MoveList::new();
    ultra::generate_legal_moves(b, &mut v);
    v
}
fn jordan_list(b: &J) -> arrayvec::ArrayVec<chess::ChessMove, 256> {
    chess::MoveGen::new_legal(b).collect()
}
fn pg(b: &Board, d: u32, bulk: bool) -> u64 {
    if d == 0 {
        if !bulk { bb(b); }
        return 1;
    }
    if bulk {
        return b.perft(d);
    }
    #[cfg(feature="exp-full-visitor")]
    {return b.visit_perft_leaves(d,&mut |leaf|{bb(leaf);});}
    #[allow(unreachable_code)]
    let mut n = 0;
    #[cfg(feature = "exp-copy-perft")]
    for m in b.legal_moves() {
        let mut c = *b;
        c.make_move_perft(m);
        n += pg(&c, d - 1, false);
    }
    #[cfg(not(feature = "exp-copy-perft"))]
    {
        let mut c = *b;
        for m in b.legal_moves() {
            let u = c.make_move_perft(m);
            n += pg(&c, d - 1, false);
            c.unmake_move_perft(m, u);
        }
    }
    n
}
fn pc(b: &C, d: u32, bulk: bool) -> u64 {
    if d == 0 {
        if !bulk { bb(b); }
        return 1;
    }
    if d == 1 && bulk {
        return cozy_count(b) as u64;
    }
    let mut n = 0;
    b.generate_moves(|ms| {
        for m in ms {
            let mut c = b.clone();
            c.play_unchecked(m);
            n += pc(&c, d - 1, bulk);
        }
        false
    });
    n
}
fn ps(b: &S, d: u32, bulk: bool) -> u64 {
    if d == 0 {
        if !bulk { bb(b); }
        return 1;
    }
    let ms = b.legal_moves();
    if d == 1 && bulk {
        return ms.len() as u64;
    }
    ms.into_iter()
        .map(|m| {
            let mut c = b.clone();
            c.play_unchecked(m);
            ps(&c, d - 1, bulk)
        })
        .sum()
}
fn pj(b: &J, d: u32, bulk: bool) -> u64 {
    if d == 0 {
        if !bulk { bb(b); }
        return 1;
    }
    let ms = chess::MoveGen::new_legal(b);
    if d == 1 && bulk {
        return ms.len() as u64;
    }
    ms.map(|m| pj(&b.make_move_new(m), d - 1, bulk)).sum()
}
fn pu(b: &mut U, d: u32, bulk: bool) -> u64 {
    if d == 0 {
        if !bulk { bb(&*b); }
        return 1;
    }
    if bulk {
        return ultra::perft(b, d);
    }
    let mut n = 0;
    for &m in ultra_list(b).as_slice() {
        b.make_move_perft(m);
        n += pu(b, d - 1, false);
        b.unmake_move_perft(m);
    }
    n
}
fn pf(b: &F, d: u32, bulk: bool) -> u64 {
    if d == 0 {
        if !bulk { bb(b); }
        return 1;
    }
    if bulk {
        return ferrum::testing::perft(b, d);
    }
    b.generate_moves()
        .iter()
        .map(|m| pf(&b.make_move_new(m), d - 1, false))
        .sum()
}
fn bench<T>(name: &str, case: &str, lib: &str, units: u64, mut f: impl FnMut() -> T) {
    if let Ok(filter)=std::env::var("BENCH_FILTER"){
        if !filter.split(',').any(|x|x==name){return;}
    }
    if let Ok(filter)=std::env::var("BENCH_LIB"){
        if !filter.split(',').any(|x|x==lib){return;}
    }
    let target = std::env::var("BENCH_MS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(40);
    let t = Instant::now();
    let mut n = 0u64;
    while t.elapsed().as_millis() < 5 {
        bb(f());
        n += 1;
    }
    let iters = ((n as f64 * target as f64 / t.elapsed().as_secs_f64() / 1000.) as u64)
        .clamp(1, 10_000_000);
    let mut samples = Vec::new();
    for _ in 0..5 {
        let t = Instant::now();
        for _ in 0..iters {
            bb(f());
        }
        samples.push(t.elapsed().as_secs_f64() * 1e9 / iters as f64);
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "{name}\t{case}\t{lib}\t{units}\t{iters}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}",
        samples[2], samples[0], samples[4], samples[1], samples[3]
    );
}
macro_rules! row {($op:expr,$case:expr,$units:expr,$($lib:literal=>$f:expr),+ $(,)?) => {$(bench($op,$case,$lib,$units,$f);)+};}
// Distinct input/state contracts, never substituted for the original
// replay_trusted_native rows. Native Shakmaty moves contain role metadata.
#[cfg(feature="exp-prepared-move")]
fn prepared_replay_benchmark(){
    use gigachess::{Move,PreparedMove};
    let mut games:Vec<(Vec<Move>,Vec<PreparedMove>,Vec<shakmaty::Move>)>=Vec::new();
    for line in include_str!("../data/lichess-2013-01-first300.uci").lines(){
        let mut g=Board::startpos();let mut p=g;let mut s=S::default();
        let(mut words,mut prepared,mut expanded)=(Vec::new(),Vec::new(),Vec::new());
        for token in line.split_ascii_whitespace(){
            let m=g.legal_moves().into_iter().find(|&m|gu(&g,m)==token).unwrap();
            let a=g.prepare_move_unchecked(m);
            let sm=token.parse::<shakmaty::uci::UciMove>().unwrap().to_move(&s).unwrap();
            g.make_move_unchecked(m);p.make_prepared_unchecked(a);s.play_unchecked(sm);
            assert_eq!(g,p);
            assert_eq!(g.zobrist(),s.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal).0);
            words.push(m);prepared.push(a);expanded.push(sm);
        }
        games.push((words,prepared,expanded));
    }
    let plies=games.iter().map(|g|g.0.len() as u64).sum();
    assert_eq!(plies,18954);
    println!("operation\tcase\tlibrary\tunits\titers\tmedian_ns\tmin_ns\tmax_ns\tp25_ns\tp75_ns");
    row!("replay_metadata","lichess300",plies,
        "giga_wire"=>||{for x in &games{let mut b=Board::startpos();for &m in &x.0{b.make_move_unchecked(m);}bb(b);}},
        "giga_prepared"=>||{for x in &games{let mut b=Board::startpos();for &m in &x.1{b.make_prepared_unchecked(m);}bb(b);}},
        "giga_prepare_included"=>||{for x in &games{let mut b=Board::startpos();for &m in &x.0{let p=b.prepare_move_unchecked(m);b.make_prepared_unchecked(p);}bb(b);}},
        "shakmaty_native"=>||{for x in &games{let mut b=S::default();for &m in &x.2{b.play_unchecked(m);}bb(b);}});
    row!("replay_polyglot_each_ply","lichess300",plies,
        "giga_hashed"=>||{for x in &games{let mut b=Board::startpos();for &m in &x.0{b.make_move_hashed(m);bb(b.zobrist());}bb(b);}},
        "shakmaty_hash"=>||{for x in &games{let mut b=S::default();for &m in &x.2{b.play_unchecked(m);bb(b.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal));}bb(b);}});
    row!("replay_polyglot_and_checkers_each_ply","lichess300",plies,
        "giga_full"=>||{for x in &games{let mut b=Board::startpos();for &m in &x.0{b.make_move_unchecked(m);bb((b.zobrist(),b.checkers_bb()));}bb(b);}},
        "shakmaty_full"=>||{for x in &games{let mut b=S::default();for &m in &x.2{b.play_unchecked(m);bb((b.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal),b.checkers()));}bb(b);}});
}

#[cfg(feature="exp-batch-replay")]
fn batch_replay_benchmark(){
    let mut games=Vec::new();
    for line in include_str!("../data/lichess-2013-01-first300.uci").lines(){
        let mut b=Board::startpos();let mut s=S::default();
        let mut words=Vec::new();let mut expanded=Vec::new();
        for token in line.split_ascii_whitespace(){
            let m=b.legal_moves().into_iter().find(|&m|gu(&b,m)==token).unwrap();
            let sm=token.parse::<shakmaty::uci::UciMove>().unwrap().to_move(&s).unwrap();
            b.make_move_unchecked(m);s.play_unchecked(sm);words.push(m);expanded.push(sm);
        }
        assert_eq!(Board::startpos().replayed_unchecked(&words),b);
        assert_eq!(Board::startpos().replayed_hashed_unchecked(&words),b);
        assert_eq!(b.zobrist(),s.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal).0);
        games.push((words,expanded));
    }
    let plies=games.iter().map(|g|g.0.len() as u64).sum();
    assert_eq!(plies,18954);
    println!("operation\tcase\tlibrary\tunits\titers\tmedian_ns\tmin_ns\tmax_ns\tp25_ns\tp75_ns");
    row!("replay_final_exact","lichess300",plies,
        "giga_full_loop"=>||{for x in &games{let mut b=Board::startpos();for &m in &x.0{b.make_move_unchecked(m);}bb((b.zobrist(),b.checkers_bb()));bb(b);}},
        "giga_batch_recompute"=>||{for x in &games{let b=Board::startpos().replayed_unchecked(&x.0);bb((b.zobrist(),b.checkers_bb()));bb(b);}},
        "giga_batch_hashed"=>||{for x in &games{let b=Board::startpos().replayed_hashed_unchecked(&x.0);bb((b.zobrist(),b.checkers_bb()));bb(b);}},
        "shakmaty_final_exact"=>||{for x in &games{let mut b=S::default();for &m in &x.1{b.play_unchecked(m);}bb((b.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal),b.checkers()));bb(b);}});
}

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        #[cfg(feature="exp-batch-replay")]
        if arg=="--batch-replay"{batch_replay_benchmark();return;}
        #[cfg(not(feature="exp-batch-replay"))]
        if arg=="--batch-replay"{panic!("--batch-replay requires exp-batch-replay");}


        #[cfg(feature="exp-prepared-move")]
        if arg=="--prepared-replay"{prepared_replay_benchmark();return;}

        if let Some(lib) = arg.strip_prefix("--cold-") {
            let t = Instant::now();
            let n=match lib {
                "giga"=>bb(bb(Board::startpos()).legal_moves()).len(),
                "shakmaty"=>bb(bb(S::default()).legal_moves()).len(),
                "cozy"=>bb(cozy_list(&bb(C::default()))).len(),
                "chess"=>bb(jordan_list(&bb(J::default()))).len(),
                "ultra"=>bb(ultra_list(&bb(U::startpos()))).len(),
                "ferrum"=>bb(bb(F::startpos()).generate_moves()).len(),
                _=>panic!("unknown cold library"),
            };
            assert_eq!(n,20);
            println!("{}",t.elapsed().as_nanos());
            return;
        }
        if arg == "--sizes" {
            println!("giga={} shakmaty={} cozy={} chess={} ultra={} ferrum={}",std::mem::size_of::<Board>(),std::mem::size_of::<S>(),std::mem::size_of::<C>(),std::mem::size_of::<J>(),std::mem::size_of::<U>(),std::mem::size_of::<F>());
            return;
        }
    }
    // Corpus runs after fixed cases, with its own parity gate.
    println!("operation\tcase\tlibrary\tunits_per_call\titerations\tmedian_ns\tmin_ns\tmax_ns\tp25_ns\tp75_ns");
    for (name, fen, expected) in POS {
        let g = parse_fen(fen).unwrap();
        let s = shak(fen);
        let c = C::from_fen(fen, false).unwrap();
        let j = fen.parse::<J>().unwrap();
        let mut u = ultra::parse_fen(fen).unwrap();
        let f = F::from_fen(fen).unwrap();
        let count = g.count_legal_moves() as usize;
        assert_eq!(
            [
                s.legal_moves().len(),
                cozy_count(&c),
                chess::MoveGen::new_legal(&j).len(),
                ultra::count_legal_moves(&u) as usize,
                f.generate_moves_count() as usize
            ],
            [count; 5],
            "{name} count"
        );
        let totals = [
            pg(&g, 4, true),
            ps(&s, 4, true),
            pc(&c, 4, true),
            pj(&j, 4, true),
            pu(&mut u, 4, true),
            pf(&f, 4, true),
        ];
        assert_eq!(totals, [expected; 6], "{name} perft4");
        let full = pg(&g, 3, false);
        assert_eq!(
            [
                ps(&s, 3, false),
                pc(&c, 3, false),
                pj(&j, 3, false),
                pu(&mut u, 3, false),
                pf(&f, 3, false)
            ],
            [full; 5],
            "{name} full perft3"
        );
        row!("count",name,count as u64,"giga"=>||bb(&g).count_legal_moves(),"shakmaty"=>||bb(&s).legal_moves().len(),"cozy"=>||cozy_count(bb(&c)),"chess"=>||chess::MoveGen::new_legal(bb(&j)).len(),"ultra"=>||ultra::count_legal_moves(bb(&u)),"ferrum"=>||bb(&f).generate_moves_count());
        row!("materialize",name,count as u64,"giga"=>||bb(&g).legal_moves(),"shakmaty"=>||bb(&s).legal_moves(),"cozy"=>||cozy_list(bb(&c)),"chess"=>||jordan_list(bb(&j)),"ultra"=>||ultra_list(bb(&u)),"ferrum"=>||bb(&f).generate_moves());
        row!("perft_bulk_d4",name,expected,"giga"=>||pg(bb(&g),4,true),"shakmaty"=>||ps(bb(&s),4,true),"cozy"=>||pc(bb(&c),4,true),"chess"=>||pj(bb(&j),4,true),"ultra"=>||pu(bb(&mut u),4,true),"ferrum"=>||pf(bb(&f),4,true));
        row!("perft_full_d3",name,full,"giga"=>||pg(bb(&g),3,false),"shakmaty"=>||ps(bb(&s),3,false),"cozy"=>||pc(bb(&c),3,false),"chess"=>||pj(bb(&j),3,false),"ultra"=>||pu(bb(&mut u),3,false),"ferrum"=>||pf(bb(&f),3,false));
        row!("fen_parse",name,1,"giga"=>||parse_fen(bb(fen)).unwrap(),"shakmaty"=>||shak(bb(fen)),"cozy"=>||C::from_fen(bb(fen),false).unwrap(),"chess"=>||bb(fen).parse::<J>().unwrap(),"ultra"=>||ultra::parse_fen(bb(fen)).unwrap(),"ferrum"=>||F::from_fen(bb(fen)).unwrap());
        row!("fen_write_native_ep",name,1,"giga"=>||bb(&g).to_fen(),"shakmaty"=>||shakmaty::fen::Fen::from_position(bb(&s),EnPassantMode::Always).to_string(),"cozy"=>||bb(&c).to_string(),"chess"=>||bb(&j).to_string(),"ultra"=>||ultra::write_fen(bb(&u)),"ferrum"=>||bb(&f).to_fen());
        row!("snapshot",name,1,"giga"=>||*bb(&g),"shakmaty"=>||bb(&s).clone(),"cozy"=>||bb(&c).clone(),"chess"=>||*bb(&j),"ultra"=>||bb(&u).clone(),"ferrum"=>||bb(&f).clone());
        // Use identical SAN for all legal moves, normalized to SAN body for conversion.
        let tokens: Vec<String> = g
            .legal_moves()
            .iter()
            .map(|m| san::move_to_san(&g, *m).unwrap().to_string())
            .collect();
        let gm: Vec<_> = tokens
            .iter()
            .map(|t| san::san_to_move(&g, t).unwrap())
            .collect();
        let sm: Vec<_> = tokens
            .iter()
            .map(|t| {
                t.parse::<shakmaty::san::SanPlus>()
                    .unwrap()
                    .san
                    .to_move(&s)
                    .unwrap()
            })
            .collect();
        let um: Vec<_> = tokens
            .iter()
            .map(|t| ultra::san::san_to_move(&u, t).unwrap())
            .collect();
        row!("san_parse_all",name,count as u64,"giga"=>||{for t in &tokens{bb(san::san_to_move(bb(&g),bb(t)).unwrap());}},"shakmaty"=>||{for t in &tokens{bb(bb(t).parse::<shakmaty::san::SanPlus>().unwrap().san.to_move(bb(&s)).unwrap());}},"ultra"=>||{for t in &tokens{bb(ultra::san::san_to_move(bb(&u),bb(t)).unwrap());}});
        for (i, t) in tokens.iter().enumerate() {
            assert_eq!(ultra::san::move_to_san(&mut u, um[i]), *t);
            assert_eq!(
                shakmaty::san::SanPlus::from_move(s.clone(), sm[i]).to_string(),
                *t
            );
        }
        row!("san_render_native",name,count as u64,"giga"=>||{for &m in &gm{bb(san::move_to_san(bb(&g),bb(m)).unwrap());}},"shakmaty"=>||{for &m in &sm{bb(shakmaty::san::SanPlus::from_move(bb(&s).clone(),bb(m)));}},"ultra"=>||{for &m in &um{bb(ultra::san::move_to_san(bb(&mut u),bb(m)));}});
        let cm = cozy_list(&c);
        let jm = jordan_list(&j);
        let fm = f.generate_moves();
        row!("apply_children_full",name,count as u64,"giga"=>||{for &m in &gm{#[cfg(feature="exp-child-copy")]{bb(bb(&g).child_after_unchecked(bb(m)));}#[cfg(not(feature="exp-child-copy"))]{let mut x=*bb(&g);x.make_move_unchecked(bb(m));bb(x);}}},"shakmaty"=>||{for &m in &sm{let mut x=bb(&s).clone();x.play_unchecked(bb(m));bb(x);}},"cozy"=>||{for &m in &cm{let mut x=bb(&c).clone();x.play_unchecked(bb(m));bb(x);}},"chess"=>||{for &m in &jm{bb(bb(&j).make_move_new(bb(m)));}},"ultra"=>||{for &m in &um{let mut x=bb(&u).clone();x.make_move(bb(m));bb(x);}},"ferrum"=>||{for m in fm.iter(){bb(bb(&f).make_move_new(bb(m)));}});
        row!("make_unmake_full",name,count as u64,"giga"=>||{let mut x=g;for &m in &gm{let undo=x.make_move_unchecked(bb(m));bb(&x);x.unmake_move(m,undo);bb(&x);}},"ultra"=>||{for &m in &um{u.make_move(bb(m));bb(&u);u.unmake_move(m);bb(&u);}});
        row!("perft_internal_ab",name,expected,"giga_default"=>||g.perft(4),"giga_reuse"=>||g.perft_reuse(4),"giga_visitor"=>||g.perft_visitor(4));
    }
    row!("startpos","start",1,"giga"=>Board::startpos,"shakmaty"=>S::default,"cozy"=>C::default,"chess"=>J::default,"ultra"=>U::startpos,"ferrum"=>F::startpos);
    row!("startpos_internal_ab","start",1,"giga_template"=>Board::startpos,"giga_legacy_parse"=>||parse_fen(bb(POS[0].1)).unwrap());
    chess960();
    corpus("seeded", None);
    corpus(
        "lichess300",
        Some(include_str!("../data/lichess-2013-01-first300.uci")),
    );
}
fn castle_uci(mut x: String, king: bool) -> String {
    if king {
        x = match x.as_str() {
            "e1h1" => "e1g1",
            "e1a1" => "e1c1",
            "e8h8" => "e8g8",
            "e8a8" => "e8c8",
            _ => return x,
        }
        .into();
    }
    x
}
fn gu(b: &Board, m: gigachess::Move) -> String {
    #[cfg(feature="exp-uci-direct")]
    let text=m.to_uci_string();
    #[cfg(not(feature="exp-uci-direct"))]
    let text=m.to_string();
    castle_uci(text, m.from() == b.king_square(b.turn()))
}
fn cu(b: &C, m: cozy_chess::Move) -> String {
    castle_uci(m.to_string(), m.from == b.king(b.side_to_move()))
}
fn su(m: shakmaty::Move) -> String {
    shakmaty::uci::UciMove::from_move(m, CastlingMode::Standard).to_string()
}
fn sorted<T>(ms: impl IntoIterator<Item = T>, f: impl Fn(T) -> String) -> Vec<String> {
    let mut v: Vec<_> = ms.into_iter().map(f).collect();
    v.sort_unstable();
    v
}
fn corpus(label: &str, real: Option<&str>) {
    let mut gb = Vec::new();
    let mut sb = Vec::new();
    let mut cb = Vec::new();
    let mut jb = Vec::new();
    let mut ub = Vec::new();
    let mut fb = Vec::new();
    let mut games = Vec::new();
    let mut rng = 0x83a765127acb3489u64;
    let inputs: Vec<Vec<&str>> = match real {
        Some(s) => s
            .lines()
            .map(|l| l.split_ascii_whitespace().collect())
            .collect(),
        None => vec![vec![""; 64]; 32],
    };
    for requested_moves in inputs {
        let (mut g, mut s, mut c, mut j, mut u, mut f) = (
            Board::startpos(),
            S::default(),
            C::default(),
            J::default(),
            U::startpos(),
            F::startpos(),
        );
        let (mut gm, mut sm, mut cm, mut jm, mut um, mut fm) = (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        for requested in requested_moves {
            let gl = g.legal_moves();
            let sl = s.legal_moves();
            let cl = cozy_list(&c);
            let jl = jordan_list(&j);
            let ul = ultra_list(&u);
            let fl = f.generate_moves();
            let list = sorted(gl.iter().copied(), |m| gu(&g, m));
            assert_eq!(list, sorted(sl.iter().copied(), su));
            assert_eq!(list, sorted(cl.iter().copied(), |m| cu(&c, m)));
            assert_eq!(list, sorted(jl.iter().copied(), |m| m.to_string()));
            assert_eq!(list, sorted(ul.iter().copied(), |m| m.to_string()));
            assert_eq!(list, sorted(fl.iter(), |m| m.to_uci()));
            gb.push(g);
            sb.push(s.clone());
            cb.push(c.clone());
            jb.push(j);
            ub.push(u.clone());
            fb.push(f.clone());
            if list.is_empty() {
                assert!(requested.is_empty(), "real game continued past terminal");
                break;
            }
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            let t = if requested.is_empty() {
                &list[(rng as usize) % list.len()]
            } else {
                list.iter()
                    .find(|m| m.as_str() == requested)
                    .expect("requested move legal")
            };
            let a = gl.iter().copied().find(|m| gu(&g, *m) == *t).unwrap();
            let b = sl.iter().copied().find(|m| su(*m) == *t).unwrap();
            let d = cl.iter().copied().find(|m| cu(&c, *m) == *t).unwrap();
            let e = jl.iter().copied().find(|m| m.to_string() == *t).unwrap();
            let h = ul.iter().copied().find(|m| m.to_string() == *t).unwrap();
            let i = fl.iter().find(|m| m.to_uci() == *t).unwrap();
            gm.push(a);
            sm.push(b);
            cm.push(d);
            jm.push(e);
            um.push(h);
            fm.push(i);
            g.play(a).unwrap();
            s.play_unchecked(b);
            c.play_unchecked(d);
            j = j.make_move_new(e);
            u.make_move(h);
            f = f.make_move_new(i);
            assert_eq!(
                g.zobrist(),
                s.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal).0
            );
        }
        games.push((gm, sm, cm, jm, um, fm));
    }
    let positions = gb.len() as u64;
    let plies = games.iter().map(|x| x.0.len() as u64).sum();
    eprintln!(
        "Exact normalized move-set parity ({label}): {positions} positions, {plies} replay plies"
    );
    for (g, s) in gb.iter().zip(&sb) {
        assert_eq!(
            g.repetition_key(),
            s.zobrist_hash::<Zobrist64>(EnPassantMode::Legal).0
        );
        assert_eq!(g.zobrist(), g.zobrist_full());
    }
    row!("polyglot_key",label,positions,"giga"=>||{for b in &gb{bb(b.zobrist());}},"shakmaty"=>||{for b in &sb{bb(b.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal));}});
    row!("polyglot_recompute",label,positions,"giga"=>||{for b in &gb{bb(b.zobrist_full());}},"shakmaty"=>||{for b in &sb{bb(b.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal));}});
    row!("repetition_key",label,positions,"giga"=>||{for b in &gb{bb(b.repetition_key());}},"shakmaty"=>||{for b in &sb{bb(b.zobrist_hash::<Zobrist64>(EnPassantMode::Legal));}});
    row!("corpus_count",label,positions,"giga"=>||{for b in &gb{bb(b.count_legal_moves());}},"shakmaty"=>||{for b in &sb{bb(b.legal_moves().len());}},"cozy"=>||{for b in &cb{bb(cozy_count(b));}},"chess"=>||{for b in &jb{bb(chess::MoveGen::new_legal(b).len());}},"ultra"=>||{for b in &ub{bb(ultra::count_legal_moves(b));}},"ferrum"=>||{for b in &fb{bb(b.generate_moves_count());}});
    row!("corpus_materialize",label,positions,"giga"=>||{for b in &gb{bb(b.legal_moves());}},"shakmaty"=>||{for b in &sb{bb(b.legal_moves());}},"cozy"=>||{for b in &cb{bb(cozy_list(b));}},"chess"=>||{for b in &jb{bb(jordan_list(b));}},"ultra"=>||{for b in &ub{bb(ultra_list(b));}},"ferrum"=>||{for b in &fb{bb(b.generate_moves());}});
    #[cfg(feature="exp-uci-sort-key")]
    for b in &gb{assert_eq!(b.legal_moves_uci_sorted(),sorted(b.legal_moves(),|m|gu(b,m)));}
    row!("corpus_uci_sorted",label,positions,"giga"=>||{for b in &gb{
        #[cfg(feature="exp-uci-sort-key")]
        bb(b.legal_moves_uci_sorted());
        #[cfg(not(feature="exp-uci-sort-key"))]
        bb(sorted(b.legal_moves(),|m|gu(b,m)));
    }},"shakmaty"=>||{for b in &sb{bb(sorted(b.legal_moves(),su));}},"cozy"=>||{for b in &cb{bb(sorted(cozy_list(b),|m|cu(b,m)));}},"chess"=>||{for b in &jb{bb(sorted(jordan_list(b),|m|m.to_string()));}},"ultra"=>||{for b in &ub{bb(sorted(ultra_list(b).iter().copied(),|m|m.to_string()));}},"ferrum"=>||{for b in &fb{bb(sorted(b.generate_moves().iter(),|m|m.to_uci()));}});
    row!("replay_trusted_native",label,plies,"giga"=>||{for x in &games{let mut b=Board::startpos();for &m in &x.0{b.make_move_unchecked(m);}bb(b);}},"shakmaty"=>||{for x in &games{let mut b=S::default();for &m in &x.1{b.play_unchecked(m);}bb(b);}},"cozy"=>||{for x in &games{let mut b=C::default();for &m in &x.2{b.play_unchecked(m);}bb(b);}},"chess"=>||{for x in &games{let mut b=J::default();for &m in &x.3{b=b.make_move_new(m);}bb(b);}},"ultra"=>||{for x in &games{let mut b=U::startpos();for &m in &x.4{b.make_move(m);}bb(b);}},"ferrum"=>||{for x in &games{let mut b=F::startpos();for &m in &x.5{b=b.make_move_new(m);}bb(b);}});
    // Supplemental matched-output lanes. Do not replace native replay or
    // fold these into the original strict 522-case score.
    row!("replay_polyglot_each_ply",label,plies,
        "giga_hashed"=>||{for x in &games{let mut b=Board::startpos();for &m in &x.0{b.make_move_hashed(m);bb(b.zobrist());}bb(b);}},
        "shakmaty_hash"=>||{for x in &games{let mut b=S::default();for &m in &x.1{b.play_unchecked(m);bb(b.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal));}bb(b);}});
    row!("replay_polyglot_and_checkers_each_ply",label,plies,
        "giga_full"=>||{for x in &games{let mut b=Board::startpos();for &m in &x.0{b.make_move_unchecked(m);bb((b.zobrist(),b.checkers_bb()));}bb(b);}},
        "shakmaty_full"=>||{for x in &games{let mut b=S::default();for &m in &x.1{b.play_unchecked(m);bb((b.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal),b.checkers()));}bb(b);}});
    row!("replay_checked_native",label,plies,"giga"=>||{for x in &games{let mut b=Board::startpos();for &m in &x.0{b.play(m).unwrap();}bb(b);}},"shakmaty"=>||{for x in &games{let mut b=S::default();for &m in &x.1{b=b.play(m).unwrap();}bb(b);}},"cozy"=>||{for x in &games{let mut b=C::default();for &m in &x.2{b.play(m);}bb(b);}},"chess"=>||{for x in &games{let mut b=J::default();for &m in &x.3{assert!(b.legal(m));b=b.make_move_new(m);}bb(b);}},"ultra"=>||{for x in &games{let mut b=U::startpos();for &m in &x.4{assert!(ultra_list(&b).as_slice().contains(&m));b.make_move(m);}bb(b);}},"ferrum"=>||{for x in &games{let mut b=F::startpos();for &m in &x.5{assert!(b.generate_moves().iter().any(|n|n==m));b=b.make_move_new(m);}bb(b);}});
}

fn chess960() {
    for (name, fen) in [
        (
            "960_start",
            "rnkrqbbn/pppppppp/8/8/8/8/PPPPPPPP/RNKRQBBN w DAda - 0 1",
        ),
        ("960_castle", "r2k3r/8/8/8/8/8/8/R2K3R w HAha - 0 1"),
    ] {
        let g = parse_fen(fen).unwrap();
        let s: S = shakmaty::fen::Fen::from_ascii(fen.as_bytes())
            .unwrap()
            .into_position(CastlingMode::Chess960)
            .unwrap();
        let c = C::from_fen(fen, true).unwrap();
        let moves = sorted(g.legal_moves(), |m| m.to_string());
        assert_eq!(
            moves,
            sorted(s.legal_moves(), |m| shakmaty::uci::UciMove::from_move(
                m,
                CastlingMode::Chess960
            )
            .to_string())
        );
        assert_eq!(moves, sorted(cozy_list(&c), |m| m.to_string()));
        let n = pg(&g, 3, true);
        assert_eq!(n, ps(&s, 3, true));
        assert_eq!(n, pc(&c, 3, true));
        row!("chess960_materialize",name,moves.len() as u64,"giga"=>||bb(&g).legal_moves(),"shakmaty"=>||bb(&s).legal_moves(),"cozy"=>||cozy_list(bb(&c)));
        row!("chess960_perft_d3",name,n,"giga"=>||pg(bb(&g),3,true),"shakmaty"=>||ps(bb(&s),3,true),"cozy"=>||pc(bb(&c),3,true));
    }
}
