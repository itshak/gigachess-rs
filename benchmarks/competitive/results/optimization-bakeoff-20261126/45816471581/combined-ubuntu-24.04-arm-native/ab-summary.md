| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.392x |
| materialize | 1.002x | 0.990x | 1.244x |
| perft_bulk_d4 | 0.999x | 0.998x | 1.240x |
| perft_full_d3 | 1.018x | 1.011x | 1.089x |
| fen_parse | 0.994x | 0.991x | 1.234x |
| fen_write_native_ep | 1.000x | 0.997x | 1.585x |
| snapshot | 0.992x | 0.987x | 0.945x |
| san_parse_all | 0.999x | 0.995x | 1.817x |
| san_render_native | 1.002x | 0.995x | 2.534x |
| apply_children_full | 1.002x | 0.991x | 1.183x |
| make_unmake_full | 1.007x | 1.004x | 1.490x |
| startpos | 0.999x | 0.999x | 1.307x |
| chess960_materialize | 1.007x | 1.002x | 1.391x |
| chess960_perft_d3 | 1.009x | 1.006x | 1.349x |
| polyglot_key | 0.986x | 0.985x | 25.501x |
| polyglot_recompute | 0.995x | 0.991x | 1.073x |
| repetition_key | 1.000x | 0.999x | 18.470x |
| corpus_count | 0.990x | 0.984x | 1.186x |
| corpus_materialize | 0.997x | 0.994x | 1.157x |
| corpus_uci_sorted | 0.999x | 0.998x | 1.447x |
| replay_trusted_native | 0.995x | 0.989x | 0.548x |
| replay_checked_native | 1.002x | 1.000x | 1.025x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.99x).
