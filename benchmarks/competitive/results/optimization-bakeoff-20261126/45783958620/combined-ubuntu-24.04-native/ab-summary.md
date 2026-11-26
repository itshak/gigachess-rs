| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.987x | 1.405x |
| materialize | 0.999x | 0.995x | 1.505x |
| perft_bulk_d4 | 0.993x | 0.977x | 1.201x |
| perft_full_d3 | 0.982x | 0.877x | 1.455x |
| fen_parse | 1.132x | 1.106x | 1.159x |
| fen_write_native_ep | 1.001x | 0.992x | 2.072x |
| snapshot | 0.913x | 0.912x | 1.080x |
| san_parse_all | 1.055x | 1.026x | 1.976x |
| san_render_native | 1.027x | 0.994x | 2.145x |
| apply_children_full | 0.961x | 0.923x | 1.018x |
| make_unmake_full | 0.932x | 0.916x | 1.078x |
| startpos | 1.000x | 1.000x | 1.155x |
| chess960_materialize | 0.969x | 0.944x | 1.400x |
| chess960_perft_d3 | 0.991x | 0.980x | 1.365x |
| polyglot_key | 1.031x | 1.001x | 40.274x |
| polyglot_recompute | 1.049x | 1.040x | 1.029x |
| repetition_key | 1.074x | 1.020x | 20.144x |
| corpus_count | 0.996x | 0.990x | 1.328x |
| corpus_materialize | 1.010x | 1.007x | 1.096x |
| corpus_uci_sorted | 1.011x | 1.004x | 1.552x |
| replay_trusted_native | 0.935x | 0.925x | 0.653x |
| replay_checked_native | 0.986x | 0.958x | 1.114x |

Cold first legal move list: 0.009 ms -> 0.012 ms (0.76x).
