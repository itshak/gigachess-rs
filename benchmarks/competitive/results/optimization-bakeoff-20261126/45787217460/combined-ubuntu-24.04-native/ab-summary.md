| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.017x | 1.006x | 1.309x |
| materialize | 0.983x | 0.929x | 1.183x |
| perft_bulk_d4 | 0.996x | 0.986x | 1.273x |
| perft_full_d3 | 1.001x | 0.986x | 1.239x |
| fen_parse | 1.079x | 1.042x | 1.262x |
| fen_write_native_ep | 0.987x | 0.927x | 1.730x |
| snapshot | 0.755x | 0.754x | 0.862x |
| san_parse_all | 0.975x | 0.963x | 2.025x |
| san_render_native | 1.072x | 0.998x | 2.207x |
| apply_children_full | 1.011x | 0.980x | 1.140x |
| make_unmake_full | 1.004x | 0.996x | 1.249x |
| startpos | 0.926x | 0.926x | 1.144x |
| chess960_materialize | 0.998x | 0.992x | 1.241x |
| chess960_perft_d3 | 0.995x | 0.978x | 1.342x |
| polyglot_key | 1.008x | 1.004x | 30.451x |
| polyglot_recompute | 1.023x | 1.021x | 1.055x |
| repetition_key | 0.963x | 0.938x | 22.283x |
| corpus_count | 1.006x | 1.004x | 1.366x |
| corpus_materialize | 0.997x | 0.995x | 1.205x |
| corpus_uci_sorted | 0.988x | 0.986x | 1.737x |
| replay_trusted_native | 0.920x | 0.855x | 0.674x |
| replay_checked_native | 0.999x | 0.992x | 1.009x |

Cold first legal move list: 0.014 ms -> 0.016 ms (0.91x).
