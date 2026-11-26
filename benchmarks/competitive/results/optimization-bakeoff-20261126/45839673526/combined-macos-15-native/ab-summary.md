| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.826x | 0.620x | 1.427x |
| materialize | 0.808x | 0.739x | 0.964x |
| perft_bulk_d4 | 0.904x | 0.704x | 1.220x |
| perft_full_d3 | 0.801x | 0.696x | 0.962x |
| fen_parse | 0.861x | 0.816x | 1.142x |
| fen_write_native_ep | 1.007x | 0.839x | 1.842x |
| snapshot | 0.958x | 0.693x | 0.827x |
| san_parse_all | 0.930x | 0.802x | 1.770x |
| san_render_native | 0.903x | 0.742x | 2.651x |
| apply_children_full | 0.941x | 0.824x | 1.122x |
| make_unmake_full | 0.851x | 0.732x | 1.185x |
| startpos | 0.884x | 0.884x | 1.136x |
| chess960_materialize | 0.949x | 0.908x | 1.573x |
| chess960_perft_d3 | 0.873x | 0.745x | 1.359x |
| polyglot_key | 0.796x | 0.756x | 34.132x |
| polyglot_recompute | 0.837x | 0.824x | 0.926x |
| repetition_key | 0.874x | 0.824x | 18.701x |
| corpus_count | 0.792x | 0.671x | 1.154x |
| corpus_materialize | 0.819x | 0.775x | 1.089x |
| corpus_uci_sorted | 0.854x | 0.836x | 1.258x |
| replay_trusted_native | 0.855x | 0.833x | 0.625x |
| replay_checked_native | 0.887x | 0.885x | 1.012x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.14x).
