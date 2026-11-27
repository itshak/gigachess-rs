| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.997x | 1.461x |
| materialize | 1.015x | 0.991x | 1.286x |
| perft_bulk_d4 | 1.014x | 0.987x | 1.355x |
| perft_full_d3 | 1.024x | 1.003x | 1.318x |
| fen_parse | 0.986x | 0.974x | 1.069x |
| fen_write_native_ep | 0.994x | 0.982x | 1.709x |
| snapshot | 1.000x | 0.995x | 1.248x |
| san_parse_all | 0.996x | 0.989x | 2.060x |
| san_render_native | 1.025x | 1.011x | 2.233x |
| apply_children_full | 0.999x | 0.980x | 1.320x |
| make_unmake_full | 1.043x | 1.004x | 1.331x |
| startpos | 1.000x | 1.000x | 1.250x |
| chess960_materialize | 0.999x | 0.988x | 1.405x |
| chess960_perft_d3 | 0.998x | 0.994x | 1.541x |
| polyglot_key | 0.997x | 0.996x | 25.636x |
| polyglot_recompute | 1.000x | 0.998x | 1.025x |
| repetition_key | 0.991x | 0.986x | 19.501x |
| corpus_count | 1.006x | 1.001x | 1.367x |
| corpus_materialize | 1.000x | 0.997x | 1.136x |
| corpus_uci_sorted | 0.995x | 0.987x | 1.724x |
| replay_trusted_native | 0.951x | 0.942x | 0.847x |
| replay_checked_native | 0.981x | 0.955x | 1.062x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.99x).
