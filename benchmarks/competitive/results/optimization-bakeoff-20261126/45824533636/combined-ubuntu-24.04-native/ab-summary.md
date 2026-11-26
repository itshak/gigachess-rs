| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.972x | 1.344x |
| materialize | 0.982x | 0.976x | 1.160x |
| perft_bulk_d4 | 1.005x | 0.994x | 1.211x |
| perft_full_d3 | 0.971x | 0.947x | 0.966x |
| fen_parse | 1.000x | 0.985x | 1.238x |
| fen_write_native_ep | 1.008x | 1.000x | 1.895x |
| snapshot | 1.002x | 0.998x | 1.250x |
| san_parse_all | 0.998x | 0.979x | 2.194x |
| san_render_native | 0.966x | 0.936x | 2.017x |
| apply_children_full | 0.965x | 0.907x | 1.182x |
| make_unmake_full | 0.990x | 0.960x | 1.186x |
| startpos | 0.999x | 0.999x | 1.199x |
| chess960_materialize | 1.021x | 1.007x | 1.474x |
| chess960_perft_d3 | 0.962x | 0.932x | 1.427x |
| polyglot_key | 1.016x | 1.002x | 20.563x |
| polyglot_recompute | 1.004x | 0.998x | 0.989x |
| repetition_key | 0.998x | 0.998x | 15.656x |
| corpus_count | 0.990x | 0.986x | 1.189x |
| corpus_materialize | 1.001x | 0.996x | 1.169x |
| corpus_uci_sorted | 0.996x | 0.994x | 1.654x |
| replay_trusted_native | 1.019x | 1.007x | 1.152x |
| replay_checked_native | 0.995x | 0.994x | 1.005x |

Cold first legal move list: 0.006 ms -> 0.008 ms (0.81x).
