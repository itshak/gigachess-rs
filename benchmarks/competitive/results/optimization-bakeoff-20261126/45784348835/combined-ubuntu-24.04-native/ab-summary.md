| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.983x | 1.399x |
| materialize | 0.998x | 0.995x | 1.489x |
| perft_bulk_d4 | 1.004x | 1.000x | 1.212x |
| perft_full_d3 | 1.005x | 0.999x | 1.688x |
| fen_parse | 1.146x | 1.112x | 1.106x |
| fen_write_native_ep | 0.995x | 0.962x | 2.111x |
| snapshot | 0.707x | 0.704x | 1.079x |
| san_parse_all | 0.997x | 0.985x | 1.859x |
| san_render_native | 1.008x | 1.002x | 2.174x |
| apply_children_full | 1.093x | 1.066x | 1.197x |
| make_unmake_full | 0.972x | 0.919x | 1.116x |
| startpos | 0.812x | 0.812x | 1.152x |
| chess960_materialize | 1.049x | 1.044x | 1.443x |
| chess960_perft_d3 | 0.996x | 0.994x | 1.385x |
| polyglot_key | 1.025x | 0.997x | 39.017x |
| polyglot_recompute | 1.048x | 1.035x | 1.024x |
| repetition_key | 1.222x | 1.099x | 22.510x |
| corpus_count | 0.999x | 0.995x | 1.343x |
| corpus_materialize | 1.007x | 1.006x | 1.072x |
| corpus_uci_sorted | 1.007x | 1.004x | 1.568x |
| replay_trusted_native | 1.145x | 1.009x | 0.820x |
| replay_checked_native | 1.012x | 1.005x | 1.123x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.92x).
