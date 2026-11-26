| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.985x | 0.929x | 1.515x |
| materialize | 0.995x | 0.975x | 1.142x |
| perft_bulk_d4 | 1.018x | 1.002x | 1.432x |
| perft_full_d3 | 1.020x | 1.005x | 1.245x |
| fen_parse | 1.068x | 1.052x | 1.538x |
| fen_write_native_ep | 0.994x | 0.972x | 1.586x |
| snapshot | 1.001x | 0.995x | 1.194x |
| san_parse_all | 0.999x | 0.985x | 1.958x |
| san_render_native | 1.017x | 0.998x | 3.347x |
| apply_children_full | 0.993x | 0.965x | 1.308x |
| make_unmake_full | 0.992x | 0.950x | 1.290x |
| startpos | 0.999x | 0.999x | 1.000x |
| chess960_materialize | 1.016x | 1.011x | 1.346x |
| chess960_perft_d3 | 0.995x | 0.994x | 1.594x |
| polyglot_key | 1.003x | 1.001x | 20.311x |
| polyglot_recompute | 0.987x | 0.979x | 0.978x |
| repetition_key | 0.997x | 0.997x | 19.307x |
| corpus_count | 1.005x | 1.005x | 1.310x |
| corpus_materialize | 0.999x | 0.989x | 1.183x |
| corpus_uci_sorted | 0.992x | 0.992x | 1.523x |
| replay_trusted_native | 0.931x | 0.864x | 1.076x |
| replay_checked_native | 0.996x | 0.995x | 1.083x |

Cold first legal move list: 0.013 ms -> 0.016 ms (0.84x).
