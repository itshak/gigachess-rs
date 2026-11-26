| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.990x | 1.224x |
| materialize | 0.995x | 0.961x | 1.591x |
| perft_bulk_d4 | 0.989x | 0.984x | 1.065x |
| perft_full_d3 | 0.997x | 0.976x | 1.453x |
| fen_parse | 1.003x | 0.991x | 1.252x |
| fen_write_native_ep | 0.987x | 0.967x | 1.805x |
| snapshot | 0.998x | 0.994x | 1.001x |
| san_parse_all | 1.013x | 0.997x | 1.862x |
| san_render_native | 0.838x | 0.816x | 1.797x |
| apply_children_full | 1.004x | 1.001x | 1.207x |
| make_unmake_full | 1.014x | 0.974x | 1.172x |
| startpos | 0.989x | 0.989x | 1.415x |
| chess960_materialize | 0.974x | 0.955x | 1.361x |
| chess960_perft_d3 | 0.990x | 0.988x | 1.381x |
| polyglot_key | 1.091x | 0.997x | 43.770x |
| polyglot_recompute | 1.020x | 1.009x | 1.055x |
| repetition_key | 1.139x | 1.025x | 20.538x |
| corpus_count | 0.959x | 0.924x | 0.991x |
| corpus_materialize | 1.008x | 1.006x | 1.084x |
| corpus_uci_sorted | 0.991x | 0.985x | 1.496x |
| replay_trusted_native | 1.020x | 1.001x | 0.869x |
| replay_checked_native | 0.982x | 0.968x | 1.077x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.98x).
