| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.180x | 0.969x | 1.619x |
| materialize | 0.985x | 0.870x | 1.032x |
| perft_bulk_d4 | 0.983x | 0.872x | 1.222x |
| perft_full_d3 | 0.918x | 0.602x | 1.235x |
| fen_parse | 1.068x | 0.824x | 1.198x |
| fen_write_native_ep | 1.073x | 0.941x | 1.683x |
| snapshot | 0.908x | 0.774x | 0.949x |
| san_parse_all | 1.014x | 0.811x | 1.953x |
| san_render_native | 0.926x | 0.732x | 2.610x |
| apply_children_full | 1.053x | 0.885x | 1.146x |
| make_unmake_full | 1.073x | 0.869x | 1.221x |
| startpos | 1.253x | 1.253x | 1.686x |
| chess960_materialize | 0.928x | 0.902x | 1.585x |
| chess960_perft_d3 | 0.995x | 0.976x | 1.499x |
| polyglot_key | 1.126x | 1.059x | 40.276x |
| polyglot_recompute | 0.819x | 0.721x | 0.939x |
| repetition_key | 0.844x | 0.811x | 18.306x |
| corpus_count | 1.073x | 1.057x | 1.322x |
| corpus_materialize | 1.112x | 1.093x | 1.214x |
| corpus_uci_sorted | 1.101x | 0.998x | 1.460x |
| replay_trusted_native | 0.897x | 0.729x | 0.596x |
| replay_checked_native | 0.950x | 0.939x | 1.257x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.89x).
