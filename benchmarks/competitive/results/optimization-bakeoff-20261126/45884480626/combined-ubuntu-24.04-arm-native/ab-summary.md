| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.978x | 1.562x |
| materialize | 1.005x | 0.992x | 1.337x |
| perft_bulk_d4 | 1.012x | 1.005x | 1.409x |
| perft_full_d3 | 1.153x | 1.034x | 1.541x |
| fen_parse | 0.994x | 0.990x | 1.208x |
| fen_write_native_ep | 1.001x | 0.998x | 1.573x |
| snapshot | 1.010x | 1.001x | 0.941x |
| san_parse_all | 0.999x | 0.994x | 1.828x |
| san_render_native | 1.004x | 0.993x | 2.693x |
| apply_children_full | 1.001x | 0.985x | 1.160x |
| make_unmake_full | 1.001x | 0.993x | 1.557x |
| startpos | 1.001x | 1.001x | 1.279x |
| chess960_materialize | 1.003x | 0.999x | 1.506x |
| chess960_perft_d3 | 1.000x | 0.998x | 1.501x |
| polyglot_key | 1.006x | 0.999x | 25.951x |
| polyglot_recompute | 0.999x | 0.998x | 1.090x |
| repetition_key | 0.993x | 0.988x | 19.266x |
| corpus_count | 1.017x | 1.012x | 1.367x |
| corpus_materialize | 1.001x | 0.995x | 1.235x |
| corpus_uci_sorted | 1.001x | 0.996x | 1.492x |
| replay_trusted_native | 1.000x | 0.998x | 0.628x |
| replay_checked_native | 1.008x | 1.005x | 1.132x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.01x).
