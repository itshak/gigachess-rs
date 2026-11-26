| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.061x | 0.612x | 1.216x |
| materialize | 1.180x | 0.918x | 1.014x |
| perft_bulk_d4 | 1.124x | 0.978x | 1.040x |
| perft_full_d3 | 1.126x | 0.839x | 1.378x |
| fen_parse | 1.082x | 0.955x | 1.195x |
| fen_write_native_ep | 1.073x | 1.031x | 1.757x |
| snapshot | 1.124x | 0.918x | 0.980x |
| san_parse_all | 1.067x | 0.983x | 1.834x |
| san_render_native | 1.032x | 0.980x | 2.471x |
| apply_children_full | 1.049x | 0.830x | 0.938x |
| make_unmake_full | 1.116x | 0.955x | 1.163x |
| startpos | 0.972x | 0.972x | 1.198x |
| chess960_materialize | 1.154x | 0.958x | 1.489x |
| chess960_perft_d3 | 0.947x | 0.877x | 1.316x |
| polyglot_key | 0.905x | 0.835x | 36.584x |
| polyglot_recompute | 1.009x | 0.986x | 0.999x |
| repetition_key | 1.050x | 0.977x | 19.402x |
| corpus_count | 1.028x | 0.963x | 1.076x |
| corpus_materialize | 1.040x | 1.037x | 0.986x |
| corpus_uci_sorted | 0.966x | 0.955x | 1.255x |
| replay_trusted_native | 1.127x | 1.108x | 0.598x |
| replay_checked_native | 0.988x | 0.849x | 0.905x |

Cold first legal move list: 0.004 ms -> 0.005 ms (0.67x).
