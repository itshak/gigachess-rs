| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.071x | 0.906x | 1.549x |
| materialize | 1.006x | 0.947x | 1.291x |
| perft_bulk_d4 | 1.029x | 0.958x | 1.468x |
| perft_full_d3 | 1.242x | 1.026x | 1.598x |
| fen_parse | 0.873x | 0.842x | 1.069x |
| fen_write_native_ep | 0.995x | 0.973x | 1.889x |
| snapshot | 1.001x | 0.996x | 1.249x |
| san_parse_all | 1.025x | 0.971x | 2.097x |
| san_render_native | 0.999x | 0.987x | 2.217x |
| apply_children_full | 1.007x | 0.978x | 1.325x |
| make_unmake_full | 0.911x | 0.781x | 1.169x |
| startpos | 0.999x | 0.999x | 1.260x |
| chess960_materialize | 0.982x | 0.930x | 1.365x |
| chess960_perft_d3 | 1.005x | 0.982x | 1.509x |
| polyglot_key | 0.987x | 0.935x | 25.436x |
| polyglot_recompute | 0.993x | 0.989x | 1.029x |
| repetition_key | 0.942x | 0.903x | 19.286x |
| corpus_count | 1.068x | 1.067x | 1.517x |
| corpus_materialize | 0.996x | 0.989x | 1.183x |
| corpus_uci_sorted | 0.988x | 0.986x | 1.806x |
| replay_trusted_native | 0.975x | 0.969x | 0.879x |
| replay_checked_native | 0.981x | 0.971x | 1.087x |

Cold first legal move list: 0.009 ms -> 0.007 ms (1.14x).
