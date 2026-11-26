| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.996x | 1.294x |
| materialize | 0.979x | 0.955x | 1.359x |
| perft_bulk_d4 | 0.966x | 0.896x | 1.289x |
| perft_full_d3 | 0.896x | 0.738x | 1.110x |
| fen_parse | 0.922x | 0.906x | 1.169x |
| fen_write_native_ep | 0.998x | 0.993x | 1.732x |
| snapshot | 0.999x | 0.996x | 0.987x |
| san_parse_all | 1.005x | 0.993x | 2.071x |
| san_render_native | 1.006x | 0.979x | 2.398x |
| apply_children_full | 1.000x | 0.990x | 1.377x |
| make_unmake_full | 0.977x | 0.962x | 1.358x |
| startpos | 1.001x | 1.001x | 1.290x |
| chess960_materialize | 0.966x | 0.959x | 1.444x |
| chess960_perft_d3 | 0.991x | 0.983x | 1.571x |
| polyglot_key | 1.000x | 0.999x | 27.742x |
| polyglot_recompute | 0.997x | 0.997x | 1.018x |
| repetition_key | 0.963x | 0.925x | 18.903x |
| corpus_count | 1.008x | 1.008x | 1.295x |
| corpus_materialize | 1.008x | 1.006x | 1.198x |
| corpus_uci_sorted | 0.999x | 0.988x | 1.807x |
| replay_trusted_native | 1.000x | 0.997x | 0.850x |
| replay_checked_native | 1.012x | 1.011x | 1.059x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.84x).
