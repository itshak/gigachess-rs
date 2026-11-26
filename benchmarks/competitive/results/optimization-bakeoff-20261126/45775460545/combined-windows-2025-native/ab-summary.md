| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.969x | 1.244x |
| materialize | 0.970x | 0.933x | 1.130x |
| perft_bulk_d4 | 1.005x | 0.991x | 1.308x |
| perft_full_d3 | 1.076x | 0.981x | 0.755x |
| fen_parse | 0.965x | 0.936x | 1.139x |
| fen_write_native_ep | 0.991x | 0.956x | 0.991x |
| snapshot | 1.000x | 0.986x | 1.038x |
| san_parse_all | 1.077x | 1.060x | 1.959x |
| san_render_native | 1.037x | 1.010x | 3.729x |
| apply_children_full | 1.243x | 1.165x | 1.031x |
| make_unmake_full | 1.075x | 1.055x | 1.065x |
| startpos | 1.151x | 1.151x | 1.109x |
| chess960_materialize | 0.743x | 0.596x | 1.087x |
| chess960_perft_d3 | 0.798x | 0.767x | 1.085x |
| polyglot_key | 1.092x | 0.966x | 26.665x |
| polyglot_recompute | 0.998x | 0.991x | 1.048x |
| repetition_key | 1.071x | 1.063x | 20.700x |
| corpus_count | 1.068x | 1.015x | 1.321x |
| corpus_materialize | 0.977x | 0.972x | 1.134x |
| corpus_uci_sorted | 1.000x | 0.998x | 0.961x |
| replay_trusted_native | 1.181x | 1.119x | 0.610x |
| replay_checked_native | 1.088x | 1.072x | 0.978x |

Cold first legal move list: 0.026 ms -> 0.027 ms (0.99x).
