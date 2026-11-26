| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.153x | 1.108x | 1.252x |
| materialize | 1.024x | 0.996x | 1.215x |
| perft_bulk_d4 | 1.082x | 1.070x | 0.956x |
| perft_full_d3 | 1.005x | 1.002x | 0.603x |
| fen_parse | 1.005x | 0.996x | 1.201x |
| fen_write_native_ep | 1.001x | 0.998x | 1.053x |
| snapshot | 0.977x | 0.972x | 0.660x |
| san_parse_all | 0.998x | 0.992x | 0.915x |
| san_render_native | 0.998x | 0.988x | 1.136x |
| apply_children_full | 1.004x | 0.999x | 0.662x |
| make_unmake_full | 1.009x | 1.003x | 0.897x |
| startpos | 0.981x | 0.981x | 0.757x |
| chess960_materialize | 0.998x | 0.971x | 1.107x |
| chess960_perft_d3 | 1.036x | 1.030x | 0.874x |
| polyglot_key | 0.967x | 0.923x | 22.493x |
| polyglot_recompute | 1.004x | 0.999x | 1.134x |
| repetition_key | 0.959x | 0.918x | 13.412x |
| corpus_count | 1.044x | 1.043x | 1.268x |
| corpus_materialize | 1.018x | 1.012x | 1.151x |
| corpus_uci_sorted | 0.992x | 0.989x | 0.567x |
| replay_trusted_native | 1.005x | 1.002x | 0.325x |
| replay_checked_native | 1.006x | 1.000x | 0.662x |

Cold first legal move list: 154.186 ms → 154.062 ms (1.00x).
