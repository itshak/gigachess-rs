| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.982x | 1.207x |
| materialize | 1.009x | 0.952x | 1.257x |
| perft_bulk_d4 | 1.024x | 1.009x | 1.165x |
| perft_full_d3 | 0.988x | 0.956x | 1.308x |
| fen_parse | 1.043x | 1.012x | 1.164x |
| fen_write_native_ep | 0.970x | 0.938x | 1.622x |
| snapshot | 1.000x | 0.998x | 1.009x |
| san_parse_all | 1.012x | 1.000x | 2.087x |
| san_render_native | 0.996x | 0.951x | 2.268x |
| apply_children_full | 1.013x | 0.989x | 1.385x |
| make_unmake_full | 0.993x | 0.975x | 1.352x |
| startpos | 1.000x | 1.000x | 1.325x |
| chess960_materialize | 0.992x | 0.976x | 1.324x |
| chess960_perft_d3 | 1.011x | 1.009x | 1.456x |
| polyglot_key | 1.019x | 1.019x | 27.619x |
| polyglot_recompute | 1.004x | 1.004x | 1.031x |
| repetition_key | 1.031x | 0.985x | 19.604x |
| corpus_count | 0.977x | 0.975x | 1.179x |
| corpus_materialize | 0.982x | 0.981x | 1.108x |
| corpus_uci_sorted | 1.010x | 1.008x | 1.752x |
| replay_trusted_native | 1.003x | 0.999x | 0.862x |
| replay_checked_native | 1.005x | 1.002x | 1.035x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.07x).
