| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.054x | 1.013x | 1.580x |
| materialize | 1.018x | 0.991x | 1.597x |
| perft_bulk_d4 | 1.067x | 1.053x | 1.251x |
| perft_full_d3 | 1.238x | 1.051x | 1.671x |
| fen_parse | 0.924x | 0.829x | 1.040x |
| fen_write_native_ep | 0.971x | 0.947x | 1.842x |
| snapshot | 1.001x | 0.992x | 1.325x |
| san_parse_all | 1.013x | 0.994x | 1.934x |
| san_render_native | 0.984x | 0.981x | 2.066x |
| apply_children_full | 1.001x | 0.999x | 1.325x |
| make_unmake_full | 0.996x | 0.994x | 1.191x |
| startpos | 0.888x | 0.888x | 1.007x |
| chess960_materialize | 0.987x | 0.983x | 1.436x |
| chess960_perft_d3 | 1.016x | 0.998x | 1.533x |
| polyglot_key | 1.073x | 0.999x | 39.413x |
| polyglot_recompute | 1.020x | 1.018x | 1.014x |
| repetition_key | 1.005x | 0.991x | 18.816x |
| corpus_count | 1.231x | 1.135x | 1.343x |
| corpus_materialize | 1.032x | 1.031x | 1.083x |
| corpus_uci_sorted | 0.985x | 0.981x | 1.542x |
| replay_trusted_native | 1.043x | 1.022x | 0.917x |
| replay_checked_native | 1.008x | 0.993x | 1.147x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.04x).
