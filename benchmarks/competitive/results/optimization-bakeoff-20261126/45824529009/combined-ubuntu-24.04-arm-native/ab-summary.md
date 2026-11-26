| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.999x | 1.379x |
| materialize | 1.000x | 0.997x | 1.250x |
| perft_bulk_d4 | 1.009x | 1.007x | 1.251x |
| perft_full_d3 | 1.000x | 0.995x | 1.069x |
| fen_parse | 0.996x | 0.991x | 1.220x |
| fen_write_native_ep | 0.998x | 0.997x | 1.563x |
| snapshot | 1.010x | 1.009x | 0.946x |
| san_parse_all | 1.004x | 1.000x | 1.827x |
| san_render_native | 0.999x | 0.993x | 2.516x |
| apply_children_full | 0.986x | 0.967x | 1.267x |
| make_unmake_full | 0.979x | 0.957x | 1.474x |
| startpos | 1.000x | 1.000x | 1.304x |
| chess960_materialize | 0.994x | 0.993x | 1.388x |
| chess960_perft_d3 | 1.000x | 0.994x | 1.348x |
| polyglot_key | 0.976x | 0.962x | 25.026x |
| polyglot_recompute | 0.999x | 0.993x | 1.065x |
| repetition_key | 0.990x | 0.981x | 18.142x |
| corpus_count | 0.985x | 0.972x | 1.200x |
| corpus_materialize | 0.989x | 0.981x | 1.164x |
| corpus_uci_sorted | 0.998x | 0.997x | 1.455x |
| replay_trusted_native | 1.029x | 1.025x | 0.622x |
| replay_checked_native | 0.964x | 0.953x | 1.023x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.01x).
