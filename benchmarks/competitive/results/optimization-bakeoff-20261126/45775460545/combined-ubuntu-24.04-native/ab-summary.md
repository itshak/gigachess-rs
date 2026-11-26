| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.007x | 0.994x | 1.278x |
| materialize | 1.006x | 0.996x | 1.090x |
| perft_bulk_d4 | 0.979x | 0.948x | 1.228x |
| perft_full_d3 | 1.082x | 1.025x | 0.678x |
| fen_parse | 1.019x | 1.005x | 1.064x |
| fen_write_native_ep | 1.026x | 1.003x | 1.224x |
| snapshot | 0.813x | 0.809x | 1.114x |
| san_parse_all | 1.026x | 1.017x | 1.999x |
| san_render_native | 0.952x | 0.921x | 2.090x |
| apply_children_full | 1.039x | 1.004x | 0.907x |
| make_unmake_full | 1.064x | 1.048x | 1.016x |
| startpos | 1.096x | 1.096x | 1.183x |
| chess960_materialize | 1.026x | 1.018x | 1.379x |
| chess960_perft_d3 | 1.019x | 1.007x | 1.391x |
| polyglot_key | 0.988x | 0.980x | 20.103x |
| polyglot_recompute | 0.998x | 0.992x | 1.014x |
| repetition_key | 1.009x | 1.006x | 18.807x |
| corpus_count | 1.003x | 1.002x | 1.292x |
| corpus_materialize | 0.992x | 0.990x | 1.202x |
| corpus_uci_sorted | 0.996x | 0.984x | 0.966x |
| replay_trusted_native | 1.051x | 1.018x | 0.673x |
| replay_checked_native | 1.044x | 1.030x | 0.937x |

Cold first legal move list: 0.014 ms -> 0.013 ms (1.07x).
