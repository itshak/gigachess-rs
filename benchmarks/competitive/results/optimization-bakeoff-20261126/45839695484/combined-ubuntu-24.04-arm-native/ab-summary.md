| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.989x | 1.541x |
| materialize | 0.997x | 0.982x | 1.336x |
| perft_bulk_d4 | 0.995x | 0.982x | 1.389x |
| perft_full_d3 | 0.997x | 0.991x | 0.835x |
| fen_parse | 1.004x | 0.996x | 1.202x |
| fen_write_native_ep | 1.005x | 1.001x | 1.585x |
| snapshot | 1.018x | 1.013x | 0.937x |
| san_parse_all | 1.002x | 0.992x | 1.831x |
| san_render_native | 0.997x | 0.986x | 2.523x |
| apply_children_full | 1.004x | 0.991x | 1.155x |
| make_unmake_full | 1.001x | 0.995x | 1.548x |
| startpos | 0.999x | 0.999x | 1.273x |
| chess960_materialize | 1.004x | 0.998x | 1.503x |
| chess960_perft_d3 | 0.993x | 0.992x | 1.489x |
| polyglot_key | 0.996x | 0.987x | 25.299x |
| polyglot_recompute | 0.999x | 0.997x | 1.087x |
| repetition_key | 1.006x | 1.001x | 19.084x |
| corpus_count | 0.989x | 0.981x | 1.365x |
| corpus_materialize | 1.009x | 1.007x | 1.246x |
| corpus_uci_sorted | 1.002x | 0.999x | 1.468x |
| replay_trusted_native | 0.999x | 0.994x | 0.614x |
| replay_checked_native | 1.002x | 0.999x | 1.046x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.97x).
