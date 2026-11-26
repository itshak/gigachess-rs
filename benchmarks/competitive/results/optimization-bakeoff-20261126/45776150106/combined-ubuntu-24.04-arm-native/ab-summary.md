| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.995x | 1.289x |
| materialize | 0.998x | 0.992x | 1.197x |
| perft_bulk_d4 | 0.970x | 0.957x | 1.236x |
| perft_full_d3 | 0.864x | 0.834x | 0.646x |
| fen_parse | 0.960x | 0.941x | 1.192x |
| fen_write_native_ep | 0.981x | 0.973x | 1.018x |
| snapshot | 0.890x | 0.879x | 0.868x |
| san_parse_all | 0.999x | 0.963x | 1.745x |
| san_render_native | 0.915x | 0.873x | 2.193x |
| apply_children_full | 0.891x | 0.883x | 0.737x |
| make_unmake_full | 0.812x | 0.784x | 0.943x |
| startpos | 0.501x | 0.501x | 0.606x |
| chess960_materialize | 0.994x | 0.992x | 1.226x |
| chess960_perft_d3 | 0.957x | 0.950x | 1.172x |
| polyglot_key | 1.121x | 1.000x | 28.564x |
| polyglot_recompute | 1.004x | 0.996x | 1.078x |
| repetition_key | 1.043x | 0.913x | 19.253x |
| corpus_count | 0.987x | 0.977x | 1.397x |
| corpus_materialize | 1.003x | 0.995x | 1.188x |
| corpus_uci_sorted | 1.003x | 1.003x | 0.971x |
| replay_trusted_native | 0.970x | 0.936x | 0.409x |
| replay_checked_native | 0.930x | 0.925x | 0.838x |

Cold first legal move list: 0.013 ms -> 0.011 ms (1.10x).
