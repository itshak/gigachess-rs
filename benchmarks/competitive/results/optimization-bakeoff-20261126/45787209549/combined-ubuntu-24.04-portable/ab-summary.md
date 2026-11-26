| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.975x | 1.178x |
| materialize | 0.966x | 0.940x | 1.226x |
| perft_bulk_d4 | 1.012x | 1.002x | 1.195x |
| perft_full_d3 | 0.981x | 0.949x | 1.129x |
| fen_parse | 0.998x | 0.978x | 1.199x |
| fen_write_native_ep | 0.990x | 0.974x | 1.694x |
| snapshot | 1.000x | 0.998x | 0.928x |
| san_parse_all | 0.993x | 0.981x | 2.076x |
| san_render_native | 1.006x | 0.976x | 2.442x |
| apply_children_full | 0.980x | 0.961x | 1.076x |
| make_unmake_full | 0.976x | 0.947x | 1.296x |
| startpos | 0.998x | 0.998x | 1.308x |
| chess960_materialize | 0.990x | 0.977x | 1.474x |
| chess960_perft_d3 | 1.013x | 1.010x | 1.418x |
| polyglot_key | 1.005x | 1.003x | 21.926x |
| polyglot_recompute | 1.069x | 1.000x | 1.052x |
| repetition_key | 1.009x | 1.002x | 20.571x |
| corpus_count | 1.006x | 1.005x | 1.225x |
| corpus_materialize | 1.003x | 1.001x | 1.299x |
| corpus_uci_sorted | 1.018x | 1.014x | 1.610x |
| replay_trusted_native | 0.978x | 0.977x | 0.664x |
| replay_checked_native | 1.007x | 0.983x | 1.003x |

Cold first legal move list: 0.012 ms -> 0.012 ms (0.99x).
