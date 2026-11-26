| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.355x |
| materialize | 1.003x | 1.001x | 1.259x |
| perft_bulk_d4 | 0.997x | 0.994x | 1.351x |
| perft_full_d3 | 1.005x | 1.002x | 1.074x |
| fen_parse | 0.996x | 0.992x | 1.228x |
| fen_write_native_ep | 1.000x | 0.995x | 1.599x |
| snapshot | 1.016x | 1.001x | 0.922x |
| san_parse_all | 1.000x | 0.998x | 1.853x |
| san_render_native | 1.000x | 0.996x | 2.541x |
| apply_children_full | 1.011x | 0.992x | 1.062x |
| make_unmake_full | 1.021x | 0.994x | 1.468x |
| startpos | 1.073x | 1.073x | 1.185x |
| chess960_materialize | 0.990x | 0.987x | 1.319x |
| chess960_perft_d3 | 0.999x | 0.997x | 1.318x |
| polyglot_key | 1.009x | 0.991x | 25.636x |
| polyglot_recompute | 1.001x | 0.999x | 1.071x |
| repetition_key | 0.986x | 0.968x | 18.703x |
| corpus_count | 1.014x | 1.014x | 1.448x |
| corpus_materialize | 0.997x | 0.994x | 1.218x |
| corpus_uci_sorted | 0.999x | 0.996x | 1.477x |
| replay_trusted_native | 0.979x | 0.949x | 0.487x |
| replay_checked_native | 0.967x | 0.965x | 0.961x |

Cold first legal move list: 0.011 ms -> 0.011 ms (0.96x).
