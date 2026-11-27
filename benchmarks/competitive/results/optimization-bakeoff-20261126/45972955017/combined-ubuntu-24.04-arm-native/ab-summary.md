| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.997x | 1.548x |
| materialize | 0.997x | 0.991x | 1.318x |
| perft_bulk_d4 | 0.997x | 0.994x | 1.393x |
| perft_full_d3 | 0.999x | 0.995x | 1.332x |
| fen_parse | 1.006x | 1.002x | 1.228x |
| fen_write_native_ep | 0.998x | 0.995x | 1.568x |
| snapshot | 1.021x | 1.017x | 0.938x |
| san_parse_all | 1.002x | 0.993x | 1.844x |
| san_render_native | 1.004x | 0.994x | 2.694x |
| apply_children_full | 1.009x | 0.986x | 1.164x |
| make_unmake_full | 0.995x | 0.986x | 1.551x |
| startpos | 0.998x | 0.998x | 1.275x |
| chess960_materialize | 0.997x | 0.994x | 1.492x |
| chess960_perft_d3 | 1.009x | 1.008x | 1.514x |
| polyglot_key | 0.977x | 0.956x | 25.122x |
| polyglot_recompute | 0.998x | 0.995x | 1.093x |
| repetition_key | 0.983x | 0.964x | 18.933x |
| corpus_count | 0.992x | 0.984x | 1.350x |
| corpus_materialize | 0.988x | 0.984x | 1.216x |
| corpus_uci_sorted | 0.999x | 0.994x | 1.463x |
| replay_trusted_native | 1.008x | 1.003x | 0.628x |
| replay_checked_native | 1.003x | 0.998x | 1.135x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.97x).
