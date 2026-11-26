| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.998x | 1.374x |
| materialize | 0.944x | 0.921x | 1.427x |
| perft_bulk_d4 | 1.003x | 0.993x | 1.178x |
| perft_full_d3 | 0.980x | 0.976x | 0.876x |
| fen_parse | 0.952x | 0.898x | 1.127x |
| fen_write_native_ep | 0.985x | 0.943x | 1.134x |
| snapshot | 0.775x | 0.771x | 1.153x |
| san_parse_all | 1.002x | 0.984x | 1.766x |
| san_render_native | 1.012x | 1.003x | 2.083x |
| apply_children_full | 1.036x | 1.001x | 0.988x |
| make_unmake_full | 1.046x | 1.008x | 0.985x |
| startpos | 1.101x | 1.101x | 1.084x |
| chess960_materialize | 1.000x | 0.999x | 1.382x |
| chess960_perft_d3 | 0.995x | 0.995x | 1.309x |
| polyglot_key | 1.005x | 1.002x | 39.300x |
| polyglot_recompute | 0.994x | 0.989x | 0.962x |
| repetition_key | 0.990x | 0.973x | 21.669x |
| corpus_count | 1.000x | 0.996x | 1.303x |
| corpus_materialize | 1.002x | 1.001x | 1.098x |
| corpus_uci_sorted | 1.003x | 1.001x | 0.966x |
| replay_trusted_native | 1.001x | 0.999x | 0.613x |
| replay_checked_native | 1.007x | 0.998x | 1.004x |

Cold first legal move list: 0.010 ms -> 0.010 ms (1.03x).
