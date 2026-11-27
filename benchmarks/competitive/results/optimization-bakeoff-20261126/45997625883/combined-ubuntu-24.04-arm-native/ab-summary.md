| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.997x | 1.552x |
| materialize | 0.999x | 0.996x | 1.317x |
| perft_bulk_d4 | 1.003x | 1.000x | 1.399x |
| perft_full_d3 | 0.998x | 0.995x | 1.330x |
| fen_parse | 1.000x | 0.994x | 1.230x |
| fen_write_native_ep | 0.999x | 0.996x | 1.588x |
| snapshot | 1.011x | 1.008x | 0.936x |
| san_parse_all | 0.988x | 0.985x | 1.812x |
| san_render_native | 1.008x | 1.002x | 2.713x |
| apply_children_full | 1.004x | 0.997x | 1.165x |
| make_unmake_full | 1.002x | 0.996x | 1.553x |
| startpos | 1.000x | 1.000x | 1.278x |
| chess960_materialize | 1.007x | 1.000x | 1.495x |
| chess960_perft_d3 | 1.000x | 0.998x | 1.498x |
| polyglot_key | 1.006x | 0.998x | 25.294x |
| polyglot_recompute | 0.998x | 0.996x | 1.089x |
| repetition_key | 1.000x | 0.999x | 18.934x |
| corpus_count | 1.006x | 1.000x | 1.386x |
| corpus_materialize | 1.003x | 0.999x | 1.225x |
| corpus_uci_sorted | 0.993x | 0.992x | 1.454x |
| replay_trusted_native | 0.985x | 0.977x | 0.626x |
| replay_checked_native | 1.013x | 1.008x | 1.137x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.15x).
