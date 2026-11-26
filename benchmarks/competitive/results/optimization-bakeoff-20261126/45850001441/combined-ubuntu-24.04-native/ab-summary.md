| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.993x | 1.514x |
| materialize | 0.999x | 0.992x | 1.578x |
| perft_bulk_d4 | 0.991x | 0.984x | 1.171x |
| perft_full_d3 | 1.226x | 1.055x | 1.648x |
| fen_parse | 0.935x | 0.821x | 1.092x |
| fen_write_native_ep | 0.960x | 0.912x | 1.830x |
| snapshot | 0.999x | 0.993x | 1.152x |
| san_parse_all | 1.116x | 1.084x | 1.926x |
| san_render_native | 0.998x | 0.992x | 2.124x |
| apply_children_full | 0.998x | 0.996x | 1.299x |
| make_unmake_full | 0.992x | 0.978x | 1.161x |
| startpos | 1.005x | 1.005x | 1.121x |
| chess960_materialize | 1.061x | 1.058x | 1.539x |
| chess960_perft_d3 | 1.006x | 1.004x | 1.509x |
| polyglot_key | 0.982x | 0.962x | 40.747x |
| polyglot_recompute | 0.973x | 0.966x | 1.030x |
| repetition_key | 0.971x | 0.948x | 24.413x |
| corpus_count | 0.992x | 0.990x | 1.098x |
| corpus_materialize | 1.000x | 0.998x | 1.069x |
| corpus_uci_sorted | 1.006x | 1.003x | 1.555x |
| replay_trusted_native | 1.002x | 0.999x | 0.864x |
| replay_checked_native | 1.014x | 1.010x | 1.069x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.88x).
