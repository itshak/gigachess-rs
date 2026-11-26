| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.997x | 1.086x |
| materialize | 1.005x | 1.003x | 1.172x |
| perft_bulk_d4 | 0.997x | 0.987x | 0.904x |
| perft_full_d3 | 0.984x | 0.962x | 0.603x |
| fen_parse | 1.008x | 1.002x | 1.207x |
| fen_write_native_ep | 0.987x | 0.984x | 1.072x |
| snapshot | 0.987x | 0.976x | 0.753x |
| san_parse_all | 0.990x | 0.979x | 0.902x |
| san_render_native | 0.980x | 0.967x | 1.122x |
| apply_children_full | 0.983x | 0.966x | 0.637x |
| make_unmake_full | 0.985x | 0.970x | 0.875x |
| startpos | 0.940x | 0.940x | 0.771x |
| chess960_materialize | 0.999x | 0.997x | 1.106x |
| chess960_perft_d3 | 0.999x | 0.992x | 0.837x |
| polyglot_key | 0.999x | 0.996x | 23.193x |
| polyglot_recompute | 0.999x | 0.996x | 1.135x |
| repetition_key | 1.013x | 0.997x | 14.002x |
| corpus_count | 0.998x | 0.995x | 1.212x |
| corpus_materialize | 0.998x | 0.993x | 1.114x |
| corpus_uci_sorted | 0.990x | 0.990x | 0.570x |
| replay_trusted_native | 0.969x | 0.966x | 0.309x |
| replay_checked_native | 0.980x | 0.979x | 0.643x |

Cold first legal move list: 154.046 ms → 154.073 ms (1.00x).
