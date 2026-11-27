| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.064x | 1.006x | 1.587x |
| materialize | 1.029x | 1.010x | 1.298x |
| perft_bulk_d4 | 1.059x | 1.037x | 1.438x |
| perft_full_d3 | 1.223x | 1.036x | 1.622x |
| fen_parse | 1.024x | 1.007x | 1.317x |
| fen_write_native_ep | 0.951x | 0.922x | 1.453x |
| snapshot | 0.994x | 0.983x | 0.992x |
| san_parse_all | 0.999x | 0.987x | 2.061x |
| san_render_native | 1.004x | 0.989x | 3.897x |
| apply_children_full | 1.003x | 0.987x | 1.421x |
| make_unmake_full | 1.008x | 0.979x | 1.367x |
| startpos | 1.001x | 1.001x | 1.336x |
| chess960_materialize | 1.034x | 1.030x | 1.364x |
| chess960_perft_d3 | 1.001x | 0.979x | 1.540x |
| polyglot_key | 0.994x | 0.993x | 29.227x |
| polyglot_recompute | 1.009x | 1.006x | 1.013x |
| repetition_key | 1.048x | 1.030x | 22.013x |
| corpus_count | 1.086x | 1.060x | 1.476x |
| corpus_materialize | 0.999x | 0.995x | 1.201x |
| corpus_uci_sorted | 1.003x | 0.999x | 1.410x |
| replay_trusted_native | 1.002x | 0.997x | 0.854x |
| replay_checked_native | 1.010x | 0.998x | 1.086x |

Cold first legal move list: 0.019 ms -> 0.017 ms (1.10x).
