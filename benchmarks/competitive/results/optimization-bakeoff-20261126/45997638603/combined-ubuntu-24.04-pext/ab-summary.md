| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.038x | 0.970x | 1.587x |
| materialize | 1.001x | 0.983x | 1.545x |
| perft_bulk_d4 | 1.055x | 1.020x | 1.236x |
| perft_full_d3 | 1.233x | 1.049x | 1.688x |
| fen_parse | 1.009x | 0.994x | 1.237x |
| fen_write_native_ep | 1.009x | 0.990x | 2.143x |
| snapshot | 1.000x | 0.998x | 1.155x |
| san_parse_all | 1.005x | 0.994x | 1.777x |
| san_render_native | 0.985x | 0.979x | 2.134x |
| apply_children_full | 0.998x | 0.994x | 1.334x |
| make_unmake_full | 1.006x | 0.994x | 1.196x |
| startpos | 0.894x | 0.894x | 1.121x |
| chess960_materialize | 0.984x | 0.946x | 1.396x |
| chess960_perft_d3 | 1.009x | 0.994x | 1.483x |
| polyglot_key | 1.006x | 1.002x | 39.918x |
| polyglot_recompute | 1.011x | 0.998x | 0.998x |
| repetition_key | 0.814x | 0.784x | 25.458x |
| corpus_count | 1.201x | 1.122x | 1.145x |
| corpus_materialize | 1.012x | 1.010x | 1.084x |
| corpus_uci_sorted | 0.980x | 0.978x | 1.550x |
| replay_trusted_native | 0.964x | 0.896x | 0.842x |
| replay_checked_native | 1.005x | 1.001x | 1.206x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.04x).
