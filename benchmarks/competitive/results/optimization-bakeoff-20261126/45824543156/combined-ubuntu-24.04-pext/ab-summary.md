| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.986x | 1.396x |
| materialize | 1.006x | 0.981x | 1.156x |
| perft_bulk_d4 | 1.006x | 0.953x | 1.266x |
| perft_full_d3 | 1.075x | 1.032x | 1.041x |
| fen_parse | 0.991x | 0.969x | 0.990x |
| fen_write_native_ep | 1.009x | 0.982x | 1.824x |
| snapshot | 1.002x | 0.998x | 1.248x |
| san_parse_all | 0.998x | 0.992x | 2.301x |
| san_render_native | 1.030x | 1.002x | 2.129x |
| apply_children_full | 1.034x | 0.974x | 1.245x |
| make_unmake_full | 0.989x | 0.917x | 1.167x |
| startpos | 1.013x | 1.013x | 1.200x |
| chess960_materialize | 1.006x | 1.001x | 1.471x |
| chess960_perft_d3 | 0.995x | 0.986x | 1.476x |
| polyglot_key | 1.008x | 1.007x | 20.839x |
| polyglot_recompute | 0.994x | 0.991x | 1.031x |
| repetition_key | 1.225x | 1.139x | 19.059x |
| corpus_count | 0.999x | 0.998x | 1.245x |
| corpus_materialize | 0.997x | 0.996x | 1.227x |
| corpus_uci_sorted | 0.999x | 0.995x | 1.655x |
| replay_trusted_native | 1.052x | 1.047x | 1.148x |
| replay_checked_native | 1.000x | 0.997x | 1.058x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.00x).
