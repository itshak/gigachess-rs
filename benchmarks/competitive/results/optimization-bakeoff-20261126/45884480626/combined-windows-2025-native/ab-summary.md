| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.034x | 0.969x | 1.549x |
| materialize | 0.978x | 0.944x | 1.118x |
| perft_bulk_d4 | 1.021x | 1.000x | 1.450x |
| perft_full_d3 | 1.317x | 1.085x | 1.701x |
| fen_parse | 0.970x | 0.951x | 1.330x |
| fen_write_native_ep | 0.963x | 0.946x | 1.527x |
| snapshot | 1.001x | 0.992x | 1.115x |
| san_parse_all | 1.015x | 0.981x | 1.977x |
| san_render_native | 1.020x | 0.989x | 3.371x |
| apply_children_full | 1.005x | 0.992x | 1.336x |
| make_unmake_full | 0.993x | 0.956x | 1.291x |
| startpos | 1.001x | 1.001x | 1.001x |
| chess960_materialize | 0.992x | 0.991x | 1.339x |
| chess960_perft_d3 | 1.008x | 0.985x | 1.624x |
| polyglot_key | 1.008x | 0.991x | 20.396x |
| polyglot_recompute | 1.019x | 1.006x | 1.010x |
| repetition_key | 1.003x | 1.002x | 19.119x |
| corpus_count | 0.991x | 0.986x | 1.308x |
| corpus_materialize | 0.998x | 0.993x | 1.207x |
| corpus_uci_sorted | 0.983x | 0.967x | 1.498x |
| replay_trusted_native | 1.029x | 1.003x | 1.144x |
| replay_checked_native | 0.998x | 0.993x | 1.116x |

Cold first legal move list: 0.016 ms -> 0.016 ms (1.01x).
