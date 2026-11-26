| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.976x | 0.961x | 1.161x |
| materialize | 0.996x | 0.975x | 1.202x |
| perft_bulk_d4 | 1.111x | 1.084x | 1.102x |
| perft_full_d3 | 1.140x | 1.109x | 0.689x |
| fen_parse | 0.902x | 0.858x | 1.062x |
| fen_write_native_ep | 1.009x | 1.002x | 0.966x |
| snapshot | 1.000x | 1.000x | 0.846x |
| san_parse_all | 1.441x | 1.382x | 1.416x |
| san_render_native | 1.576x | 1.482x | 1.927x |
| apply_children_full | 1.041x | 1.019x | 0.783x |
| make_unmake_full | 1.059x | 1.037x | 0.914x |
| startpos | 1.005x | 1.005x | 0.726x |
| chess960_materialize | 1.029x | 0.926x | 1.195x |
| chess960_perft_d3 | 1.136x | 1.061x | 1.093x |
| polyglot_key | 1.010x | 0.999x | 30.280x |
| polyglot_recompute | 1.000x | 0.994x | 1.011x |
| repetition_key | 1.089x | 1.061x | 17.234x |
| corpus_count | 1.023x | 1.016x | 1.205x |
| corpus_materialize | 1.003x | 1.003x | 1.195x |
| corpus_uci_sorted | 1.225x | 1.215x | 0.801x |
| replay_trusted_native | 1.082x | 1.036x | 0.527x |
| replay_checked_native | 1.194x | 1.115x | 0.870x |

Cold first legal move list: 278.626 ms → 0.013 ms (21932.17x).
