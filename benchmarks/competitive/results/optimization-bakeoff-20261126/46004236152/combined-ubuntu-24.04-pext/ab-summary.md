| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.040x | 1.003x | 1.616x |
| materialize | 1.084x | 1.044x | 1.626x |
| perft_bulk_d4 | 1.069x | 1.053x | 1.276x |
| perft_full_d3 | 1.236x | 1.045x | 1.873x |
| fen_parse | 0.996x | 0.993x | 1.157x |
| fen_write_native_ep | 0.966x | 0.925x | 1.825x |
| snapshot | 0.998x | 0.986x | 1.152x |
| san_parse_all | 0.998x | 0.993x | 1.837x |
| san_render_native | 0.984x | 0.979x | 2.114x |
| apply_children_full | 1.002x | 0.992x | 1.344x |
| make_unmake_full | 0.957x | 0.918x | 1.153x |
| startpos | 0.901x | 0.901x | 1.132x |
| chess960_materialize | 1.073x | 1.058x | 1.507x |
| chess960_perft_d3 | 1.007x | 0.984x | 1.497x |
| polyglot_key | 1.003x | 0.998x | 39.588x |
| polyglot_recompute | 0.997x | 0.994x | 1.001x |
| repetition_key | 1.011x | 1.011x | 23.974x |
| corpus_count | 1.146x | 1.115x | 1.283x |
| corpus_materialize | 1.017x | 1.012x | 1.090x |
| corpus_uci_sorted | 1.013x | 1.011x | 1.568x |
| replay_trusted_native | 1.019x | 0.986x | 0.932x |
| replay_checked_native | 0.961x | 0.949x | 1.111x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.12x).
