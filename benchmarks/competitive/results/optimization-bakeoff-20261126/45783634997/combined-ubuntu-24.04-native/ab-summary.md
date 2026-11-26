| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.989x | 1.305x |
| materialize | 0.993x | 0.979x | 1.120x |
| perft_bulk_d4 | 1.007x | 0.973x | 1.215x |
| perft_full_d3 | 1.161x | 1.135x | 1.100x |
| fen_parse | 0.976x | 0.961x | 1.190x |
| fen_write_native_ep | 1.318x | 1.251x | 1.467x |
| snapshot | 0.984x | 0.957x | 0.959x |
| san_parse_all | 0.992x | 0.979x | 1.746x |
| san_render_native | 1.049x | 0.987x | 2.328x |
| apply_children_full | 1.210x | 1.191x | 1.144x |
| make_unmake_full | 1.016x | 0.999x | 1.266x |
| startpos | 1.005x | 1.005x | 1.241x |
| chess960_materialize | 0.999x | 0.990x | 1.348x |
| chess960_perft_d3 | 0.992x | 0.982x | 1.340x |
| polyglot_key | 1.059x | 1.001x | 39.156x |
| polyglot_recompute | 1.014x | 1.006x | 1.033x |
| repetition_key | 1.022x | 1.019x | 29.680x |
| corpus_count | 0.992x | 0.987x | 1.189x |
| corpus_materialize | 0.960x | 0.950x | 1.116x |
| corpus_uci_sorted | 1.015x | 1.013x | 1.660x |
| replay_trusted_native | 1.030x | 1.022x | 0.639x |
| replay_checked_native | 1.007x | 0.996x | 0.878x |

Cold first legal move list: 0.014 ms -> 0.014 ms (1.04x).
