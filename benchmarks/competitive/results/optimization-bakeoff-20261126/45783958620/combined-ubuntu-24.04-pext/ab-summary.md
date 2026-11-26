| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 1.001x | 1.489x |
| materialize | 0.998x | 0.992x | 1.588x |
| perft_bulk_d4 | 1.007x | 0.993x | 1.268x |
| perft_full_d3 | 0.976x | 0.855x | 1.605x |
| fen_parse | 0.992x | 0.977x | 1.019x |
| fen_write_native_ep | 1.005x | 0.952x | 1.939x |
| snapshot | 1.001x | 0.999x | 1.154x |
| san_parse_all | 1.011x | 0.975x | 1.917x |
| san_render_native | 1.004x | 0.972x | 2.101x |
| apply_children_full | 0.970x | 0.927x | 1.038x |
| make_unmake_full | 0.928x | 0.914x | 1.061x |
| startpos | 1.121x | 1.121x | 1.210x |
| chess960_materialize | 0.970x | 0.941x | 1.425x |
| chess960_perft_d3 | 0.982x | 0.966x | 1.356x |
| polyglot_key | 0.965x | 0.931x | 38.626x |
| polyglot_recompute | 0.996x | 0.995x | 1.015x |
| repetition_key | 1.010x | 0.984x | 19.265x |
| corpus_count | 1.002x | 1.001x | 1.370x |
| corpus_materialize | 1.004x | 0.999x | 1.157x |
| corpus_uci_sorted | 0.993x | 0.989x | 1.582x |
| replay_trusted_native | 0.932x | 0.915x | 0.651x |
| replay_checked_native | 0.971x | 0.934x | 1.031x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.98x).
