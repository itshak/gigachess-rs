| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 1.000x | 1.357x |
| materialize | 0.991x | 0.984x | 1.227x |
| perft_bulk_d4 | 0.999x | 0.995x | 1.346x |
| perft_full_d3 | 1.003x | 0.999x | 1.068x |
| fen_parse | 0.999x | 0.996x | 1.245x |
| fen_write_native_ep | 0.995x | 0.984x | 1.559x |
| snapshot | 1.014x | 1.008x | 0.912x |
| san_parse_all | 1.003x | 0.998x | 1.850x |
| san_render_native | 0.994x | 0.988x | 2.541x |
| apply_children_full | 1.020x | 0.969x | 1.168x |
| make_unmake_full | 1.031x | 1.018x | 1.493x |
| startpos | 0.999x | 0.999x | 1.184x |
| chess960_materialize | 0.994x | 0.989x | 1.319x |
| chess960_perft_d3 | 1.009x | 1.006x | 1.321x |
| polyglot_key | 1.012x | 0.999x | 25.514x |
| polyglot_recompute | 1.000x | 0.999x | 1.076x |
| repetition_key | 0.993x | 0.985x | 18.886x |
| corpus_count | 0.987x | 0.984x | 1.488x |
| corpus_materialize | 1.012x | 1.012x | 1.217x |
| corpus_uci_sorted | 1.001x | 0.998x | 1.464x |
| replay_trusted_native | 1.042x | 1.021x | 0.557x |
| replay_checked_native | 1.051x | 1.046x | 1.033x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.88x).
