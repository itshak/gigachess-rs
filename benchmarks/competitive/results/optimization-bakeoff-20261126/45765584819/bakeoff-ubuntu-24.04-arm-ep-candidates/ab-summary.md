| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.985x | 1.078x |
| materialize | 0.997x | 0.995x | 1.174x |
| perft_bulk_d4 | 1.018x | 0.999x | 0.903x |
| perft_full_d3 | 1.001x | 0.998x | 0.613x |
| fen_parse | 0.998x | 0.995x | 1.214x |
| fen_write_native_ep | 1.004x | 0.992x | 1.053x |
| snapshot | 1.022x | 1.012x | 0.799x |
| san_parse_all | 1.002x | 0.998x | 0.919x |
| san_render_native | 1.006x | 1.000x | 1.152x |
| apply_children_full | 0.999x | 0.994x | 0.654x |
| make_unmake_full | 1.010x | 1.005x | 0.896x |
| startpos | 0.999x | 0.999x | 0.860x |
| chess960_materialize | 1.001x | 1.001x | 1.114x |
| chess960_perft_d3 | 1.021x | 1.005x | 0.859x |
| polyglot_key | 0.988x | 0.978x | 22.572x |
| polyglot_recompute | 1.003x | 0.998x | 1.132x |
| repetition_key | 1.004x | 0.996x | 13.855x |
| corpus_count | 1.022x | 1.006x | 1.231x |
| corpus_materialize | 0.997x | 0.995x | 1.115x |
| corpus_uci_sorted | 1.013x | 1.010x | 0.576x |
| replay_trusted_native | 0.991x | 0.984x | 0.318x |
| replay_checked_native | 0.996x | 0.993x | 0.664x |

Cold first legal move list: 153.941 ms → 153.906 ms (1.00x).
