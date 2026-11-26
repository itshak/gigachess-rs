| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.983x | 1.307x |
| materialize | 1.000x | 0.928x | 1.186x |
| perft_bulk_d4 | 1.003x | 0.977x | 1.209x |
| perft_full_d3 | 0.829x | 0.730x | 1.053x |
| fen_parse | 1.041x | 0.969x | 1.281x |
| fen_write_native_ep | 1.039x | 1.008x | 1.600x |
| snapshot | 1.018x | 0.996x | 1.024x |
| san_parse_all | 1.049x | 1.017x | 2.088x |
| san_render_native | 0.909x | 0.846x | 3.487x |
| apply_children_full | 0.884x | 0.784x | 1.064x |
| make_unmake_full | 0.932x | 0.888x | 1.128x |
| startpos | 0.999x | 0.999x | 1.252x |
| chess960_materialize | 0.973x | 0.862x | 1.195x |
| chess960_perft_d3 | 1.055x | 1.032x | 1.370x |
| polyglot_key | 1.052x | 1.018x | 26.036x |
| polyglot_recompute | 0.984x | 0.918x | 1.027x |
| repetition_key | 1.059x | 1.021x | 21.334x |
| corpus_count | 0.971x | 0.948x | 1.156x |
| corpus_materialize | 1.006x | 0.994x | 1.072x |
| corpus_uci_sorted | 1.042x | 1.011x | 1.426x |
| replay_trusted_native | 0.923x | 0.913x | 0.654x |
| replay_checked_native | 0.947x | 0.946x | 0.924x |

Cold first legal move list: 0.018 ms -> 0.018 ms (1.04x).
