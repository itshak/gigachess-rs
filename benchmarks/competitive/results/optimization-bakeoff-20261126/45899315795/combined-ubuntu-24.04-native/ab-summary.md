| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.971x | 0.938x | 1.448x |
| materialize | 0.966x | 0.930x | 1.534x |
| perft_bulk_d4 | 0.975x | 0.954x | 1.137x |
| perft_full_d3 | 0.907x | 0.892x | 1.250x |
| fen_parse | 0.996x | 0.952x | 1.122x |
| fen_write_native_ep | 0.916x | 0.839x | 1.873x |
| snapshot | 0.672x | 0.670x | 0.867x |
| san_parse_all | 0.981x | 0.967x | 1.827x |
| san_render_native | 0.917x | 0.806x | 1.907x |
| apply_children_full | 0.961x | 0.919x | 1.346x |
| make_unmake_full | 0.953x | 0.910x | 1.137x |
| startpos | 1.008x | 1.008x | 1.187x |
| chess960_materialize | 0.938x | 0.936x | 1.539x |
| chess960_perft_d3 | 0.960x | 0.927x | 1.445x |
| polyglot_key | 1.046x | 1.003x | 40.167x |
| polyglot_recompute | 1.022x | 1.018x | 1.039x |
| repetition_key | 1.033x | 1.010x | 20.232x |
| corpus_count | 0.934x | 0.876x | 1.008x |
| corpus_materialize | 1.017x | 1.010x | 1.066x |
| corpus_uci_sorted | 1.010x | 1.001x | 1.551x |
| replay_trusted_native | 0.984x | 0.974x | 0.826x |
| replay_checked_native | 0.904x | 0.900x | 1.079x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.87x).
