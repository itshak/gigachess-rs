| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.995x | 1.289x |
| materialize | 0.999x | 0.995x | 1.198x |
| perft_bulk_d4 | 1.014x | 1.002x | 1.294x |
| perft_full_d3 | 0.991x | 0.981x | 0.760x |
| fen_parse | 0.994x | 0.988x | 1.231x |
| fen_write_native_ep | 0.987x | 0.979x | 1.077x |
| snapshot | 1.028x | 1.021x | 0.882x |
| san_parse_all | 1.000x | 0.995x | 1.732x |
| san_render_native | 0.994x | 0.983x | 2.376x |
| apply_children_full | 1.157x | 1.124x | 0.966x |
| make_unmake_full | 1.015x | 0.995x | 1.170x |
| startpos | 1.073x | 1.073x | 1.181x |
| chess960_materialize | 1.000x | 0.995x | 1.231x |
| chess960_perft_d3 | 1.006x | 1.002x | 1.232x |
| polyglot_key | 0.997x | 0.990x | 25.787x |
| polyglot_recompute | 1.002x | 0.997x | 1.073x |
| repetition_key | 1.010x | 0.989x | 19.022x |
| corpus_count | 0.995x | 0.990x | 1.415x |
| corpus_materialize | 0.999x | 0.998x | 1.201x |
| corpus_uci_sorted | 1.005x | 1.005x | 0.972x |
| replay_trusted_native | 1.037x | 1.020x | 0.422x |
| replay_checked_native | 0.992x | 0.991x | 0.894x |

Cold first legal move list: 0.012 ms -> 0.012 ms (1.01x).
