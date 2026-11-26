| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.998x | 1.296x |
| materialize | 0.997x | 0.971x | 1.198x |
| perft_bulk_d4 | 0.996x | 0.949x | 1.219x |
| perft_full_d3 | 1.015x | 0.972x | 1.212x |
| fen_parse | 1.160x | 1.134x | 1.181x |
| fen_write_native_ep | 0.993x | 0.953x | 1.723x |
| snapshot | 1.001x | 0.996x | 1.248x |
| san_parse_all | 1.013x | 0.976x | 2.073x |
| san_render_native | 1.004x | 0.996x | 2.199x |
| apply_children_full | 1.001x | 0.999x | 1.195x |
| make_unmake_full | 0.996x | 0.986x | 1.216x |
| startpos | 1.000x | 1.000x | 1.251x |
| chess960_materialize | 1.019x | 1.010x | 1.295x |
| chess960_perft_d3 | 0.982x | 0.974x | 1.368x |
| polyglot_key | 1.093x | 1.024x | 30.083x |
| polyglot_recompute | 1.022x | 1.019x | 1.037x |
| repetition_key | 1.022x | 1.014x | 19.462x |
| corpus_count | 0.996x | 0.995x | 1.264x |
| corpus_materialize | 0.998x | 0.998x | 1.117x |
| corpus_uci_sorted | 1.006x | 0.994x | 1.824x |
| replay_trusted_native | 1.004x | 0.999x | 0.825x |
| replay_checked_native | 1.014x | 1.003x | 1.047x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.03x).
