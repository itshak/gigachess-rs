| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.996x | 1.199x |
| materialize | 0.948x | 0.914x | 1.579x |
| perft_bulk_d4 | 1.009x | 0.999x | 1.068x |
| perft_full_d3 | 1.000x | 0.989x | 1.455x |
| fen_parse | 1.020x | 0.993x | 1.129x |
| fen_write_native_ep | 1.005x | 0.975x | 2.064x |
| snapshot | 0.999x | 0.986x | 1.002x |
| san_parse_all | 0.980x | 0.947x | 1.968x |
| san_render_native | 1.006x | 1.001x | 2.171x |
| apply_children_full | 1.016x | 1.009x | 1.328x |
| make_unmake_full | 1.015x | 0.949x | 1.249x |
| startpos | 0.996x | 0.996x | 1.281x |
| chess960_materialize | 1.022x | 0.988x | 1.406x |
| chess960_perft_d3 | 1.004x | 1.000x | 1.390x |
| polyglot_key | 0.989x | 0.980x | 40.931x |
| polyglot_recompute | 0.982x | 0.981x | 0.973x |
| repetition_key | 1.020x | 0.996x | 19.972x |
| corpus_count | 0.977x | 0.977x | 0.988x |
| corpus_materialize | 0.994x | 0.991x | 1.089x |
| corpus_uci_sorted | 1.003x | 0.999x | 1.525x |
| replay_trusted_native | 1.012x | 0.997x | 0.876x |
| replay_checked_native | 1.121x | 1.082x | 1.203x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.01x).
