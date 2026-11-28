| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.057x | 0.999x | 1.374x |
| materialize | 1.004x | 0.990x | 1.732x |
| perft_bulk_d4 | 1.075x | 1.048x | 1.273x |
| perft_full_d3 | 1.164x | 1.031x | 1.552x |
| fen_parse | 0.956x | 0.913x | 1.169x |
| fen_write_native_ep | 1.007x | 1.000x | 1.949x |
| snapshot | 1.001x | 0.992x | 1.001x |
| san_parse_all | 0.929x | 0.915x | 1.782x |
| san_render_native | 0.980x | 0.966x | 2.156x |
| apply_children_full | 1.010x | 1.002x | 1.382x |
| make_unmake_full | 0.980x | 0.948x | 1.273x |
| startpos | 0.990x | 0.990x | 1.349x |
| chess960_materialize | 0.984x | 0.973x | 1.583x |
| chess960_perft_d3 | 1.012x | 1.001x | 1.514x |
| polyglot_key | 0.978x | 0.955x | 40.459x |
| polyglot_recompute | 1.635x | 1.323x | 1.567x |
| repetition_key | 0.877x | 0.738x | 20.446x |
| corpus_count | 1.142x | 1.107x | 1.343x |
| corpus_materialize | 0.993x | 0.991x | 1.131x |
| corpus_uci_sorted | 0.993x | 0.990x | 1.542x |
| replay_trusted_native | 1.001x | 0.982x | 0.945x |
| replay_checked_native | 1.001x | 1.000x | 1.188x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.10x).
