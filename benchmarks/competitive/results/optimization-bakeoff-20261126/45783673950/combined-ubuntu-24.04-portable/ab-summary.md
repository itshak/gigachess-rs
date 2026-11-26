| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.986x | 1.198x |
| materialize | 0.998x | 0.984x | 1.252x |
| perft_bulk_d4 | 0.996x | 0.975x | 1.212x |
| perft_full_d3 | 0.777x | 0.757x | 1.003x |
| fen_parse | 0.997x | 0.989x | 1.223x |
| fen_write_native_ep | 1.027x | 1.018x | 0.960x |
| snapshot | 1.002x | 0.999x | 1.012x |
| san_parse_all | 1.004x | 0.997x | 2.034x |
| san_render_native | 1.168x | 1.094x | 2.374x |
| apply_children_full | 0.979x | 0.947x | 1.147x |
| make_unmake_full | 1.006x | 0.999x | 1.307x |
| startpos | 0.971x | 0.971x | 1.291x |
| chess960_materialize | 0.988x | 0.974x | 1.356x |
| chess960_perft_d3 | 1.000x | 0.996x | 1.406x |
| polyglot_key | 0.903x | 0.886x | 21.922x |
| polyglot_recompute | 0.997x | 0.993x | 1.041x |
| repetition_key | 1.024x | 0.989x | 17.772x |
| corpus_count | 0.995x | 0.986x | 1.296x |
| corpus_materialize | 0.982x | 0.981x | 1.184x |
| corpus_uci_sorted | 0.993x | 0.992x | 1.802x |
| replay_trusted_native | 0.972x | 0.956x | 0.702x |
| replay_checked_native | 1.000x | 0.995x | 1.055x |

Cold first legal move list: 0.013 ms -> 0.013 ms (1.01x).
