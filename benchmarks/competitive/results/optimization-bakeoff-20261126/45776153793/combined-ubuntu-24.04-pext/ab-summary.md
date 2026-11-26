| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.985x | 1.337x |
| materialize | 1.009x | 0.979x | 1.242x |
| perft_bulk_d4 | 0.931x | 0.913x | 1.261x |
| perft_full_d3 | 0.877x | 0.848x | 0.701x |
| fen_parse | 0.827x | 0.779x | 0.979x |
| fen_write_native_ep | 1.001x | 0.991x | 0.960x |
| snapshot | 0.965x | 0.957x | 0.857x |
| san_parse_all | 0.875x | 0.798x | 1.715x |
| san_render_native | 0.916x | 0.870x | 2.019x |
| apply_children_full | 0.836x | 0.810x | 0.776x |
| make_unmake_full | 0.850x | 0.833x | 0.857x |
| startpos | 0.812x | 0.812x | 1.000x |
| chess960_materialize | 0.983x | 0.976x | 1.284x |
| chess960_perft_d3 | 0.930x | 0.908x | 1.216x |
| polyglot_key | 0.975x | 0.949x | 25.176x |
| polyglot_recompute | 0.974x | 0.971x | 1.010x |
| repetition_key | 0.898x | 0.851x | 17.752x |
| corpus_count | 0.986x | 0.976x | 1.426x |
| corpus_materialize | 0.992x | 0.989x | 1.197x |
| corpus_uci_sorted | 1.006x | 1.005x | 1.024x |
| replay_trusted_native | 0.878x | 0.843x | 0.525x |
| replay_checked_native | 0.798x | 0.713x | 0.751x |

Cold first legal move list: 0.014 ms -> 0.015 ms (0.96x).
