| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.949x | 1.402x |
| materialize | 1.024x | 0.992x | 1.345x |
| perft_bulk_d4 | 0.996x | 0.960x | 1.160x |
| perft_full_d3 | 1.657x | 1.490x | 1.388x |
| fen_parse | 1.103x | 1.052x | 1.192x |
| fen_write_native_ep | 1.011x | 0.950x | 1.316x |
| snapshot | 2.645x | 2.528x | 1.009x |
| san_parse_all | 1.008x | 0.903x | 1.588x |
| san_render_native | 1.010x | 0.966x | 3.926x |
| apply_children_full | 1.007x | 0.965x | 0.954x |
| make_unmake_full | 0.989x | 0.973x | 1.002x |
| startpos | 0.952x | 0.952x | 1.210x |
| chess960_materialize | 1.017x | 1.007x | 1.262x |
| chess960_perft_d3 | 1.061x | 1.013x | 1.360x |
| polyglot_key | 1.318x | 1.308x | 41.265x |
| polyglot_recompute | 1.011x | 0.984x | 1.020x |
| repetition_key | 0.960x | 0.775x | 24.640x |
| corpus_count | 1.049x | 0.976x | 1.313x |
| corpus_materialize | 1.018x | 1.004x | 1.097x |
| corpus_uci_sorted | 1.021x | 0.978x | 0.984x |
| replay_trusted_native | 0.969x | 0.933x | 0.577x |
| replay_checked_native | 1.077x | 0.974x | 0.962x |

Cold first legal move list: 0.035 ms -> 0.029 ms (1.20x).
