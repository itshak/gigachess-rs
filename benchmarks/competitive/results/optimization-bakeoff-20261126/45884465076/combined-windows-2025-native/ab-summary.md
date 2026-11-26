| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.007x | 0.953x | 1.485x |
| materialize | 1.025x | 0.984x | 1.201x |
| perft_bulk_d4 | 1.013x | 0.987x | 1.374x |
| perft_full_d3 | 1.031x | 1.012x | 1.305x |
| fen_parse | 1.014x | 0.986x | 1.365x |
| fen_write_native_ep | 0.982x | 0.873x | 1.482x |
| snapshot | 0.998x | 0.966x | 1.489x |
| san_parse_all | 0.996x | 0.920x | 1.948x |
| san_render_native | 1.009x | 0.962x | 3.997x |
| apply_children_full | 0.984x | 0.927x | 1.362x |
| make_unmake_full | 1.012x | 0.944x | 1.276x |
| startpos | 1.000x | 1.000x | 1.004x |
| chess960_materialize | 0.978x | 0.976x | 1.405x |
| chess960_perft_d3 | 1.012x | 1.012x | 1.519x |
| polyglot_key | 0.985x | 0.981x | 37.521x |
| polyglot_recompute | 1.033x | 1.003x | 1.003x |
| repetition_key | 1.001x | 0.988x | 28.320x |
| corpus_count | 0.997x | 0.995x | 1.158x |
| corpus_materialize | 1.034x | 0.993x | 1.110x |
| corpus_uci_sorted | 0.987x | 0.980x | 1.365x |
| replay_trusted_native | 1.010x | 0.989x | 0.743x |
| replay_checked_native | 0.999x | 0.997x | 1.177x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.07x).
