| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.931x | 0.916x | 1.436x |
| materialize | 0.986x | 0.952x | 1.136x |
| perft_bulk_d4 | 1.082x | 0.948x | 1.429x |
| perft_full_d3 | 0.862x | 0.819x | 1.060x |
| fen_parse | 0.937x | 0.901x | 1.231x |
| fen_write_native_ep | 1.012x | 0.988x | 1.641x |
| snapshot | 0.774x | 0.731x | 1.067x |
| san_parse_all | 0.893x | 0.886x | 1.716x |
| san_render_native | 0.923x | 0.877x | 3.050x |
| apply_children_full | 0.935x | 0.871x | 1.932x |
| make_unmake_full | 0.916x | 0.860x | 1.170x |
| startpos | 1.000x | 1.000x | 1.082x |
| chess960_materialize | 0.905x | 0.820x | 1.253x |
| chess960_perft_d3 | 0.924x | 0.894x | 1.496x |
| polyglot_key | 1.051x | 0.996x | 21.105x |
| polyglot_recompute | 0.978x | 0.973x | 0.943x |
| repetition_key | 1.008x | 0.961x | 19.309x |
| corpus_count | 0.978x | 0.969x | 1.286x |
| corpus_materialize | 0.963x | 0.954x | 1.143x |
| corpus_uci_sorted | 0.974x | 0.956x | 1.473x |
| replay_trusted_native | 0.722x | 0.651x | 0.846x |
| replay_checked_native | 0.924x | 0.901x | 0.918x |

Cold first legal move list: 0.015 ms -> 0.013 ms (1.15x).
