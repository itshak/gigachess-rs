| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.052x | 1.012x | 1.575x |
| materialize | 1.004x | 0.984x | 1.588x |
| perft_bulk_d4 | 1.056x | 1.040x | 1.232x |
| perft_full_d3 | 1.227x | 1.065x | 1.655x |
| fen_parse | 1.004x | 0.999x | 1.069x |
| fen_write_native_ep | 1.002x | 0.996x | 1.863x |
| snapshot | 1.005x | 0.995x | 1.152x |
| san_parse_all | 1.001x | 0.992x | 2.031x |
| san_render_native | 1.000x | 0.993x | 2.139x |
| apply_children_full | 1.007x | 1.000x | 1.347x |
| make_unmake_full | 0.962x | 0.927x | 1.152x |
| startpos | 1.000x | 1.000x | 1.117x |
| chess960_materialize | 0.984x | 0.978x | 1.472x |
| chess960_perft_d3 | 1.009x | 0.982x | 1.513x |
| polyglot_key | 1.013x | 1.000x | 40.109x |
| polyglot_recompute | 1.011x | 1.009x | 1.019x |
| repetition_key | 1.235x | 1.081x | 23.625x |
| corpus_count | 1.200x | 1.113x | 1.298x |
| corpus_materialize | 1.000x | 0.996x | 1.048x |
| corpus_uci_sorted | 0.998x | 0.997x | 1.571x |
| replay_trusted_native | 1.057x | 1.021x | 0.934x |
| replay_checked_native | 1.008x | 0.990x | 1.152x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.24x).
