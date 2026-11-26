| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.992x | 1.482x |
| materialize | 0.991x | 0.979x | 1.585x |
| perft_bulk_d4 | 0.995x | 0.977x | 1.257x |
| perft_full_d3 | 0.988x | 0.979x | 1.503x |
| fen_parse | 1.099x | 1.066x | 1.171x |
| fen_write_native_ep | 0.956x | 0.898x | 1.862x |
| snapshot | 0.996x | 0.982x | 1.157x |
| san_parse_all | 0.998x | 0.962x | 1.902x |
| san_render_native | 0.997x | 0.994x | 2.169x |
| apply_children_full | 0.980x | 0.946x | 1.071x |
| make_unmake_full | 0.984x | 0.942x | 1.135x |
| startpos | 1.000x | 1.000x | 1.283x |
| chess960_materialize | 0.928x | 0.924x | 1.382x |
| chess960_perft_d3 | 0.998x | 0.998x | 1.383x |
| polyglot_key | 1.004x | 1.004x | 37.968x |
| polyglot_recompute | 1.002x | 0.992x | 1.045x |
| repetition_key | 1.139x | 0.972x | 22.246x |
| corpus_count | 1.006x | 1.002x | 1.402x |
| corpus_materialize | 0.985x | 0.980x | 1.095x |
| corpus_uci_sorted | 0.989x | 0.987x | 1.581x |
| replay_trusted_native | 0.929x | 0.883x | 0.651x |
| replay_checked_native | 0.987x | 0.964x | 1.056x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.02x).
