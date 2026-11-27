| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.902x | 0.806x | 1.500x |
| materialize | 1.018x | 0.870x | 1.144x |
| perft_bulk_d4 | 1.017x | 0.837x | 1.241x |
| perft_full_d3 | 0.904x | 0.661x | 1.119x |
| fen_parse | 1.106x | 1.031x | 1.227x |
| fen_write_native_ep | 1.083x | 0.861x | 1.753x |
| snapshot | 1.076x | 0.934x | 1.012x |
| san_parse_all | 0.978x | 0.921x | 1.687x |
| san_render_native | 0.997x | 0.803x | 2.742x |
| apply_children_full | 1.063x | 0.944x | 1.179x |
| make_unmake_full | 1.001x | 0.735x | 1.376x |
| startpos | 1.009x | 1.009x | 1.222x |
| chess960_materialize | 0.973x | 0.926x | 1.559x |
| chess960_perft_d3 | 0.995x | 0.985x | 1.553x |
| polyglot_key | 1.062x | 0.921x | 34.339x |
| polyglot_recompute | 1.166x | 1.046x | 1.001x |
| repetition_key | 1.050x | 0.997x | 20.311x |
| corpus_count | 1.083x | 1.025x | 1.167x |
| corpus_materialize | 1.136x | 1.055x | 1.221x |
| corpus_uci_sorted | 1.212x | 1.044x | 1.649x |
| replay_trusted_native | 1.173x | 1.018x | 0.619x |
| replay_checked_native | 1.129x | 0.987x | 1.221x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.25x).
