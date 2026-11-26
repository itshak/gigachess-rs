| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.977x | 1.144x |
| materialize | 0.973x | 0.945x | 1.402x |
| perft_bulk_d4 | 0.980x | 0.963x | 1.145x |
| perft_full_d3 | 0.978x | 0.962x | 1.340x |
| fen_parse | 0.891x | 0.813x | 1.069x |
| fen_write_native_ep | 1.038x | 1.000x | 1.877x |
| snapshot | 1.016x | 1.014x | 0.991x |
| san_parse_all | 0.985x | 0.969x | 1.884x |
| san_render_native | 0.914x | 0.896x | 2.298x |
| apply_children_full | 0.932x | 0.892x | 1.073x |
| make_unmake_full | 0.937x | 0.895x | 1.283x |
| startpos | 1.001x | 1.001x | 1.287x |
| chess960_materialize | 1.006x | 1.004x | 1.319x |
| chess960_perft_d3 | 0.990x | 0.987x | 1.283x |
| polyglot_key | 1.006x | 1.002x | 31.082x |
| polyglot_recompute | 1.003x | 1.001x | 1.098x |
| repetition_key | 0.992x | 0.992x | 20.428x |
| corpus_count | 1.003x | 0.999x | 1.231x |
| corpus_materialize | 1.025x | 1.021x | 1.182x |
| corpus_uci_sorted | 1.001x | 0.993x | 1.519x |
| replay_trusted_native | 0.974x | 0.925x | 0.666x |
| replay_checked_native | 0.952x | 0.927x | 0.982x |

Cold first legal move list: 0.012 ms -> 0.011 ms (1.05x).
