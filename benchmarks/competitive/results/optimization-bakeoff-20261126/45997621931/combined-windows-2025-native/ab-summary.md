| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.920x | 1.473x |
| materialize | 1.023x | 0.989x | 1.248x |
| perft_bulk_d4 | 1.024x | 0.922x | 1.327x |
| perft_full_d3 | 1.049x | 1.019x | 1.330x |
| fen_parse | 1.124x | 1.030x | 1.334x |
| fen_write_native_ep | 1.007x | 0.971x | 1.546x |
| snapshot | 1.006x | 0.917x | 0.979x |
| san_parse_all | 1.055x | 0.969x | 2.086x |
| san_render_native | 1.045x | 1.001x | 3.918x |
| apply_children_full | 1.036x | 0.960x | 1.401x |
| make_unmake_full | 0.957x | 0.881x | 1.310x |
| startpos | 0.994x | 0.994x | 1.243x |
| chess960_materialize | 1.218x | 1.072x | 1.372x |
| chess960_perft_d3 | 1.049x | 1.021x | 1.534x |
| polyglot_key | 0.978x | 0.911x | 26.165x |
| polyglot_recompute | 1.000x | 0.996x | 1.017x |
| repetition_key | 1.032x | 0.929x | 20.187x |
| corpus_count | 1.001x | 0.991x | 1.354x |
| corpus_materialize | 1.032x | 0.993x | 1.135x |
| corpus_uci_sorted | 0.980x | 0.978x | 1.410x |
| replay_trusted_native | 0.985x | 0.977x | 0.824x |
| replay_checked_native | 0.968x | 0.962x | 1.081x |

Cold first legal move list: 0.015 ms -> 0.015 ms (0.97x).
