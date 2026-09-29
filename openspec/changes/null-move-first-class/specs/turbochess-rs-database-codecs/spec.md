## ADDED Requirements

### Requirement: Database Codecs SHALL Carry Null Moves End to End

The system SHALL carry the null move (`0xffff` word, `--` SAN) through every database codec: `parse_movetext_to_moves2` SHALL accept `--` and `Z0` tokens by playing the null-move path (rejecting them in check as illegal, as decided by the same fresh test); `moves2_to_san_movetext` SHALL render a `0xffff` word as `--` with no suffix and SHALL number it as a full move (the number advances whoever passed, matching the ChessBase export rule); `replay_moves2_hashes` / `replay_movetext_hashes` SHALL dispatch `0xffff` words to the null-move path with incremental hashes maintained; `position_stats` SHALL count positions reached across null moves like any other ply.

#### Scenario: Movetext with a null parses to the marker word

- **WHEN** movetext containing `--` or `Z0` outside check is parsed
- **THEN** the emitted `moves2` stream contains `0xffff` at that ply, and a null token in check is a codec error

#### Scenario: moves2 with the marker renders as dashes and round-trips

- **WHEN** a `moves2` blob containing `0xffff` is rendered to SAN movetext and re-parsed
- **THEN** the null ply renders as `--` with no `+`/`#`, move numbering stays consistent (fullmove advances), and re-parsing reproduces the identical blob

#### Scenario: Hash replay stays incremental across a pass

- **WHEN** a `moves2` stream with `0xffff` is replayed for hashes
- **THEN** every yielded hash equals from-scratch recomputation at that ply, including the positions before and after the pass

#### Scenario: Position stats count across nulls

- **WHEN** a batch containing null moves is aggregated
- **THEN** per-position counts and samples match a sequential reference that plays nulls through the same path
