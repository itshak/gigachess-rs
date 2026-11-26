| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.947x | 0.900x | 1.307x |
| materialize | 0.977x | 0.951x | 1.222x |
| perft_bulk_d4 | 0.967x | 0.949x | 1.206x |
| perft_full_d3 | 0.934x | 0.896x | 0.974x |
| fen_parse | 1.015x | 0.994x | 1.232x |
| fen_write_native_ep | 0.950x | 0.909x | 1.481x |
| snapshot | 1.066x | 1.056x | 1.019x |
| san_parse_all | 0.973x | 0.961x | 1.796x |
| san_render_native | 0.975x | 0.968x | 2.467x |
| apply_children_full | 0.983x | 0.965x | 1.134x |
| make_unmake_full | 0.979x | 0.947x | 1.498x |
| startpos | 0.933x | 0.933x | 1.244x |
| chess960_materialize | 0.979x | 0.965x | 1.358x |
| chess960_perft_d3 | 0.979x | 0.978x | 1.320x |
| polyglot_key | 1.062x | 1.018x | 26.676x |
| polyglot_recompute | 0.991x | 0.985x | 1.077x |
| repetition_key | 1.057x | 1.002x | 20.156x |
| corpus_count | 0.965x | 0.965x | 1.165x |
| corpus_materialize | 0.980x | 0.977x | 1.148x |
| corpus_uci_sorted | 1.024x | 1.023x | 1.483x |
| replay_trusted_native | 0.991x | 0.986x | 0.604x |
| replay_checked_native | 0.979x | 0.976x | 1.028x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.99x).
