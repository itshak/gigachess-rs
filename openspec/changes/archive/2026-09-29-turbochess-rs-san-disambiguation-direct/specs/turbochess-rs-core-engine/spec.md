## ADDED Requirements

### Requirement: SAN Disambiguation SHALL Equal a Legal-Move Query

`move_to_san_body` SHALL emit a file, rank, or file-and-rank qualifier exactly when another piece of the same role and colour can **legally** reach the same square, and SHALL emit none when no such piece can. The candidate set therefore equals the set a full legal-move generation would produce for the position, and the qualifier is the minimal one: the file when no other candidate shares it, otherwise the rank when none shares that, otherwise both.

Legality of a candidate SHALL mean that the move is pseudo-legal (which the `attackers_bb` pre-filter already decides exactly for knights, kings and sliders) **and** that after it is made the *mover's* king is not attacked by the side to move next. Both colours SHALL come from the position before the candidate is made. A candidate built with a different promotion than the move being rendered SHALL not count, and the search SHALL stop after eight candidates.

#### Scenario: A pinned twin is not a candidate
- **WHEN** the position is `r1bq1b1r/ppp3pp/2n1k3/3np3/2B5/2N2Q2/PPPP1PPP/R1B1K2R b KQ - 3 8` and the move is c6-e7
- **THEN** the SAN is `Ne7` without a qualifier, because the other knight is pinned and a pinned piece does not reach the square legally

#### Scenario: A defender on the king's own file is not an attack
- **WHEN** a candidate is tested for legality in a position where the mover's king is defended by one of the mover's own pieces along a rank, and the test asks whether the side to move *after* the candidate attacks its own king's square
- **THEN** the candidate is judged on the mover's king against the opponent, so the defender is not mistaken for an attacker; a hint that the notation requires is not dropped

#### Scenario: The qualifier is minimal
- **WHEN** three knights can legally reach one square, two of them from the same file as the move and one from another
- **THEN** the moves from the shared file carry the rank and the third carries its file, and no move carries both when a file or a rank alone identifies it

#### Scenario: The query agrees with a movegen oracle
- **WHEN** `tests/san_disambiguation_property.rs` renders every legal move of 200,000 positions from random playouts across the standard start and Chess960 starts, against a reference implementation that filters `generate_moves_into`
- **THEN** the two agree on every move of every position
