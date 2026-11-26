| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.551x |
| materialize | 1.005x | 0.999x | 1.327x |
| perft_bulk_d4 | 1.001x | 0.997x | 1.391x |
| perft_full_d3 | 1.144x | 1.023x | 1.523x |
| fen_parse | 1.000x | 0.994x | 1.226x |
| fen_write_native_ep | 1.002x | 0.999x | 1.569x |
| snapshot | 0.992x | 0.988x | 0.945x |
| san_parse_all | 0.995x | 0.986x | 1.836x |
| san_render_native | 0.998x | 0.980x | 2.714x |
| apply_children_full | 0.997x | 0.988x | 1.156x |
| make_unmake_full | 1.000x | 0.984x | 1.551x |
| startpos | 0.999x | 0.999x | 1.297x |
| chess960_materialize | 1.007x | 1.003x | 1.500x |
| chess960_perft_d3 | 1.011x | 1.010x | 1.505x |
| polyglot_key | 0.990x | 0.983x | 25.094x |
| polyglot_recompute | 0.994x | 0.988x | 1.082x |
| repetition_key | 0.996x | 0.993x | 19.359x |
| corpus_count | 1.003x | 1.003x | 1.378x |
| corpus_materialize | 1.000x | 1.000x | 1.231x |
| corpus_uci_sorted | 0.996x | 0.994x | 1.451x |
| replay_trusted_native | 0.995x | 0.990x | 0.641x |
| replay_checked_native | 0.996x | 0.991x | 1.122x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.89x).
