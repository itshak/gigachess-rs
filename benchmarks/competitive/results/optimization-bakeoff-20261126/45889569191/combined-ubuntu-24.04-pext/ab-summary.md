| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.022x | 0.980x | 1.599x |
| materialize | 1.082x | 1.040x | 1.661x |
| perft_bulk_d4 | 1.022x | 0.995x | 1.217x |
| perft_full_d3 | 1.254x | 1.059x | 1.682x |
| fen_parse | 0.998x | 0.964x | 1.128x |
| fen_write_native_ep | 1.011x | 0.994x | 2.043x |
| snapshot | 1.000x | 0.999x | 1.330x |
| san_parse_all | 0.912x | 0.887x | 1.680x |
| san_render_native | 1.032x | 1.022x | 2.143x |
| apply_children_full | 1.000x | 0.995x | 1.253x |
| make_unmake_full | 1.002x | 0.991x | 1.200x |
| startpos | 1.251x | 1.251x | 1.253x |
| chess960_materialize | 1.057x | 1.054x | 1.489x |
| chess960_perft_d3 | 1.002x | 0.998x | 1.486x |
| polyglot_key | 0.995x | 0.991x | 38.603x |
| polyglot_recompute | 1.092x | 0.893x | 1.087x |
| repetition_key | 0.977x | 0.948x | 22.429x |
| corpus_count | 0.954x | 0.904x | 1.071x |
| corpus_materialize | 1.021x | 1.017x | 1.086x |
| corpus_uci_sorted | 1.025x | 1.024x | 1.581x |
| replay_trusted_native | 1.025x | 1.006x | 0.906x |
| replay_checked_native | 1.003x | 0.992x | 1.174x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.91x).
