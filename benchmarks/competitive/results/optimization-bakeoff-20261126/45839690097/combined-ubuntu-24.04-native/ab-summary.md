| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.029x | 0.984x | 1.484x |
| materialize | 0.975x | 0.922x | 1.475x |
| perft_bulk_d4 | 0.998x | 0.991x | 1.165x |
| perft_full_d3 | 1.053x | 1.023x | 1.349x |
| fen_parse | 1.024x | 1.010x | 1.158x |
| fen_write_native_ep | 1.006x | 0.988x | 1.963x |
| snapshot | 0.999x | 0.989x | 1.329x |
| san_parse_all | 0.928x | 0.901x | 1.670x |
| san_render_native | 1.007x | 0.979x | 2.134x |
| apply_children_full | 1.001x | 0.995x | 1.218x |
| make_unmake_full | 1.012x | 0.979x | 1.169x |
| startpos | 1.121x | 1.121x | 1.253x |
| chess960_materialize | 0.938x | 0.891x | 1.320x |
| chess960_perft_d3 | 1.016x | 1.008x | 1.519x |
| polyglot_key | 0.959x | 0.909x | 37.835x |
| polyglot_recompute | 1.052x | 1.022x | 0.998x |
| repetition_key | 0.773x | 0.749x | 23.982x |
| corpus_count | 1.004x | 0.971x | 0.931x |
| corpus_materialize | 0.994x | 0.993x | 1.045x |
| corpus_uci_sorted | 0.996x | 0.992x | 1.568x |
| replay_trusted_native | 1.023x | 1.007x | 0.876x |
| replay_checked_native | 1.001x | 1.001x | 1.141x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.10x).
