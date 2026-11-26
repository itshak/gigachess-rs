| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.355x |
| materialize | 1.000x | 0.995x | 1.227x |
| perft_bulk_d4 | 0.998x | 0.995x | 1.352x |
| perft_full_d3 | 1.020x | 1.003x | 1.080x |
| fen_parse | 1.007x | 1.001x | 1.241x |
| fen_write_native_ep | 0.999x | 0.991x | 1.060x |
| snapshot | 1.076x | 1.073x | 0.911x |
| san_parse_all | 1.003x | 0.996x | 1.833x |
| san_render_native | 0.999x | 0.994x | 2.549x |
| apply_children_full | 1.021x | 1.016x | 1.049x |
| make_unmake_full | 1.002x | 0.994x | 1.433x |
| startpos | 1.037x | 1.037x | 1.185x |
| chess960_materialize | 0.985x | 0.983x | 1.298x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.319x |
| polyglot_key | 0.990x | 0.985x | 25.467x |
| polyglot_recompute | 0.995x | 0.990x | 1.071x |
| repetition_key | 1.000x | 0.999x | 19.241x |
| corpus_count | 0.976x | 0.974x | 1.455x |
| corpus_materialize | 0.992x | 0.990x | 1.186x |
| corpus_uci_sorted | 0.997x | 0.994x | 1.458x |
| replay_trusted_native | 0.985x | 0.981x | 0.490x |
| replay_checked_native | 0.978x | 0.976x | 0.976x |

Cold first legal move list: 0.011 ms -> 0.011 ms (1.00x).
