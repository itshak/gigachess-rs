| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.980x | 0.957x | 1.519x |
| materialize | 1.004x | 0.999x | 1.340x |
| perft_bulk_d4 | 1.026x | 1.004x | 1.437x |
| perft_full_d3 | 1.000x | 0.986x | 1.332x |
| fen_parse | 1.003x | 0.999x | 1.228x |
| fen_write_native_ep | 1.001x | 0.999x | 1.631x |
| snapshot | 0.990x | 0.982x | 0.958x |
| san_parse_all | 0.998x | 0.991x | 1.833x |
| san_render_native | 1.004x | 0.991x | 2.692x |
| apply_children_full | 0.998x | 0.991x | 1.152x |
| make_unmake_full | 1.003x | 0.988x | 1.564x |
| startpos | 1.000x | 1.000x | 1.330x |
| chess960_materialize | 1.008x | 1.006x | 1.503x |
| chess960_perft_d3 | 1.005x | 1.000x | 1.504x |
| polyglot_key | 1.027x | 1.005x | 25.211x |
| polyglot_recompute | 0.997x | 0.997x | 1.087x |
| repetition_key | 1.015x | 0.996x | 18.933x |
| corpus_count | 1.104x | 1.086x | 1.464x |
| corpus_materialize | 1.006x | 0.999x | 1.243x |
| corpus_uci_sorted | 1.008x | 1.005x | 1.461x |
| replay_trusted_native | 1.008x | 1.002x | 0.631x |
| replay_checked_native | 1.011x | 1.007x | 1.143x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.03x).
