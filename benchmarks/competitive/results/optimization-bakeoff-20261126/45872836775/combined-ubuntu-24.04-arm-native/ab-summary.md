| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.555x |
| materialize | 1.007x | 1.004x | 1.324x |
| perft_bulk_d4 | 0.999x | 0.996x | 1.401x |
| perft_full_d3 | 1.001x | 1.000x | 1.335x |
| fen_parse | 0.986x | 0.972x | 1.211x |
| fen_write_native_ep | 0.999x | 0.996x | 1.569x |
| snapshot | 0.991x | 0.990x | 0.949x |
| san_parse_all | 0.994x | 0.990x | 1.843x |
| san_render_native | 1.004x | 0.995x | 2.669x |
| apply_children_full | 0.996x | 0.987x | 1.157x |
| make_unmake_full | 1.008x | 1.002x | 1.564x |
| startpos | 0.999x | 0.999x | 1.305x |
| chess960_materialize | 1.002x | 0.999x | 1.500x |
| chess960_perft_d3 | 1.001x | 0.999x | 1.498x |
| polyglot_key | 0.983x | 0.972x | 25.665x |
| polyglot_recompute | 0.926x | 0.918x | 1.008x |
| repetition_key | 0.995x | 0.990x | 19.565x |
| corpus_count | 1.004x | 1.003x | 1.344x |
| corpus_materialize | 1.000x | 0.999x | 1.225x |
| corpus_uci_sorted | 1.000x | 0.999x | 1.461x |
| replay_trusted_native | 0.996x | 0.994x | 0.625x |
| replay_checked_native | 1.013x | 1.007x | 1.139x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.12x).
