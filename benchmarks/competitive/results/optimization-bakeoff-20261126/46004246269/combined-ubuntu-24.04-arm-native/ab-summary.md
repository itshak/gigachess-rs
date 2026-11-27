| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.984x | 1.564x |
| materialize | 1.015x | 0.997x | 1.346x |
| perft_bulk_d4 | 1.043x | 1.019x | 1.453x |
| perft_full_d3 | 1.132x | 1.002x | 1.507x |
| fen_parse | 0.994x | 0.990x | 1.224x |
| fen_write_native_ep | 0.998x | 0.995x | 1.589x |
| snapshot | 0.995x | 0.992x | 0.948x |
| san_parse_all | 1.000x | 0.996x | 1.815x |
| san_render_native | 0.996x | 0.990x | 2.670x |
| apply_children_full | 0.995x | 0.985x | 1.158x |
| make_unmake_full | 0.996x | 0.982x | 1.550x |
| startpos | 0.999x | 0.999x | 1.301x |
| chess960_materialize | 1.010x | 1.009x | 1.501x |
| chess960_perft_d3 | 1.017x | 1.002x | 1.526x |
| polyglot_key | 0.997x | 0.992x | 25.681x |
| polyglot_recompute | 0.991x | 0.985x | 1.082x |
| repetition_key | 0.998x | 0.998x | 19.175x |
| corpus_count | 1.073x | 1.029x | 1.483x |
| corpus_materialize | 0.994x | 0.985x | 1.228x |
| corpus_uci_sorted | 1.000x | 0.995x | 1.464x |
| replay_trusted_native | 1.007x | 0.999x | 0.638x |
| replay_checked_native | 0.981x | 0.979x | 1.106x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.11x).
