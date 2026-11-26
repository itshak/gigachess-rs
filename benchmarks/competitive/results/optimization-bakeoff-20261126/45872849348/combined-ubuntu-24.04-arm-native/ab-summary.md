| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.551x |
| materialize | 0.996x | 0.992x | 1.322x |
| perft_bulk_d4 | 0.987x | 0.972x | 1.374x |
| perft_full_d3 | 0.994x | 0.987x | 1.325x |
| fen_parse | 1.002x | 0.998x | 1.233x |
| fen_write_native_ep | 1.001x | 0.991x | 1.584x |
| snapshot | 1.008x | 1.003x | 0.947x |
| san_parse_all | 0.997x | 0.991x | 1.810x |
| san_render_native | 0.985x | 0.967x | 2.649x |
| apply_children_full | 0.974x | 0.932x | 1.125x |
| make_unmake_full | 0.949x | 0.916x | 1.482x |
| startpos | 1.000x | 1.000x | 1.305x |
| chess960_materialize | 0.985x | 0.970x | 1.476x |
| chess960_perft_d3 | 0.999x | 0.995x | 1.501x |
| polyglot_key | 1.006x | 0.999x | 26.113x |
| polyglot_recompute | 0.997x | 0.997x | 1.086x |
| repetition_key | 1.004x | 0.998x | 19.410x |
| corpus_count | 1.002x | 0.996x | 1.374x |
| corpus_materialize | 0.992x | 0.991x | 1.228x |
| corpus_uci_sorted | 0.999x | 0.998x | 1.462x |
| replay_trusted_native | 1.049x | 0.976x | 0.666x |
| replay_checked_native | 0.999x | 0.997x | 1.120x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.01x).
