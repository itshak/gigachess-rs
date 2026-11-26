| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.933x | 1.376x |
| materialize | 0.996x | 0.978x | 1.207x |
| perft_bulk_d4 | 0.983x | 0.964x | 1.323x |
| perft_full_d3 | 0.984x | 0.958x | 1.170x |
| fen_parse | 0.845x | 0.820x | 1.040x |
| fen_write_native_ep | 1.006x | 0.988x | 0.918x |
| snapshot | 1.335x | 1.319x | 1.249x |
| san_parse_all | 1.028x | 1.010x | 2.092x |
| san_render_native | 0.970x | 0.938x | 2.144x |
| apply_children_full | 1.013x | 0.990x | 1.193x |
| make_unmake_full | 0.995x | 0.973x | 1.216x |
| startpos | 0.926x | 0.926x | 1.073x |
| chess960_materialize | 1.002x | 1.001x | 1.271x |
| chess960_perft_d3 | 1.005x | 1.002x | 1.386x |
| polyglot_key | 0.992x | 0.982x | 29.623x |
| polyglot_recompute | 1.007x | 1.005x | 1.043x |
| repetition_key | 1.031x | 0.995x | 22.791x |
| corpus_count | 1.013x | 0.999x | 1.481x |
| corpus_materialize | 1.008x | 1.008x | 1.205x |
| corpus_uci_sorted | 0.998x | 0.995x | 1.784x |
| replay_trusted_native | 1.021x | 0.998x | 0.726x |
| replay_checked_native | 0.978x | 0.978x | 0.978x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.04x).
