| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.977x | 0.937x | 1.363x |
| materialize | 0.970x | 0.936x | 1.229x |
| perft_bulk_d4 | 1.011x | 0.999x | 1.384x |
| perft_full_d3 | 0.989x | 0.944x | 1.200x |
| fen_parse | 1.016x | 1.000x | 1.110x |
| fen_write_native_ep | 0.964x | 0.937x | 1.698x |
| snapshot | 1.000x | 0.996x | 1.250x |
| san_parse_all | 0.989x | 0.983x | 2.027x |
| san_render_native | 0.983x | 0.947x | 2.165x |
| apply_children_full | 0.981x | 0.970x | 1.121x |
| make_unmake_full | 0.995x | 0.989x | 1.243x |
| startpos | 1.001x | 1.001x | 1.073x |
| chess960_materialize | 1.006x | 1.000x | 1.293x |
| chess960_perft_d3 | 0.985x | 0.979x | 1.353x |
| polyglot_key | 1.070x | 1.042x | 29.747x |
| polyglot_recompute | 0.998x | 0.997x | 1.043x |
| repetition_key | 0.994x | 0.991x | 21.053x |
| corpus_count | 1.016x | 1.010x | 1.498x |
| corpus_materialize | 1.006x | 1.001x | 1.215x |
| corpus_uci_sorted | 1.011x | 1.008x | 1.750x |
| replay_trusted_native | 0.997x | 0.994x | 0.695x |
| replay_checked_native | 1.024x | 1.017x | 0.994x |

Cold first legal move list: 0.013 ms -> 0.011 ms (1.12x).
