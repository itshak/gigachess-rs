| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.984x | 0.917x | 1.489x |
| materialize | 0.965x | 0.900x | 1.491x |
| perft_bulk_d4 | 0.954x | 0.943x | 1.131x |
| perft_full_d3 | 0.881x | 0.834x | 1.140x |
| fen_parse | 0.998x | 0.975x | 0.968x |
| fen_write_native_ep | 0.993x | 0.962x | 1.954x |
| snapshot | 0.675x | 0.670x | 0.941x |
| san_parse_all | 0.960x | 0.907x | 1.702x |
| san_render_native | 0.937x | 0.883x | 1.963x |
| apply_children_full | 0.965x | 0.937x | 1.316x |
| make_unmake_full | 0.949x | 0.937x | 1.105x |
| startpos | 0.772x | 0.772x | 0.914x |
| chess960_materialize | 0.866x | 0.846x | 1.279x |
| chess960_perft_d3 | 0.924x | 0.874x | 1.354x |
| polyglot_key | 0.973x | 0.949x | 40.136x |
| polyglot_recompute | 1.014x | 1.008x | 1.003x |
| repetition_key | 0.844x | 0.728x | 22.695x |
| corpus_count | 0.999x | 0.993x | 0.942x |
| corpus_materialize | 1.001x | 0.999x | 1.087x |
| corpus_uci_sorted | 0.992x | 0.989x | 1.540x |
| replay_trusted_native | 0.941x | 0.935x | 0.831x |
| replay_checked_native | 0.935x | 0.897x | 1.020x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.96x).
