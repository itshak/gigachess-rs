| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.951x | 0.723x | 1.244x |
| materialize | 0.973x | 0.947x | 1.165x |
| perft_bulk_d4 | 0.883x | 0.818x | 1.120x |
| perft_full_d3 | 0.871x | 0.785x | 0.975x |
| fen_parse | 0.933x | 0.906x | 1.360x |
| fen_write_native_ep | 1.019x | 0.982x | 1.276x |
| snapshot | 0.296x | 0.286x | 0.989x |
| san_parse_all | 0.904x | 0.874x | 1.807x |
| san_render_native | 0.891x | 0.830x | 3.401x |
| apply_children_full | 0.694x | 0.636x | 0.883x |
| make_unmake_full | 0.900x | 0.858x | 1.107x |
| startpos | 0.973x | 0.973x | 3.464x |
| chess960_materialize | 1.116x | 1.022x | 1.191x |
| chess960_perft_d3 | 0.928x | 0.801x | 1.178x |
| polyglot_key | 0.945x | 0.908x | 34.938x |
| polyglot_recompute | 0.988x | 0.980x | 0.984x |
| repetition_key | 0.569x | 0.487x | 14.274x |
| corpus_count | 0.975x | 0.960x | 1.222x |
| corpus_materialize | 1.017x | 0.987x | 1.272x |
| corpus_uci_sorted | 1.103x | 1.091x | 1.514x |
| replay_trusted_native | 0.915x | 0.879x | 0.668x |
| replay_checked_native | 1.015x | 0.986x | 0.946x |

Cold first legal move list: 0.023 ms -> 0.025 ms (0.90x).
