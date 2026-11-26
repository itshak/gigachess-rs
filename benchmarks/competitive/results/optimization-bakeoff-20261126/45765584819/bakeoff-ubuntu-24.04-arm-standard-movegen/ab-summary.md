| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.986x | 1.087x |
| materialize | 1.005x | 0.996x | 1.190x |
| perft_bulk_d4 | 1.029x | 1.005x | 0.916x |
| perft_full_d3 | 0.998x | 0.995x | 0.611x |
| fen_parse | 0.995x | 0.993x | 1.217x |
| fen_write_native_ep | 0.998x | 0.982x | 1.072x |
| snapshot | 0.993x | 0.972x | 0.786x |
| san_parse_all | 0.995x | 0.991x | 0.920x |
| san_render_native | 0.996x | 0.993x | 1.142x |
| apply_children_full | 1.005x | 0.996x | 0.653x |
| make_unmake_full | 0.999x | 0.989x | 0.891x |
| startpos | 1.029x | 1.029x | 0.831x |
| chess960_materialize | 0.999x | 0.999x | 1.117x |
| chess960_perft_d3 | 1.001x | 0.996x | 0.843x |
| polyglot_key | 1.102x | 0.999x | 22.967x |
| polyglot_recompute | 1.001x | 0.991x | 1.133x |
| repetition_key | 1.007x | 0.999x | 13.945x |
| corpus_count | 0.996x | 0.985x | 1.201x |
| corpus_materialize | 0.992x | 0.985x | 1.125x |
| corpus_uci_sorted | 0.999x | 0.999x | 0.571x |
| replay_trusted_native | 0.989x | 0.985x | 0.312x |
| replay_checked_native | 0.988x | 0.983x | 0.655x |

Cold first legal move list: 153.984 ms → 153.984 ms (1.00x).
