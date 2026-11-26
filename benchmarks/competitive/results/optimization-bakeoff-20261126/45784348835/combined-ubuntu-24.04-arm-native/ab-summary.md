| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.357x |
| materialize | 0.996x | 0.989x | 1.239x |
| perft_bulk_d4 | 1.003x | 0.999x | 1.359x |
| perft_full_d3 | 0.992x | 0.981x | 1.054x |
| fen_parse | 1.007x | 1.004x | 1.246x |
| fen_write_native_ep | 1.000x | 0.997x | 1.576x |
| snapshot | 0.967x | 0.960x | 1.016x |
| san_parse_all | 1.001x | 0.997x | 1.829x |
| san_render_native | 0.998x | 0.988x | 2.563x |
| apply_children_full | 1.027x | 0.987x | 1.075x |
| make_unmake_full | 0.995x | 0.982x | 1.445x |
| startpos | 0.868x | 0.868x | 1.162x |
| chess960_materialize | 1.004x | 1.003x | 1.335x |
| chess960_perft_d3 | 0.997x | 0.996x | 1.316x |
| polyglot_key | 0.980x | 0.968x | 25.108x |
| polyglot_recompute | 0.998x | 0.993x | 1.070x |
| repetition_key | 0.993x | 0.967x | 19.159x |
| corpus_count | 0.997x | 0.991x | 1.471x |
| corpus_materialize | 0.996x | 0.995x | 1.205x |
| corpus_uci_sorted | 0.990x | 0.984x | 1.452x |
| replay_trusted_native | 1.040x | 1.021x | 0.521x |
| replay_checked_native | 0.990x | 0.986x | 0.981x |

Cold first legal move list: 0.010 ms -> 0.011 ms (0.98x).
