| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.983x | 1.559x |
| materialize | 1.011x | 0.992x | 1.342x |
| perft_bulk_d4 | 1.041x | 1.017x | 1.450x |
| perft_full_d3 | 1.130x | 1.008x | 1.515x |
| fen_parse | 1.016x | 0.941x | 1.236x |
| fen_write_native_ep | 0.984x | 0.973x | 1.557x |
| snapshot | 1.010x | 1.005x | 0.948x |
| san_parse_all | 1.001x | 0.997x | 1.850x |
| san_render_native | 1.002x | 0.989x | 2.693x |
| apply_children_full | 0.995x | 0.976x | 1.151x |
| make_unmake_full | 1.016x | 0.999x | 1.590x |
| startpos | 0.999x | 0.999x | 1.302x |
| chess960_materialize | 1.001x | 0.998x | 1.504x |
| chess960_perft_d3 | 1.021x | 1.010x | 1.517x |
| polyglot_key | 1.004x | 0.994x | 25.717x |
| polyglot_recompute | 1.363x | 1.273x | 1.477x |
| repetition_key | 0.999x | 0.997x | 19.476x |
| corpus_count | 1.098x | 1.063x | 1.496x |
| corpus_materialize | 0.991x | 0.987x | 1.218x |
| corpus_uci_sorted | 1.001x | 0.998x | 1.462x |
| replay_trusted_native | 1.001x | 0.995x | 0.638x |
| replay_checked_native | 1.021x | 1.010x | 1.147x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.91x).
