| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.556x |
| materialize | 0.999x | 0.996x | 1.317x |
| perft_bulk_d4 | 0.997x | 0.989x | 1.398x |
| perft_full_d3 | 0.998x | 0.972x | 1.329x |
| fen_parse | 0.991x | 0.984x | 1.217x |
| fen_write_native_ep | 1.000x | 0.999x | 1.580x |
| snapshot | 1.008x | 1.003x | 0.944x |
| san_parse_all | 1.003x | 0.993x | 1.837x |
| san_render_native | 1.013x | 0.975x | 2.708x |
| apply_children_full | 1.002x | 0.962x | 1.153x |
| make_unmake_full | 1.015x | 0.980x | 1.579x |
| startpos | 1.000x | 1.000x | 1.305x |
| chess960_materialize | 0.984x | 0.982x | 1.472x |
| chess960_perft_d3 | 0.985x | 0.981x | 1.478x |
| polyglot_key | 0.998x | 0.995x | 25.677x |
| polyglot_recompute | 0.999x | 0.998x | 1.087x |
| repetition_key | 1.012x | 0.999x | 19.424x |
| corpus_count | 0.994x | 0.991x | 1.358x |
| corpus_materialize | 0.989x | 0.989x | 1.214x |
| corpus_uci_sorted | 0.998x | 0.997x | 1.463x |
| replay_trusted_native | 0.949x | 0.936x | 0.606x |
| replay_checked_native | 0.995x | 0.985x | 1.132x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.10x).
