| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.985x | 1.184x |
| materialize | 1.090x | 1.042x | 1.367x |
| perft_bulk_d4 | 0.998x | 0.993x | 0.873x |
| perft_full_d3 | 1.001x | 0.997x | 0.605x |
| fen_parse | 0.997x | 0.990x | 1.139x |
| fen_write_native_ep | 1.002x | 0.993x | 1.193x |
| snapshot | 1.044x | 1.031x | 0.787x |
| san_parse_all | 1.003x | 0.994x | 0.909x |
| san_render_native | 1.007x | 0.996x | 1.057x |
| apply_children_full | 1.004x | 0.998x | 0.666x |
| make_unmake_full | 1.005x | 1.001x | 0.757x |
| startpos | 1.071x | 1.071x | 0.663x |
| chess960_materialize | 0.939x | 0.929x | 1.126x |
| chess960_perft_d3 | 1.004x | 1.002x | 0.961x |
| polyglot_key | 1.049x | 1.003x | 36.416x |
| polyglot_recompute | 0.998x | 0.996x | 0.976x |
| repetition_key | 1.039x | 1.000x | 14.711x |
| corpus_count | 1.000x | 0.997x | 1.146x |
| corpus_materialize | 1.016x | 1.015x | 1.085x |
| corpus_uci_sorted | 1.206x | 1.202x | 0.756x |
| replay_trusted_native | 1.005x | 1.001x | 0.515x |
| replay_checked_native | 1.004x | 0.998x | 0.759x |

Cold first legal move list: 148.331 ms → 161.688 ms (0.92x).
