| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.472x |
| materialize | 0.997x | 0.989x | 1.546x |
| perft_bulk_d4 | 1.001x | 0.999x | 1.261x |
| perft_full_d3 | 0.977x | 0.921x | 1.490x |
| fen_parse | 1.000x | 0.999x | 1.072x |
| fen_write_native_ep | 0.998x | 0.994x | 1.228x |
| snapshot | 1.294x | 1.291x | 1.154x |
| san_parse_all | 1.000x | 0.996x | 1.872x |
| san_render_native | 1.019x | 1.015x | 2.115x |
| apply_children_full | 1.029x | 1.017x | 1.063x |
| make_unmake_full | 1.015x | 0.997x | 1.157x |
| startpos | 1.099x | 1.099x | 1.119x |
| chess960_materialize | 0.993x | 0.987x | 1.455x |
| chess960_perft_d3 | 1.004x | 1.003x | 1.398x |
| polyglot_key | 1.008x | 1.002x | 41.520x |
| polyglot_recompute | 0.997x | 0.992x | 1.046x |
| repetition_key | 1.059x | 1.033x | 24.044x |
| corpus_count | 0.999x | 0.997x | 1.376x |
| corpus_materialize | 1.002x | 1.001x | 1.113x |
| corpus_uci_sorted | 0.993x | 0.991x | 1.552x |
| replay_trusted_native | 1.007x | 0.999x | 0.713x |
| replay_checked_native | 0.988x | 0.985x | 1.093x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.08x).
