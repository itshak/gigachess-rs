## MODIFIED Requirements

### Requirement: Null Move SHALL Pass the Turn as a Board Primitive

The system SHALL expose `Board::make_null_move() -> Result<Undo, IllegalMove>` and `Board::unmake_null_move(Undo)` as the null-move (pass) primitive: placement and castling rights untouched, side to move flipped, en-passant square cleared, halfmove clock advanced, and the full move completed (the number advances, whoever passed — after the pass White is to move at the next number). The incremental Polyglot hash and the cached `checkers` SHALL be maintained, so `zobrist()` still equals `zobrist_full()` and `in_check()` stays branch-free. A null move with the side to move in check SHALL return `IllegalMove`. Unmake SHALL restore the exact prior position when paired like `unmake_move` (no ordinary moves interleaved past the matching make).

The null move SHALL additionally be a first-class `Move` word: `Move::NULL` is `0xffff` with `is_null()` true, and no legal move word SHALL ever equal it. All eight make/play entry points (`play`, `play_fast`, `play_hashed`, `play_checkered`, `make_move_unchecked`, `make_move_hashed`, `make_move_checkered`, `make_move_fast`) SHALL check `is_null()` first and take the null path with that entry's own hash/checkers maintenance; the shared generic body SHALL `debug_assert(!is_null)` so a caller that bypasses dispatch fails in debug rather than corrupting in production. The pass/refuse test SHALL always be a fresh `attackers_to(king_square(turn), turn.other(), occupied()) == 0` computation, never the cached `checkers` (`in_check()`), in every variant including the non-fast ones. `is_pseudo_legal(Move::NULL)` SHALL be true; `is_legal(Move::NULL)` SHALL be the fresh test. `legal_moves()` and all bulk counters/visitors SHALL never include the null move. `Move::NULL` SHALL pair exclusively with `unmake_null_move`; passing a null word to `unmake_move` (or the fast/perft unmakes) is undefined and SHALL `debug_assert`.

#### Scenario: Pass flips the side and preserves the placement

- **WHEN** `make_null_move()` is played on `rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1`
- **THEN** the board becomes `rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2` (turn flipped, ep cleared, clocks advanced) with `zobrist() == zobrist_full()`, and `unmake_null_move` restores the position bit-for-bit

#### Scenario: Null move in check is illegal

- **WHEN** the side to move is in check
- **THEN** `make_null_move()` returns `IllegalMove`

#### Scenario: Null travels as a Move word through every entry point

- **WHEN** `Move::NULL` reaches `play`, `play_fast`, `play_hashed`, `play_checkered`, `make_move_unchecked`, `make_move_hashed`, `make_move_checkered` or `make_move_fast` outside check
- **THEN** each produces the null-move result (turn flipped, ep cleared, `fullmove + 1`, castling rights untouched, that entry's hash/checkers maintenance honored), and in check each validating entry returns `IllegalMove`

#### Scenario: A stale checkers cache never decides a pass

- **WHEN** a checking move is made with `play_fast` — leaving `checkers` stale — so the side to move is in check while `in_check()` reads false
- **THEN** `is_legal(Move::NULL)` is false, `play(Move::NULL)` and every null variant return `IllegalMove`, and all verdicts equal a fresh `attackers_to` test

#### Scenario: Null legality without enumeration

- **WHEN** `is_legal(Move::NULL)` is queried in any cache state
- **THEN** it is true exactly when a fresh `attackers_to` test finds the side to move not in check, `is_pseudo_legal(Move::NULL)` is true, and `legal_moves()` never contains `Move::NULL`
- **AND** it is answered by a dedicated null branch, never by the generic make-and-test body, which asks whether the *opponent's* king survives the move — the wrong question for a pass, and the wrong answer on a stale-cache check position

#### Scenario: Legality of a pass is decided for the mover, not the opponent

- **WHEN** the side to move is in check and `is_legal(Move::NULL)` is queried
- **THEN** it is `false`, whether the `checkers` cache is fresh or stale
- **AND** a test pins this against a position where the generic body's opponent-king test would return `true`

#### Scenario: Null pairs only with its own unmake

- **WHEN** a null move is unmade with `unmake_null_move`
- **THEN** the exact prior position is restored; passing `Move::NULL` to `unmake_move` is undefined and debug-asserts rather than silently round-tripping through phantom squares

### Requirement: Move Representation SHALL Use 16-bit Packed Encoding

The system SHALL represent a move as a single `u16` in `moves2` wire format — `from | (to << 6) | (promo << 12)` — convertible to and from `Move` without allocation, and SHALL keep castling as king-to-rook squares (`e1h1`, `e1a1`) for standard chess and Chess960 alike. The packed form SHALL be the currency of the batch replay engine, so a `moves2` stream replays without materialising anything per move.

Word `0xffff` is RESERVED as the null-move marker (`Move::NULL`): it names the same square twice with a promotion value above 4, so no legal move word SHALL ever equal it. `Move::NULL` SHALL render as UCI `0000` and SHALL be detected with a single `u16` compare (`is_null()`), keeping the hot replay loop branch-cheap and allocation-free.

#### Scenario: Packed moves round-trip and stay compact

- **WHEN** a `Move` is built for each kind of move — a quiet move, a capture, a double push, an en-passant capture, a promotion and both castlings — and converted to its `u16` word and back
- **THEN** every move round-trips to the identical `Move`
- **AND** every word fits in 16 bits, and a `moves2` stream of a whole game replays through `replay.rs` with no allocation.

#### Scenario: Null word is unambiguous

- **WHEN** any legal move is encoded
- **THEN** its word is never `0xffff`, and `Move::from_word(0xffff).is_null()` is true while `Move::from_word(w).is_null()` is false for every legal `w`
