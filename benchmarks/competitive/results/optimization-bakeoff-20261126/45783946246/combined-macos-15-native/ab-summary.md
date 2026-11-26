| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.989x | 0.697x | 1.231x |
| materialize | 1.165x | 0.827x | 1.097x |
| perft_bulk_d4 | 1.049x | 0.883x | 1.238x |
| perft_full_d3 | 0.937x | 0.821x | 1.354x |
| fen_parse | 0.962x | 0.703x | 1.166x |
| fen_write_native_ep | 0.972x | 0.836x | 1.666x |
| snapshot | 0.859x | 0.555x | 0.762x |
| san_parse_all | 1.086x | 0.930x | 1.977x |
| san_render_native | 1.039x | 0.880x | 2.751x |
| apply_children_full | 1.018x | 0.777x | 0.904x |
| make_unmake_full | 0.991x | 0.821x | 1.231x |
| startpos | 1.118x | 1.118x | 1.197x |
| chess960_materialize | 0.847x | 0.808x | 1.312x |
| chess960_perft_d3 | 1.008x | 0.868x | 1.424x |
| polyglot_key | 1.019x | 0.831x | 34.388x |
| polyglot_recompute | 1.059x | 0.966x | 0.964x |
| repetition_key | 1.098x | 1.040x | 19.659x |
| corpus_count | 1.279x | 1.155x | 1.450x |
| corpus_materialize | 1.131x | 1.041x | 1.205x |
| corpus_uci_sorted | 0.991x | 0.979x | 1.520x |
| replay_trusted_native | 0.997x | 0.954x | 0.566x |
| replay_checked_native | 0.910x | 0.670x | 0.834x |

Cold first legal move list: 0.017 ms -> 0.009 ms (1.90x).
