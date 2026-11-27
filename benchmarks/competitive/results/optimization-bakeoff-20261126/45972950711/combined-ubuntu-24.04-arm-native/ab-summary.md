| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.548x |
| materialize | 0.999x | 0.993x | 1.322x |
| perft_bulk_d4 | 1.000x | 0.993x | 1.390x |
| perft_full_d3 | 0.998x | 0.995x | 1.331x |
| fen_parse | 0.998x | 0.991x | 1.223x |
| fen_write_native_ep | 0.998x | 0.995x | 1.586x |
| snapshot | 1.009x | 0.998x | 0.935x |
| san_parse_all | 0.998x | 0.996x | 1.825x |
| san_render_native | 1.000x | 0.995x | 2.711x |
| apply_children_full | 0.946x | 0.894x | 1.096x |
| make_unmake_full | 0.953x | 0.884x | 1.476x |
| startpos | 0.999x | 0.999x | 1.272x |
| chess960_materialize | 1.002x | 0.999x | 1.499x |
| chess960_perft_d3 | 1.003x | 0.999x | 1.496x |
| polyglot_key | 0.977x | 0.959x | 25.031x |
| polyglot_recompute | 0.998x | 0.998x | 1.089x |
| repetition_key | 0.988x | 0.977x | 19.068x |
| corpus_count | 0.994x | 0.989x | 1.368x |
| corpus_materialize | 0.995x | 0.993x | 1.228x |
| corpus_uci_sorted | 0.994x | 0.994x | 1.456x |
| replay_trusted_native | 0.940x | 0.926x | 0.585x |
| replay_checked_native | 0.936x | 0.936x | 1.053x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.04x).
