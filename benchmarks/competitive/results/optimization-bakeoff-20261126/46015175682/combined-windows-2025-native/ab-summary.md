| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.014x | 0.993x | 1.564x |
| materialize | 1.011x | 0.976x | 1.147x |
| perft_bulk_d4 | 1.044x | 0.970x | 1.465x |
| perft_full_d3 | 1.334x | 1.076x | 1.698x |
| fen_parse | 1.024x | 0.960x | 1.339x |
| fen_write_native_ep | 1.011x | 1.000x | 1.589x |
| snapshot | 1.004x | 0.996x | 1.158x |
| san_parse_all | 0.997x | 0.985x | 1.966x |
| san_render_native | 1.017x | 0.989x | 3.343x |
| apply_children_full | 1.056x | 1.026x | 1.356x |
| make_unmake_full | 1.065x | 1.026x | 1.321x |
| startpos | 1.000x | 1.000x | 1.000x |
| chess960_materialize | 1.027x | 1.007x | 1.373x |
| chess960_perft_d3 | 0.995x | 0.945x | 1.553x |
| polyglot_key | 0.995x | 0.994x | 20.200x |
| polyglot_recompute | 1.310x | 1.123x | 1.223x |
| repetition_key | 0.996x | 0.987x | 19.259x |
| corpus_count | 1.109x | 1.073x | 1.395x |
| corpus_materialize | 1.033x | 1.023x | 1.185x |
| corpus_uci_sorted | 1.043x | 1.039x | 1.496x |
| replay_trusted_native | 1.071x | 1.037x | 1.152x |
| replay_checked_native | 1.053x | 1.019x | 1.096x |

Cold first legal move list: 0.013 ms -> 0.012 ms (1.07x).
