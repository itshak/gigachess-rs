| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.138x | 1.065x | 1.232x |
| materialize | 1.015x | 0.998x | 1.185x |
| perft_bulk_d4 | 1.303x | 1.266x | 1.145x |
| perft_full_d3 | 1.257x | 1.185x | 0.752x |
| fen_parse | 1.020x | 1.012x | 1.218x |
| fen_write_native_ep | 0.990x | 0.971x | 1.051x |
| snapshot | 1.235x | 1.224x | 0.944x |
| san_parse_all | 1.933x | 1.847x | 1.776x |
| san_render_native | 2.078x | 1.629x | 2.382x |
| apply_children_full | 1.272x | 1.229x | 0.824x |
| make_unmake_full | 1.244x | 1.223x | 1.125x |
| startpos | 1.493x | 1.493x | 1.146x |
| chess960_materialize | 1.084x | 1.001x | 1.196x |
| chess960_perft_d3 | 1.305x | 1.193x | 1.091x |
| polyglot_key | 1.123x | 1.021x | 25.808x |
| polyglot_recompute | 0.952x | 0.935x | 1.075x |
| repetition_key | 1.388x | 1.195x | 19.168x |
| corpus_count | 1.155x | 1.112x | 1.396x |
| corpus_materialize | 1.072x | 1.066x | 1.193x |
| corpus_uci_sorted | 1.334x | 1.334x | 0.772x |
| replay_trusted_native | 1.277x | 1.209x | 0.407x |
| replay_checked_native | 1.343x | 1.303x | 0.882x |

Cold first legal move list: 154.089 ms → 0.011 ms (14121.03x).
