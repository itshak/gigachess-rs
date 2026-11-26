| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.050x | 0.857x | 1.138x |
| materialize | 1.004x | 0.870x | 1.183x |
| perft_bulk_d4 | 1.050x | 0.956x | 0.932x |
| perft_full_d3 | 0.998x | 0.983x | 0.600x |
| fen_parse | 1.017x | 1.010x | 1.216x |
| fen_write_native_ep | 1.004x | 0.996x | 1.061x |
| snapshot | 0.994x | 0.978x | 0.751x |
| san_parse_all | 1.000x | 0.994x | 0.915x |
| san_render_native | 1.000x | 0.997x | 1.143x |
| apply_children_full | 1.000x | 0.990x | 0.657x |
| make_unmake_full | 0.991x | 0.967x | 0.891x |
| startpos | 0.988x | 0.988x | 0.782x |
| chess960_materialize | 0.989x | 0.985x | 1.093x |
| chess960_perft_d3 | 1.003x | 1.003x | 0.841x |
| polyglot_key | 1.007x | 0.996x | 23.486x |
| polyglot_recompute | 1.000x | 0.991x | 1.131x |
| repetition_key | 1.013x | 0.997x | 14.227x |
| corpus_count | 0.946x | 0.922x | 1.154x |
| corpus_materialize | 0.994x | 0.992x | 1.117x |
| corpus_uci_sorted | 0.997x | 0.995x | 0.565x |
| replay_trusted_native | 1.000x | 0.998x | 0.315x |
| replay_checked_native | 0.999x | 0.998x | 0.651x |

Cold first legal move list: 154.176 ms → 154.295 ms (1.00x).
