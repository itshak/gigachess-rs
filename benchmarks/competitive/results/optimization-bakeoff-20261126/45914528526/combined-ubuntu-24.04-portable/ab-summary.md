| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.995x | 1.290x |
| materialize | 1.002x | 0.993x | 1.344x |
| perft_bulk_d4 | 0.992x | 0.976x | 1.281x |
| perft_full_d3 | 0.999x | 0.990x | 1.291x |
| fen_parse | 0.950x | 0.872x | 1.289x |
| fen_write_native_ep | 0.994x | 0.978x | 1.710x |
| snapshot | 0.999x | 0.996x | 0.979x |
| san_parse_all | 0.982x | 0.970x | 2.158x |
| san_render_native | 1.004x | 0.982x | 2.253x |
| apply_children_full | 0.976x | 0.936x | 1.382x |
| make_unmake_full | 0.960x | 0.904x | 1.327x |
| startpos | 1.001x | 1.001x | 1.336x |
| chess960_materialize | 1.057x | 1.046x | 1.672x |
| chess960_perft_d3 | 1.000x | 1.000x | 1.589x |
| polyglot_key | 0.989x | 0.979x | 21.346x |
| polyglot_recompute | 1.007x | 1.000x | 0.975x |
| repetition_key | 0.986x | 0.970x | 19.757x |
| corpus_count | 1.004x | 1.002x | 1.227x |
| corpus_materialize | 1.017x | 0.998x | 1.380x |
| corpus_uci_sorted | 1.015x | 1.008x | 1.619x |
| replay_trusted_native | 1.021x | 0.999x | 1.103x |
| replay_checked_native | 0.927x | 0.902x | 1.066x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.26x).
