| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.044x | 1.013x | 1.622x |
| materialize | 1.012x | 0.989x | 1.596x |
| perft_bulk_d4 | 1.053x | 1.024x | 1.253x |
| perft_full_d3 | 1.220x | 1.093x | 1.649x |
| fen_parse | 0.997x | 0.992x | 1.128x |
| fen_write_native_ep | 0.999x | 0.990x | 2.050x |
| snapshot | 0.999x | 0.994x | 1.000x |
| san_parse_all | 0.927x | 0.899x | 1.711x |
| san_render_native | 0.986x | 0.977x | 2.103x |
| apply_children_full | 0.975x | 0.934x | 1.321x |
| make_unmake_full | 1.002x | 0.971x | 1.200x |
| startpos | 0.890x | 0.890x | 1.000x |
| chess960_materialize | 1.113x | 1.102x | 1.490x |
| chess960_perft_d3 | 1.006x | 1.000x | 1.486x |
| polyglot_key | 0.973x | 0.945x | 38.701x |
| polyglot_recompute | 1.019x | 1.017x | 1.015x |
| repetition_key | 0.767x | 0.692x | 18.719x |
| corpus_count | 1.170x | 1.121x | 1.296x |
| corpus_materialize | 1.022x | 1.014x | 1.091x |
| corpus_uci_sorted | 1.023x | 1.022x | 1.597x |
| replay_trusted_native | 0.979x | 0.936x | 0.869x |
| replay_checked_native | 0.964x | 0.957x | 1.164x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.98x).
