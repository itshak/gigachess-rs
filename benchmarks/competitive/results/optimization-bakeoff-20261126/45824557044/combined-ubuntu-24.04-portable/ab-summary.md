| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.045x | 0.993x | 1.195x |
| materialize | 0.992x | 0.941x | 1.249x |
| perft_bulk_d4 | 1.009x | 0.976x | 1.159x |
| perft_full_d3 | 0.996x | 0.973x | 1.331x |
| fen_parse | 1.055x | 1.048x | 1.220x |
| fen_write_native_ep | 0.998x | 0.993x | 1.709x |
| snapshot | 0.998x | 0.994x | 1.024x |
| san_parse_all | 1.017x | 0.991x | 2.078x |
| san_render_native | 0.997x | 0.895x | 2.250x |
| apply_children_full | 0.989x | 0.951x | 1.313x |
| make_unmake_full | 0.996x | 0.978x | 1.340x |
| startpos | 0.969x | 0.969x | 1.267x |
| chess960_materialize | 1.001x | 1.000x | 1.451x |
| chess960_perft_d3 | 0.996x | 0.995x | 1.451x |
| polyglot_key | 1.002x | 0.997x | 23.240x |
| polyglot_recompute | 1.004x | 1.000x | 1.034x |
| repetition_key | 1.053x | 0.978x | 18.297x |
| corpus_count | 1.003x | 1.002x | 1.224x |
| corpus_materialize | 1.002x | 0.999x | 1.130x |
| corpus_uci_sorted | 0.985x | 0.980x | 1.762x |
| replay_trusted_native | 1.006x | 1.003x | 0.847x |
| replay_checked_native | 1.008x | 1.004x | 1.049x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.01x).
