| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.997x | 1.553x |
| materialize | 1.004x | 1.000x | 1.316x |
| perft_bulk_d4 | 1.000x | 0.998x | 1.399x |
| perft_full_d3 | 1.144x | 1.024x | 1.527x |
| fen_parse | 1.002x | 0.989x | 1.207x |
| fen_write_native_ep | 1.000x | 0.995x | 1.596x |
| snapshot | 1.009x | 1.001x | 0.933x |
| san_parse_all | 1.003x | 1.000x | 1.819x |
| san_render_native | 1.003x | 0.998x | 2.534x |
| apply_children_full | 1.005x | 0.996x | 1.163x |
| make_unmake_full | 1.022x | 0.995x | 1.546x |
| startpos | 1.000x | 1.000x | 1.277x |
| chess960_materialize | 0.997x | 0.995x | 1.493x |
| chess960_perft_d3 | 1.010x | 1.007x | 1.504x |
| polyglot_key | 0.998x | 0.993x | 25.857x |
| polyglot_recompute | 1.001x | 1.000x | 1.087x |
| repetition_key | 1.001x | 1.001x | 19.334x |
| corpus_count | 1.022x | 1.012x | 1.361x |
| corpus_materialize | 1.012x | 1.007x | 1.210x |
| corpus_uci_sorted | 1.007x | 1.007x | 1.461x |
| replay_trusted_native | 1.004x | 1.003x | 0.630x |
| replay_checked_native | 0.999x | 0.988x | 1.047x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.01x).
