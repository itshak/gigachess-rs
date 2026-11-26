| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.051x | 0.983x | 1.431x |
| materialize | 1.019x | 0.982x | 1.267x |
| perft_bulk_d4 | 1.006x | 0.973x | 1.355x |
| perft_full_d3 | 1.001x | 0.988x | 1.065x |
| fen_parse | 0.991x | 0.987x | 1.222x |
| fen_write_native_ep | 0.995x | 0.995x | 1.572x |
| snapshot | 1.000x | 0.996x | 1.012x |
| san_parse_all | 1.000x | 0.996x | 1.835x |
| san_render_native | 0.996x | 0.986x | 2.547x |
| apply_children_full | 0.981x | 0.960x | 1.039x |
| make_unmake_full | 0.994x | 0.976x | 1.427x |
| startpos | 0.963x | 0.963x | 1.209x |
| chess960_materialize | 1.078x | 0.993x | 1.426x |
| chess960_perft_d3 | 1.051x | 1.007x | 1.388x |
| polyglot_key | 0.994x | 0.992x | 24.886x |
| polyglot_recompute | 1.000x | 1.000x | 1.073x |
| repetition_key | 0.992x | 0.984x | 18.839x |
| corpus_count | 1.020x | 1.012x | 1.512x |
| corpus_materialize | 1.007x | 1.005x | 1.208x |
| corpus_uci_sorted | 1.001x | 1.000x | 1.462x |
| replay_trusted_native | 1.003x | 1.000x | 0.502x |
| replay_checked_native | 0.999x | 0.993x | 0.990x |

Cold first legal move list: 0.011 ms -> 0.009 ms (1.32x).
