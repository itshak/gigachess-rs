| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.955x | 0.925x | 1.240x |
| materialize | 0.981x | 0.917x | 1.131x |
| perft_bulk_d4 | 0.988x | 0.968x | 1.208x |
| perft_full_d3 | 0.997x | 0.980x | 1.235x |
| fen_parse | 1.071x | 1.028x | 1.317x |
| fen_write_native_ep | 1.003x | 0.963x | 1.565x |
| snapshot | 0.994x | 0.962x | 0.994x |
| san_parse_all | 1.026x | 0.990x | 2.092x |
| san_render_native | 0.971x | 0.936x | 3.735x |
| apply_children_full | 0.980x | 0.952x | 1.349x |
| make_unmake_full | 1.013x | 0.989x | 1.328x |
| startpos | 1.005x | 1.005x | 1.258x |
| chess960_materialize | 1.013x | 1.011x | 1.301x |
| chess960_perft_d3 | 0.994x | 0.987x | 1.404x |
| polyglot_key | 1.120x | 0.997x | 30.253x |
| polyglot_recompute | 1.085x | 0.992x | 1.039x |
| repetition_key | 1.045x | 0.998x | 21.572x |
| corpus_count | 1.032x | 0.996x | 1.224x |
| corpus_materialize | 1.007x | 0.998x | 1.081x |
| corpus_uci_sorted | 0.983x | 0.979x | 1.427x |
| replay_trusted_native | 0.991x | 0.990x | 0.832x |
| replay_checked_native | 1.007x | 1.004x | 1.003x |

Cold first legal move list: 0.013 ms -> 0.015 ms (0.87x).
