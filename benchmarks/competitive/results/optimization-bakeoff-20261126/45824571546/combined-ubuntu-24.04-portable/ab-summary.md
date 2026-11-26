| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.015x | 1.002x | 1.141x |
| materialize | 1.017x | 0.988x | 1.262x |
| perft_bulk_d4 | 1.002x | 0.958x | 1.113x |
| perft_full_d3 | 1.018x | 0.952x | 1.300x |
| fen_parse | 1.011x | 0.999x | 1.223x |
| fen_write_native_ep | 1.024x | 0.992x | 1.524x |
| snapshot | 1.013x | 0.980x | 1.003x |
| san_parse_all | 1.014x | 0.944x | 1.701x |
| san_render_native | 1.018x | 0.999x | 2.257x |
| apply_children_full | 0.986x | 0.891x | 1.145x |
| make_unmake_full | 1.007x | 0.984x | 1.328x |
| startpos | 1.004x | 1.004x | 1.278x |
| chess960_materialize | 1.005x | 0.965x | 1.443x |
| chess960_perft_d3 | 1.003x | 0.994x | 1.386x |
| polyglot_key | 1.037x | 1.017x | 31.692x |
| polyglot_recompute | 1.055x | 0.982x | 1.064x |
| repetition_key | 1.002x | 0.996x | 19.925x |
| corpus_count | 1.021x | 1.006x | 1.038x |
| corpus_materialize | 1.031x | 1.025x | 1.132x |
| corpus_uci_sorted | 1.011x | 1.003x | 1.579x |
| replay_trusted_native | 0.954x | 0.925x | 0.655x |
| replay_checked_native | 1.025x | 1.015x | 1.191x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.99x).
