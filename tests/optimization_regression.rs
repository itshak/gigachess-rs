// SPDX-License-Identifier: MIT
//! Final candidate gate: real public mainlines, native state/undo, SAN and UCI.
use gigachess::{Board,Move,Role,Color,Square,fen::parse_fen,san};
use shakmaty::{Chess,Position,CastlingMode,EnPassantMode,uci::UciMove,zobrist::Zobrist64,san::SanPlus};
use std::collections::HashSet;

fn uci(b:&Board,m:Move)->String {
    let s=m.to_string();
    if m.from()!=b.king_square(b.turn()){return s;}
    match s.as_str(){"e1h1"=>"e1g1".into(),"e1a1"=>"e1c1".into(),"e8h8"=>"e8g8".into(),"e8a8"=>"e8c8".into(),_=>s}
}
fn same_perft_state(a:&Board,b:&Board) {
    assert_eq!(a.turn(),b.turn());assert_eq!(a.castling_rights(),b.castling_rights());
    assert_eq!(a.en_passant(),b.en_passant());assert_eq!(a.checkers_bb(),b.checkers_bb());
    for c in [Color::White,Color::Black] {
        assert_eq!(a.occ_color(c),b.occ_color(c));assert_eq!(a.king_square(c),b.king_square(c));
        for r in [Role::Pawn,Role::Knight,Role::Bishop,Role::Rook,Role::Queen,Role::King] {
            assert_eq!(a.piece_bb(c,r),b.piece_bb(c,r));
        }
    }
}
#[test]
fn real_games_exact_san_legal_and_undo() {
    let mut positions=0;
    let mut compared=0;
    let mut rng=0x90abcd12abcd3456u64;
    for line in include_str!("../benchmarks/competitive/data/lichess-2013-01-first300.uci").lines(){
        let mut g=Board::startpos();let mut s=Chess::default();
        for token in line.split_ascii_whitespace(){
            positions+=1;
            let gm=g.legal_moves();
            let sm=s.legal_moves();
            let legal:HashSet<_>=gm.iter().map(|m|m.word()).collect();
            assert_eq!(gm.len(),sm.len());
            let mut actual=None;
            for &m in &gm {
                compared+=1;
                let normalized=uci(&g,m);
                let parsed:UciMove=normalized.parse().unwrap();
                let reference=parsed.to_move(&s).unwrap();
                let text=SanPlus::from_move(s.clone(),reference).to_string();
                let rendered=san::move_to_san(&g,m).unwrap();
                assert_eq!(rendered.as_str(),text);
                assert_eq!(san::san_to_move(&g,&text),Some(m));
                assert!(g.is_legal(m));
                if normalized==token {actual=Some((m,reference));}
                // Test exact reversible state for every legal move, including
                // promotions, en passant, captures, and canonical castling.
                let mut full=g;let undo=full.make_move_unchecked(m);
                assert_eq!(full.zobrist(),full.zobrist_full());
                let mut slim=g;let pu=slim.make_move_perft(m);
                same_perft_state(&full,&slim);
                slim.unmake_move_perft(m,pu);assert_eq!(slim,g);
                full.unmake_move(m,undo);assert_eq!(full,g);
            }
            // Include negative legality tests; null is intentionally not in
            // the generated list, and encoded promotions are handled above.
            for _ in 0..12 {
                rng^=rng<<13;rng^=rng>>7;rng^=rng<<17;
                let m=Move::quiet(Square((rng&63) as u8),Square(((rng>>6)&63) as u8));
                assert_eq!(g.is_legal(m),legal.contains(&m.word()),"illegal/valid disagreement at {}",g.to_fen());
                #[cfg(feature="exp-checked-direct")]
                {
                    let mut checked=g;
                    let result=checked.play(m);
                    assert_eq!(result.is_ok(),legal.contains(&m.word()));
                    if let Ok(u)=result{checked.unmake_move(m,u);}
                    assert_eq!(checked,g);
                }
            }
            let (m,reference)=actual.expect("corpus move must exist");
            g.play(m).unwrap();s.play_unchecked(reference);
            assert_eq!(g.zobrist(),s.zobrist_hash::<Zobrist64>(EnPassantMode::PseudoLegal).0);
            assert_eq!(g.repetition_key(),s.zobrist_hash::<Zobrist64>(EnPassantMode::Legal).0);
            assert_eq!(g.to_fen(),shakmaty::fen::Fen::from_position(&s,EnPassantMode::Always).to_string());
        }
    }
    assert_eq!(positions,18954);
    assert!(compared>400000);
    eprintln!("verified {positions} positions, {compared} legal move SAN/undo transitions");
}

#[test]
fn static_slider_exhaustive_relevant_blockers() {
    fn ray(s:u8,occ:u64,diag:bool)->u64{
        let dirs=if diag{[(1,1),(1,-1),(-1,1),(-1,-1)]}else{[(1,0),(-1,0),(0,1),(0,-1)]};
        let mut a=0u64;
        for (df,dr) in dirs{
            let(mut f,mut r)=(s as i32%8+df,s as i32/8+dr);
            while (0..8).contains(&f)&&(0..8).contains(&r){
                let b=1u64<<(r*8+f);a|=b;if occ&b!=0{break;}f+=df;r+=dr;
            }
        }
        a
    }
    let tables=gigachess::attacks::tables();
    for sq in 0..64u8{
        for diag in [false,true]{
            let mask=if diag{tables.bishop.mask[sq as usize]}else{tables.rook.mask[sq as usize]};
            let mut occ=0u64;
            loop{
                let actual=if diag{gigachess::attacks::bishop_attacks(sq,occ)}else{gigachess::attacks::rook_attacks(sq,occ)};
                assert_eq!(actual,ray(sq,occ,diag));
                occ=occ.wrapping_sub(mask)&mask;if occ==0{break;}
            }
        }
    }
}

#[test]
fn chess960_reversible_special_moves(){
    for f in ["7k/8/8/8/8/8/8/2RKR3 w Q - 0 1",
              "7k/8/8/8/8/8/8/3KR3 w K - 0 1",
              "r2k3r/8/8/8/8/8/8/R2K3R w HAha - 0 1"]{
        let b=parse_fen(f).unwrap();
        let s:Chess=shakmaty::fen::Fen::from_ascii(f.as_bytes()).unwrap().into_position(CastlingMode::Chess960).unwrap();
        for m in b.legal_moves(){
            let ref_m=UciMove::from_move(s.legal_moves().into_iter().find(|x|UciMove::from_move(*x,CastlingMode::Chess960).to_string()==m.to_string()).unwrap(),CastlingMode::Chess960);
            assert!(ref_m.to_move(&s).is_ok());
            let mut child=b;let undo=child.make_move_unchecked(m);child.unmake_move(m,undo);assert_eq!(child,b);
        }
    }
}

#[cfg(feature="exp-uci-direct")]
#[test]
fn direct_uci_matches_display_for_every_word(){
    for word in 0..=u16::MAX{let m=Move::from_word(word);assert_eq!(m.to_uci_string(),m.to_string());}
}

#[cfg(feature="exp-full-visitor")]
#[test]
fn full_visitor_observes_identical_leaf_states() {
    fn reference(b:&Board,d:u32,out:&mut Vec<Board>){
        if d==0{out.push(*b);return;}
        for m in b.legal_moves(){let mut c=*b;c.make_move_perft(m);reference(&c,d-1,out);}
    }
    for b in [Board::startpos(),parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap(),parse_fen("4k3/P7/8/3pP3/8/8/8/4K3 w - d6 0 1").unwrap()]{
        for d in 0..=2{
            let mut expected=Vec::new();reference(&b,d,&mut expected);
            let mut actual=Vec::new();let n=b.visit_perft_leaves(d,&mut |leaf|actual.push(*leaf));
            assert_eq!(n,expected.len() as u64);
            assert_eq!(actual,expected);
        }
    }
}
