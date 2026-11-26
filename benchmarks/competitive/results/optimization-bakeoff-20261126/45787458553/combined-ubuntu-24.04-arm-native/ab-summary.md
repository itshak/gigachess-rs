| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 1.000x | 1.357x |
| materialize | 1.001x | 0.997x | 1.238x |
| perft_bulk_d4 | 0.981x | 0.964x | 1.336x |
| perft_full_d3 | 0.972x | 0.947x | 1.021x |
| fen_parse | 0.997x | 0.991x | 1.239x |
| fen_write_native_ep | 1.010x | 1.007x | 1.563x |
| snapshot | 0.939x | 0.937x | 0.894x |
| san_parse_all | 1.011x | 0.970x | 1.854x |
| san_render_native | 0.899x | 0.846x | 2.289x |
| apply_children_full | 0.988x | 0.977x | 1.035x |
| make_unmake_full | 0.933x | 0.906x | 1.341x |
| startpos | 1.038x | 1.038x | 1.279x |
| chess960_materialize | 0.997x | 0.994x | 1.322x |
| chess960_perft_d3 | 0.988x | 0.983x | 1.318x |
| polyglot_key | 1.006x | 1.002x | 25.962x |
| polyglot_recompute | 0.999x | 0.995x | 1.076x |
| repetition_key | 0.988x | 0.960x | 18.737x |
| corpus_count | 1.012x | 1.000x | 1.502x |
| corpus_materialize | 1.016x | 1.010x | 1.206x |
| corpus_uci_sorted | 1.010x | 1.008x | 1.464x |
| replay_trusted_native | 1.016x | 1.011x | 0.501x |
| replay_checked_native | 0.915x | 0.910x | 0.904x |

Cold first legal move list: 0.011 ms -> 0.010 ms (1.13x).
