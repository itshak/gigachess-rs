| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.015x | 0.994x | 1.585x |
| materialize | 1.007x | 0.995x | 1.330x |
| perft_bulk_d4 | 1.014x | 1.006x | 1.412x |
| perft_full_d3 | 1.002x | 0.998x | 1.334x |
| fen_parse | 0.996x | 0.991x | 1.226x |
| fen_write_native_ep | 0.999x | 0.995x | 1.573x |
| snapshot | 0.999x | 0.997x | 0.945x |
| san_parse_all | 0.998x | 0.990x | 1.830x |
| san_render_native | 1.003x | 0.998x | 2.671x |
| apply_children_full | 0.995x | 0.982x | 1.155x |
| make_unmake_full | 1.006x | 0.987x | 1.573x |
| startpos | 1.000x | 1.000x | 1.304x |
| chess960_materialize | 0.998x | 0.994x | 1.495x |
| chess960_perft_d3 | 1.006x | 0.997x | 1.500x |
| polyglot_key | 1.014x | 1.002x | 25.974x |
| polyglot_recompute | 1.001x | 0.998x | 1.091x |
| repetition_key | 1.017x | 0.998x | 19.303x |
| corpus_count | 1.014x | 1.002x | 1.368x |
| corpus_materialize | 1.005x | 0.998x | 1.229x |
| corpus_uci_sorted | 1.004x | 1.001x | 1.462x |
| replay_trusted_native | 0.987x | 0.980x | 0.618x |
| replay_checked_native | 1.016x | 1.007x | 1.149x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.03x).
