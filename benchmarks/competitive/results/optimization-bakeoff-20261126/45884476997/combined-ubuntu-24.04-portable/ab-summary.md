| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.959x | 1.281x |
| materialize | 0.928x | 0.889x | 1.325x |
| perft_bulk_d4 | 0.982x | 0.964x | 1.255x |
| perft_full_d3 | 1.210x | 1.018x | 1.570x |
| fen_parse | 1.184x | 1.102x | 1.263x |
| fen_write_native_ep | 0.996x | 0.981x | 1.771x |
| snapshot | 1.001x | 1.000x | 0.980x |
| san_parse_all | 0.987x | 0.981x | 2.179x |
| san_render_native | 0.988x | 0.913x | 2.241x |
| apply_children_full | 1.023x | 1.001x | 1.448x |
| make_unmake_full | 1.007x | 0.987x | 1.391x |
| startpos | 1.002x | 1.002x | 1.334x |
| chess960_materialize | 1.082x | 1.063x | 1.692x |
| chess960_perft_d3 | 1.015x | 1.002x | 1.603x |
| polyglot_key | 0.992x | 0.979x | 21.496x |
| polyglot_recompute | 1.062x | 0.975x | 1.033x |
| repetition_key | 1.003x | 0.993x | 20.081x |
| corpus_count | 0.998x | 0.997x | 1.225x |
| corpus_materialize | 0.998x | 0.996x | 1.351x |
| corpus_uci_sorted | 1.016x | 1.015x | 1.607x |
| replay_trusted_native | 0.802x | 0.607x | 0.863x |
| replay_checked_native | 0.972x | 0.934x | 1.110x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.83x).
