| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.986x | 1.544x |
| materialize | 0.991x | 0.984x | 1.319x |
| perft_bulk_d4 | 0.998x | 0.978x | 1.388x |
| perft_full_d3 | 0.996x | 0.989x | 1.333x |
| fen_parse | 0.991x | 0.988x | 1.220x |
| fen_write_native_ep | 0.997x | 0.991x | 1.576x |
| snapshot | 1.008x | 1.005x | 0.935x |
| san_parse_all | 0.998x | 0.992x | 1.825x |
| san_render_native | 1.002x | 0.998x | 2.684x |
| apply_children_full | 1.004x | 1.001x | 1.163x |
| make_unmake_full | 0.994x | 0.983x | 1.544x |
| startpos | 0.998x | 0.998x | 1.266x |
| chess960_materialize | 0.986x | 0.985x | 1.464x |
| chess960_perft_d3 | 0.996x | 0.983x | 1.507x |
| polyglot_key | 1.011x | 0.998x | 25.649x |
| polyglot_recompute | 0.991x | 0.989x | 1.078x |
| repetition_key | 1.008x | 0.999x | 19.236x |
| corpus_count | 1.001x | 0.999x | 1.359x |
| corpus_materialize | 0.998x | 0.996x | 1.231x |
| corpus_uci_sorted | 0.999x | 0.997x | 1.460x |
| replay_trusted_native | 1.003x | 1.000x | 0.638x |
| replay_checked_native | 1.005x | 1.001x | 1.135x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.01x).
