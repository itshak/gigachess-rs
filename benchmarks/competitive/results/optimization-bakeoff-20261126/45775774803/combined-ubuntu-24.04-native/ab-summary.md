| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.999x | 1.364x |
| materialize | 1.006x | 1.002x | 1.465x |
| perft_bulk_d4 | 1.006x | 0.997x | 1.181x |
| perft_full_d3 | 0.972x | 0.967x | 0.863x |
| fen_parse | 0.888x | 0.845x | 0.968x |
| fen_write_native_ep | 1.002x | 0.996x | 1.413x |
| snapshot | 1.052x | 1.051x | 1.108x |
| san_parse_all | 1.032x | 0.995x | 1.756x |
| san_render_native | 1.000x | 0.989x | 2.091x |
| apply_children_full | 1.214x | 1.163x | 0.989x |
| make_unmake_full | 0.998x | 0.995x | 0.935x |
| startpos | 0.894x | 0.894x | 1.080x |
| chess960_materialize | 1.005x | 1.002x | 1.351x |
| chess960_perft_d3 | 1.002x | 1.001x | 1.316x |
| polyglot_key | 0.941x | 0.887x | 38.072x |
| polyglot_recompute | 1.000x | 0.997x | 0.980x |
| repetition_key | 1.227x | 1.019x | 29.713x |
| corpus_count | 1.002x | 0.999x | 1.279x |
| corpus_materialize | 1.002x | 1.002x | 1.086x |
| corpus_uci_sorted | 0.996x | 0.996x | 0.975x |
| replay_trusted_native | 1.009x | 1.005x | 0.609x |
| replay_checked_native | 1.090x | 1.082x | 1.054x |

Cold first legal move list: 0.010 ms -> 0.009 ms (1.03x).
