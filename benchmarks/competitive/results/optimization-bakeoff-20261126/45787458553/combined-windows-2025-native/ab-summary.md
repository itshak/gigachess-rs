| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.990x | 1.359x |
| materialize | 1.000x | 0.981x | 1.098x |
| perft_bulk_d4 | 0.939x | 0.912x | 1.238x |
| perft_full_d3 | 0.969x | 0.914x | 1.001x |
| fen_parse | 1.038x | 1.015x | 1.245x |
| fen_write_native_ep | 1.011x | 0.998x | 1.600x |
| snapshot | 0.837x | 0.826x | 1.039x |
| san_parse_all | 0.955x | 0.943x | 2.060x |
| san_render_native | 0.934x | 0.904x | 3.131x |
| apply_children_full | 0.966x | 0.935x | 1.022x |
| make_unmake_full | 0.927x | 0.885x | 1.142x |
| startpos | 1.000x | 1.000x | 1.082x |
| chess960_materialize | 1.010x | 1.003x | 1.233x |
| chess960_perft_d3 | 0.955x | 0.951x | 1.355x |
| polyglot_key | 0.980x | 0.959x | 20.671x |
| polyglot_recompute | 0.973x | 0.963x | 0.964x |
| repetition_key | 0.941x | 0.868x | 16.584x |
| corpus_count | 1.001x | 0.995x | 1.328x |
| corpus_materialize | 0.982x | 0.965x | 1.147x |
| corpus_uci_sorted | 1.026x | 0.998x | 1.512x |
| replay_trusted_native | 1.110x | 1.098x | 0.773x |
| replay_checked_native | 0.932x | 0.924x | 0.900x |

Cold first legal move list: 0.026 ms -> 0.020 ms (1.35x).
