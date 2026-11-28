| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.057x | 0.999x | 1.351x |
| materialize | 0.999x | 0.964x | 1.735x |
| perft_bulk_d4 | 1.072x | 1.052x | 1.270x |
| perft_full_d3 | 1.139x | 1.026x | 1.527x |
| fen_parse | 0.938x | 0.765x | 1.073x |
| fen_write_native_ep | 0.996x | 0.970x | 1.959x |
| snapshot | 1.002x | 0.996x | 1.001x |
| san_parse_all | 0.991x | 0.983x | 1.891x |
| san_render_native | 0.986x | 0.952x | 2.106x |
| apply_children_full | 0.979x | 0.886x | 1.334x |
| make_unmake_full | 0.939x | 0.858x | 1.193x |
| startpos | 1.001x | 1.001x | 1.353x |
| chess960_materialize | 0.981x | 0.978x | 1.448x |
| chess960_perft_d3 | 1.023x | 1.011x | 1.530x |
| polyglot_key | 1.036x | 0.998x | 44.000x |
| polyglot_recompute | 1.635x | 1.310x | 1.634x |
| repetition_key | 1.083x | 0.990x | 26.370x |
| corpus_count | 1.138x | 1.108x | 1.293x |
| corpus_materialize | 1.005x | 1.000x | 1.154x |
| corpus_uci_sorted | 1.000x | 0.999x | 1.541x |
| replay_trusted_native | 0.940x | 0.905x | 0.864x |
| replay_checked_native | 0.897x | 0.850x | 1.051x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.12x).
