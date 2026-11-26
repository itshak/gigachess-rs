| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.989x | 0.942x | 1.273x |
| materialize | 0.987x | 0.975x | 1.710x |
| perft_bulk_d4 | 1.013x | 0.972x | 1.199x |
| perft_full_d3 | 1.176x | 1.023x | 1.560x |
| fen_parse | 1.005x | 0.993x | 1.056x |
| fen_write_native_ep | 1.019x | 1.004x | 1.846x |
| snapshot | 0.998x | 0.989x | 1.001x |
| san_parse_all | 1.004x | 0.993x | 1.912x |
| san_render_native | 0.986x | 0.949x | 2.173x |
| apply_children_full | 1.003x | 0.997x | 1.382x |
| make_unmake_full | 1.057x | 1.008x | 1.264x |
| startpos | 0.995x | 0.995x | 1.280x |
| chess960_materialize | 1.071x | 1.063x | 1.572x |
| chess960_perft_d3 | 0.981x | 0.961x | 1.475x |
| polyglot_key | 1.071x | 1.003x | 43.916x |
| polyglot_recompute | 1.027x | 1.011x | 1.060x |
| repetition_key | 1.044x | 0.997x | 21.755x |
| corpus_count | 1.114x | 1.061x | 1.264x |
| corpus_materialize | 1.009x | 1.006x | 1.144x |
| corpus_uci_sorted | 1.001x | 0.997x | 1.545x |
| replay_trusted_native | 1.001x | 0.981x | 0.873x |
| replay_checked_native | 1.018x | 1.001x | 1.215x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.16x).
