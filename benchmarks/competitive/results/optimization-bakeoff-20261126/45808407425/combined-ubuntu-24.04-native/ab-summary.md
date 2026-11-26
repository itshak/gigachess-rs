| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.951x | 1.404x |
| materialize | 0.917x | 0.848x | 1.459x |
| perft_bulk_d4 | 0.960x | 0.938x | 1.168x |
| perft_full_d3 | 0.915x | 0.878x | 1.390x |
| fen_parse | 0.910x | 0.866x | 1.079x |
| fen_write_native_ep | 1.012x | 0.971x | 1.952x |
| snapshot | 0.672x | 0.670x | 1.225x |
| san_parse_all | 0.926x | 0.907x | 1.765x |
| san_render_native | 0.949x | 0.910x | 2.028x |
| apply_children_full | 0.875x | 0.859x | 1.110x |
| make_unmake_full | 0.920x | 0.906x | 1.022x |
| startpos | 1.292x | 1.292x | 2.166x |
| chess960_materialize | 0.966x | 0.950x | 1.399x |
| chess960_perft_d3 | 0.956x | 0.951x | 1.313x |
| polyglot_key | 1.016x | 0.998x | 41.399x |
| polyglot_recompute | 1.012x | 1.011x | 1.029x |
| repetition_key | 1.096x | 0.935x | 22.121x |
| corpus_count | 1.017x | 0.999x | 1.349x |
| corpus_materialize | 1.017x | 1.014x | 1.120x |
| corpus_uci_sorted | 1.012x | 1.008x | 1.586x |
| replay_trusted_native | 0.903x | 0.895x | 0.714x |
| replay_checked_native | 0.907x | 0.896x | 0.968x |

Cold first legal move list: 0.009 ms -> 0.012 ms (0.74x).
