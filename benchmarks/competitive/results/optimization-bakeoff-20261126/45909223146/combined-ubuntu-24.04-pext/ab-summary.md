| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.027x | 0.998x | 1.608x |
| materialize | 1.040x | 1.014x | 1.593x |
| perft_bulk_d4 | 1.048x | 1.027x | 1.245x |
| perft_full_d3 | 0.992x | 0.980x | 1.331x |
| fen_parse | 0.938x | 0.837x | 1.077x |
| fen_write_native_ep | 0.968x | 0.929x | 1.825x |
| snapshot | 0.999x | 0.993x | 0.998x |
| san_parse_all | 1.021x | 0.995x | 2.011x |
| san_render_native | 0.985x | 0.980x | 2.096x |
| apply_children_full | 0.998x | 0.995x | 1.351x |
| make_unmake_full | 0.994x | 0.978x | 1.191x |
| startpos | 0.897x | 0.897x | 0.997x |
| chess960_materialize | 0.996x | 0.971x | 1.511x |
| chess960_perft_d3 | 1.006x | 0.992x | 1.475x |
| polyglot_key | 0.988x | 0.973x | 40.141x |
| polyglot_recompute | 1.023x | 1.018x | 1.065x |
| repetition_key | 0.819x | 0.693x | 20.122x |
| corpus_count | 1.188x | 1.121x | 1.328x |
| corpus_materialize | 1.009x | 1.005x | 1.091x |
| corpus_uci_sorted | 0.996x | 0.995x | 1.525x |
| replay_trusted_native | 0.960x | 0.920x | 0.850x |
| replay_checked_native | 1.010x | 1.003x | 1.175x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.14x).
