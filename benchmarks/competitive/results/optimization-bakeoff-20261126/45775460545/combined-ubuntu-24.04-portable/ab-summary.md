| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 1.000x | 1.185x |
| materialize | 0.985x | 0.927x | 1.492x |
| perft_bulk_d4 | 1.006x | 0.995x | 1.162x |
| perft_full_d3 | 1.014x | 0.964x | 0.953x |
| fen_parse | 1.028x | 0.995x | 1.134x |
| fen_write_native_ep | 0.985x | 0.954x | 1.433x |
| snapshot | 1.105x | 1.099x | 0.998x |
| san_parse_all | 1.109x | 1.087x | 1.767x |
| san_render_native | 1.027x | 1.017x | 2.170x |
| apply_children_full | 1.033x | 0.986x | 1.005x |
| make_unmake_full | 1.058x | 1.050x | 1.063x |
| startpos | 0.999x | 0.999x | 1.287x |
| chess960_materialize | 1.012x | 0.958x | 1.299x |
| chess960_perft_d3 | 1.018x | 1.005x | 1.319x |
| polyglot_key | 0.970x | 0.963x | 39.809x |
| polyglot_recompute | 0.957x | 0.954x | 0.957x |
| repetition_key | 0.883x | 0.773x | 22.645x |
| corpus_count | 0.999x | 0.998x | 1.195x |
| corpus_materialize | 0.992x | 0.991x | 1.145x |
| corpus_uci_sorted | 1.002x | 0.998x | 0.970x |
| replay_trusted_native | 1.068x | 1.064x | 0.632x |
| replay_checked_native | 1.128x | 1.012x | 1.040x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.91x).
