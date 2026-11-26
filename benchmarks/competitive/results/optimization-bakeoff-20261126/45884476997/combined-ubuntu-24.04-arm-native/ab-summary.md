| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.550x |
| materialize | 1.001x | 0.993x | 1.323x |
| perft_bulk_d4 | 1.000x | 0.992x | 1.396x |
| perft_full_d3 | 1.121x | 1.007x | 1.496x |
| fen_parse | 1.006x | 1.000x | 1.232x |
| fen_write_native_ep | 0.995x | 0.987x | 1.568x |
| snapshot | 1.007x | 1.005x | 0.945x |
| san_parse_all | 0.991x | 0.981x | 1.820x |
| san_render_native | 1.032x | 0.998x | 2.756x |
| apply_children_full | 1.003x | 0.976x | 1.157x |
| make_unmake_full | 1.017x | 0.986x | 1.581x |
| startpos | 0.999x | 0.999x | 1.303x |
| chess960_materialize | 0.992x | 0.989x | 1.481x |
| chess960_perft_d3 | 0.995x | 0.986x | 1.488x |
| polyglot_key | 0.993x | 0.993x | 25.470x |
| polyglot_recompute | 1.048x | 1.032x | 1.141x |
| repetition_key | 1.012x | 1.000x | 19.292x |
| corpus_count | 1.013x | 1.004x | 1.345x |
| corpus_materialize | 1.006x | 1.000x | 1.228x |
| corpus_uci_sorted | 0.998x | 0.997x | 1.456x |
| replay_trusted_native | 0.907x | 0.890x | 0.582x |
| replay_checked_native | 0.970x | 0.964x | 1.107x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.87x).
