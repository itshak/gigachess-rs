| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.964x | 0.855x | 1.534x |
| materialize | 1.114x | 0.870x | 1.245x |
| perft_bulk_d4 | 0.986x | 0.868x | 1.242x |
| perft_full_d3 | 1.084x | 0.854x | 1.451x |
| fen_parse | 1.069x | 0.778x | 1.238x |
| fen_write_native_ep | 1.060x | 0.807x | 1.879x |
| snapshot | 1.049x | 0.700x | 0.930x |
| san_parse_all | 0.955x | 0.839x | 1.704x |
| san_render_native | 0.998x | 0.886x | 3.046x |
| apply_children_full | 0.996x | 0.706x | 1.086x |
| make_unmake_full | 1.129x | 0.869x | 1.358x |
| startpos | 1.192x | 1.192x | 1.107x |
| chess960_materialize | 0.977x | 0.897x | 1.562x |
| chess960_perft_d3 | 0.891x | 0.870x | 1.424x |
| polyglot_key | 0.827x | 0.799x | 38.774x |
| polyglot_recompute | 0.839x | 0.618x | 1.007x |
| repetition_key | 1.054x | 1.016x | 21.592x |
| corpus_count | 1.016x | 0.969x | 1.209x |
| corpus_materialize | 0.895x | 0.786x | 1.219x |
| corpus_uci_sorted | 0.982x | 0.948x | 1.468x |
| replay_trusted_native | 1.080x | 0.792x | 0.700x |
| replay_checked_native | 1.077x | 1.000x | 1.461x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.83x).
