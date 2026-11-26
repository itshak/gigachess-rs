| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.992x | 0.977x | 1.544x |
| materialize | 0.992x | 0.972x | 1.583x |
| perft_bulk_d4 | 0.975x | 0.960x | 1.154x |
| perft_full_d3 | 0.938x | 0.916x | 1.274x |
| fen_parse | 0.889x | 0.842x | 0.995x |
| fen_write_native_ep | 0.991x | 0.942x | 2.039x |
| snapshot | 0.672x | 0.670x | 1.000x |
| san_parse_all | 0.998x | 0.972x | 1.723x |
| san_render_native | 0.954x | 0.909x | 1.997x |
| apply_children_full | 0.931x | 0.910x | 1.273x |
| make_unmake_full | 0.930x | 0.909x | 1.081x |
| startpos | 0.686x | 0.686x | 0.910x |
| chess960_materialize | 0.877x | 0.818x | 1.352x |
| chess960_perft_d3 | 0.915x | 0.891x | 1.350x |
| polyglot_key | 1.034x | 1.001x | 40.389x |
| polyglot_recompute | 1.055x | 1.052x | 1.054x |
| repetition_key | 0.779x | 0.692x | 18.905x |
| corpus_count | 1.039x | 1.016x | 1.117x |
| corpus_materialize | 0.982x | 0.978x | 1.081x |
| corpus_uci_sorted | 1.000x | 0.995x | 1.564x |
| replay_trusted_native | 0.903x | 0.860x | 0.800x |
| replay_checked_native | 0.971x | 0.954x | 1.015x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.10x).
