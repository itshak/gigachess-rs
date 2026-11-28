| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.051x | 0.966x | 1.375x |
| materialize | 1.017x | 0.991x | 1.370x |
| perft_bulk_d4 | 1.052x | 1.032x | 1.371x |
| perft_full_d3 | 1.178x | 0.990x | 1.496x |
| fen_parse | 1.013x | 0.954x | 1.295x |
| fen_write_native_ep | 1.000x | 0.986x | 1.684x |
| snapshot | 1.000x | 0.998x | 1.006x |
| san_parse_all | 0.999x | 0.985x | 1.935x |
| san_render_native | 1.008x | 0.971x | 2.272x |
| apply_children_full | 1.024x | 1.003x | 1.389x |
| make_unmake_full | 0.992x | 0.955x | 1.402x |
| startpos | 1.002x | 1.002x | 1.322x |
| chess960_materialize | 0.981x | 0.975x | 1.450x |
| chess960_perft_d3 | 1.012x | 1.001x | 1.610x |
| polyglot_key | 1.036x | 0.999x | 25.469x |
| polyglot_recompute | 1.218x | 1.115x | 1.247x |
| repetition_key | 1.070x | 1.005x | 19.614x |
| corpus_count | 1.095x | 1.084x | 1.393x |
| corpus_materialize | 1.001x | 1.000x | 1.224x |
| corpus_uci_sorted | 0.992x | 0.978x | 1.702x |
| replay_trusted_native | 0.942x | 0.899x | 0.859x |
| replay_checked_native | 0.987x | 0.950x | 1.151x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.99x).
