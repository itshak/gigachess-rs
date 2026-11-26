| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.296x |
| materialize | 0.996x | 0.990x | 1.195x |
| perft_bulk_d4 | 1.001x | 0.997x | 1.293x |
| perft_full_d3 | 0.992x | 0.980x | 0.759x |
| fen_parse | 1.000x | 0.998x | 1.224x |
| fen_write_native_ep | 1.004x | 1.002x | 1.045x |
| snapshot | 1.001x | 1.000x | 0.922x |
| san_parse_all | 1.002x | 1.000x | 1.764x |
| san_render_native | 1.003x | 0.997x | 2.405x |
| apply_children_full | 1.028x | 1.004x | 0.844x |
| make_unmake_full | 0.988x | 0.963x | 1.123x |
| startpos | 1.039x | 1.039x | 1.147x |
| chess960_materialize | 0.996x | 0.994x | 1.228x |
| chess960_perft_d3 | 0.995x | 0.991x | 1.230x |
| polyglot_key | 1.017x | 1.002x | 25.536x |
| polyglot_recompute | 1.003x | 1.000x | 1.071x |
| repetition_key | 1.013x | 0.970x | 18.858x |
| corpus_count | 0.989x | 0.980x | 1.379x |
| corpus_materialize | 0.996x | 0.993x | 1.190x |
| corpus_uci_sorted | 0.997x | 0.995x | 0.973x |
| replay_trusted_native | 0.985x | 0.948x | 0.401x |
| replay_checked_native | 1.022x | 1.018x | 0.903x |

Cold first legal move list: 0.013 ms -> 0.013 ms (1.01x).
