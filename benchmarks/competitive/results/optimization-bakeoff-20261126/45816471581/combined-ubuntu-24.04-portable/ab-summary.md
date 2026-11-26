| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.021x | 0.994x | 1.205x |
| materialize | 0.996x | 0.986x | 1.599x |
| perft_bulk_d4 | 0.993x | 0.981x | 1.069x |
| perft_full_d3 | 1.006x | 0.990x | 1.451x |
| fen_parse | 0.982x | 0.941x | 1.150x |
| fen_write_native_ep | 0.982x | 0.949x | 2.032x |
| snapshot | 1.004x | 0.995x | 1.004x |
| san_parse_all | 1.073x | 1.041x | 1.857x |
| san_render_native | 1.004x | 0.996x | 2.157x |
| apply_children_full | 0.996x | 0.989x | 1.214x |
| make_unmake_full | 0.995x | 0.990x | 1.157x |
| startpos | 1.004x | 1.004x | 1.364x |
| chess960_materialize | 0.958x | 0.942x | 1.325x |
| chess960_perft_d3 | 0.994x | 0.991x | 1.386x |
| polyglot_key | 0.952x | 0.911x | 42.077x |
| polyglot_recompute | 0.982x | 0.982x | 1.042x |
| repetition_key | 0.961x | 0.928x | 17.363x |
| corpus_count | 0.957x | 0.927x | 1.010x |
| corpus_materialize | 0.998x | 0.996x | 1.084x |
| corpus_uci_sorted | 1.012x | 1.008x | 1.560x |
| replay_trusted_native | 1.019x | 1.004x | 0.829x |
| replay_checked_native | 0.977x | 0.971x | 1.093x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.02x).
