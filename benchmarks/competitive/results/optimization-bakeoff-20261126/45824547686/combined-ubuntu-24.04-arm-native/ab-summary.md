| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.987x | 1.376x |
| materialize | 1.004x | 0.996x | 1.240x |
| perft_bulk_d4 | 0.999x | 0.997x | 1.231x |
| perft_full_d3 | 0.999x | 0.997x | 1.065x |
| fen_parse | 0.996x | 0.989x | 1.226x |
| fen_write_native_ep | 0.997x | 0.988x | 1.605x |
| snapshot | 0.990x | 0.988x | 0.947x |
| san_parse_all | 0.998x | 0.990x | 1.828x |
| san_render_native | 0.999x | 0.990x | 2.541x |
| apply_children_full | 0.994x | 0.965x | 1.279x |
| make_unmake_full | 0.995x | 0.989x | 1.498x |
| startpos | 0.998x | 0.998x | 1.290x |
| chess960_materialize | 0.984x | 0.966x | 1.356x |
| chess960_perft_d3 | 0.997x | 0.988x | 1.339x |
| polyglot_key | 0.974x | 0.960x | 25.354x |
| polyglot_recompute | 0.995x | 0.995x | 1.066x |
| repetition_key | 0.989x | 0.980x | 18.370x |
| corpus_count | 1.002x | 1.000x | 1.227x |
| corpus_materialize | 1.000x | 0.999x | 1.169x |
| corpus_uci_sorted | 0.994x | 0.988x | 1.452x |
| replay_trusted_native | 0.982x | 0.974x | 0.594x |
| replay_checked_native | 0.996x | 0.991x | 1.063x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.99x).
