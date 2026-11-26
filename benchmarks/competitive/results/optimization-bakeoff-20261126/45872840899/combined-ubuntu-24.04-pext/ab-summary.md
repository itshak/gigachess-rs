| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.965x | 1.556x |
| materialize | 0.917x | 0.865x | 1.490x |
| perft_bulk_d4 | 1.004x | 0.994x | 1.188x |
| perft_full_d3 | 1.018x | 0.996x | 1.397x |
| fen_parse | 1.001x | 0.971x | 1.157x |
| fen_write_native_ep | 0.971x | 0.914x | 1.976x |
| snapshot | 1.006x | 0.980x | 1.154x |
| san_parse_all | 0.994x | 0.980x | 1.828x |
| san_render_native | 0.988x | 0.973x | 2.042x |
| apply_children_full | 0.982x | 0.968x | 1.310x |
| make_unmake_full | 0.969x | 0.945x | 1.152x |
| startpos | 1.120x | 1.120x | 1.116x |
| chess960_materialize | 0.938x | 0.936x | 1.414x |
| chess960_perft_d3 | 1.002x | 0.986x | 1.476x |
| polyglot_key | 0.983x | 0.964x | 39.226x |
| polyglot_recompute | 1.018x | 1.015x | 1.024x |
| repetition_key | 0.809x | 0.697x | 19.273x |
| corpus_count | 0.949x | 0.905x | 1.064x |
| corpus_materialize | 1.003x | 0.996x | 1.074x |
| corpus_uci_sorted | 1.007x | 1.002x | 1.547x |
| replay_trusted_native | 0.930x | 0.920x | 0.823x |
| replay_checked_native | 0.948x | 0.936x | 1.100x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.88x).
