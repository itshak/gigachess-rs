| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.057x | 1.001x | 1.359x |
| materialize | 1.065x | 1.036x | 1.743x |
| perft_bulk_d4 | 1.073x | 1.048x | 1.268x |
| perft_full_d3 | 1.166x | 1.027x | 1.548x |
| fen_parse | 0.929x | 0.759x | 1.087x |
| fen_write_native_ep | 1.003x | 0.996x | 1.809x |
| snapshot | 1.002x | 0.996x | 1.000x |
| san_parse_all | 1.001x | 0.981x | 1.902x |
| san_render_native | 1.018x | 0.996x | 2.202x |
| apply_children_full | 1.008x | 0.995x | 1.401x |
| make_unmake_full | 1.013x | 0.998x | 1.332x |
| startpos | 0.995x | 0.995x | 1.426x |
| chess960_materialize | 0.934x | 0.927x | 1.483x |
| chess960_perft_d3 | 1.017x | 1.004x | 1.523x |
| polyglot_key | 1.002x | 0.998x | 42.692x |
| polyglot_recompute | 1.615x | 1.270x | 1.600x |
| repetition_key | 1.042x | 0.997x | 25.540x |
| corpus_count | 1.144x | 1.113x | 1.338x |
| corpus_materialize | 1.013x | 1.007x | 1.161x |
| corpus_uci_sorted | 0.992x | 0.985x | 1.527x |
| replay_trusted_native | 0.951x | 0.913x | 0.860x |
| replay_checked_native | 1.002x | 1.000x | 1.180x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.87x).
