| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.948x | 1.454x |
| materialize | 1.025x | 0.939x | 1.235x |
| perft_bulk_d4 | 1.016x | 0.969x | 1.341x |
| perft_full_d3 | 1.027x | 0.988x | 1.312x |
| fen_parse | 1.063x | 1.003x | 1.274x |
| fen_write_native_ep | 0.978x | 0.942x | 1.513x |
| snapshot | 1.002x | 0.991x | 1.000x |
| san_parse_all | 1.000x | 0.988x | 2.067x |
| san_render_native | 1.020x | 0.994x | 3.763x |
| apply_children_full | 1.020x | 0.948x | 1.346x |
| make_unmake_full | 0.972x | 0.876x | 1.291x |
| startpos | 0.993x | 0.993x | 1.242x |
| chess960_materialize | 1.008x | 0.992x | 1.374x |
| chess960_perft_d3 | 0.994x | 0.991x | 1.527x |
| polyglot_key | 0.943x | 0.899x | 25.823x |
| polyglot_recompute | 0.993x | 0.989x | 1.042x |
| repetition_key | 0.959x | 0.933x | 20.323x |
| corpus_count | 0.981x | 0.968x | 1.427x |
| corpus_materialize | 0.973x | 0.955x | 1.148x |
| corpus_uci_sorted | 0.952x | 0.932x | 1.430x |
| replay_trusted_native | 0.963x | 0.953x | 0.802x |
| replay_checked_native | 0.894x | 0.889x | 0.936x |

Cold first legal move list: 0.017 ms -> 0.019 ms (0.89x).
