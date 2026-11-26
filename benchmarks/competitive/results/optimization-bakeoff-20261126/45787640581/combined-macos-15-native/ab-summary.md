| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.099x | 0.851x | 1.334x |
| materialize | 1.009x | 0.793x | 0.984x |
| perft_bulk_d4 | 0.833x | 0.659x | 1.019x |
| perft_full_d3 | 0.994x | 0.845x | 1.405x |
| fen_parse | 0.923x | 0.794x | 1.187x |
| fen_write_native_ep | 0.985x | 0.870x | 1.742x |
| snapshot | 1.015x | 0.799x | 0.897x |
| san_parse_all | 0.987x | 0.795x | 1.951x |
| san_render_native | 1.017x | 0.748x | 2.646x |
| apply_children_full | 0.969x | 0.841x | 0.993x |
| make_unmake_full | 0.979x | 0.813x | 1.229x |
| startpos | 1.050x | 1.050x | 1.696x |
| chess960_materialize | 0.847x | 0.798x | 1.216x |
| chess960_perft_d3 | 0.786x | 0.561x | 1.214x |
| polyglot_key | 0.937x | 0.917x | 42.610x |
| polyglot_recompute | 0.961x | 0.843x | 1.121x |
| repetition_key | 0.994x | 0.848x | 20.683x |
| corpus_count | 0.619x | 0.578x | 1.032x |
| corpus_materialize | 0.962x | 0.881x | 0.991x |
| corpus_uci_sorted | 0.936x | 0.830x | 1.361x |
| replay_trusted_native | 1.098x | 0.929x | 0.662x |
| replay_checked_native | 0.930x | 0.891x | 1.032x |

Cold first legal move list: 0.008 ms -> 0.004 ms (1.91x).
