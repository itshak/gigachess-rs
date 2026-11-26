| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.994x | 1.562x |
| materialize | 1.000x | 0.994x | 1.595x |
| perft_bulk_d4 | 1.000x | 0.996x | 1.185x |
| perft_full_d3 | 0.995x | 0.990x | 1.339x |
| fen_parse | 0.982x | 0.972x | 1.025x |
| fen_write_native_ep | 1.007x | 0.989x | 2.146x |
| snapshot | 1.000x | 0.992x | 0.998x |
| san_parse_all | 1.001x | 0.991x | 1.842x |
| san_render_native | 0.983x | 0.977x | 2.113x |
| apply_children_full | 1.001x | 0.998x | 1.355x |
| make_unmake_full | 0.982x | 0.938x | 1.173x |
| startpos | 0.900x | 0.900x | 1.003x |
| chess960_materialize | 1.012x | 1.007x | 1.568x |
| chess960_perft_d3 | 1.000x | 0.999x | 1.479x |
| polyglot_key | 1.074x | 1.003x | 38.437x |
| polyglot_recompute | 1.073x | 0.870x | 1.062x |
| repetition_key | 1.004x | 1.000x | 22.646x |
| corpus_count | 1.006x | 0.998x | 1.128x |
| corpus_materialize | 1.002x | 0.999x | 1.079x |
| corpus_uci_sorted | 1.028x | 1.023x | 1.607x |
| replay_trusted_native | 0.999x | 0.997x | 0.905x |
| replay_checked_native | 1.003x | 1.000x | 1.179x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.87x).
