| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.061x | 0.997x | 1.372x |
| materialize | 1.010x | 0.996x | 1.374x |
| perft_bulk_d4 | 1.050x | 1.025x | 1.344x |
| perft_full_d3 | 1.232x | 1.040x | 1.597x |
| fen_parse | 0.848x | 0.782x | 1.119x |
| fen_write_native_ep | 1.025x | 0.995x | 1.738x |
| snapshot | 0.998x | 0.993x | 0.974x |
| san_parse_all | 0.995x | 0.984x | 2.241x |
| san_render_native | 1.018x | 0.972x | 2.296x |
| apply_children_full | 1.009x | 0.965x | 1.421x |
| make_unmake_full | 1.024x | 0.974x | 1.420x |
| startpos | 1.000x | 1.000x | 1.335x |
| chess960_materialize | 1.009x | 1.003x | 1.633x |
| chess960_perft_d3 | 0.984x | 0.973x | 1.560x |
| polyglot_key | 1.005x | 0.989x | 21.866x |
| polyglot_recompute | 1.019x | 1.017x | 0.993x |
| repetition_key | 1.001x | 0.997x | 20.111x |
| corpus_count | 1.084x | 1.078x | 1.320x |
| corpus_materialize | 0.997x | 0.993x | 1.373x |
| corpus_uci_sorted | 1.008x | 1.000x | 1.587x |
| replay_trusted_native | 1.000x | 0.984x | 1.094x |
| replay_checked_native | 1.017x | 0.996x | 1.177x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.97x).
