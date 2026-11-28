| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.120x | 1.029x | 1.557x |
| materialize | 1.084x | 0.738x | 1.019x |
| perft_bulk_d4 | 1.218x | 1.116x | 1.406x |
| perft_full_d3 | 1.309x | 1.072x | 1.487x |
| fen_parse | 1.016x | 0.795x | 1.189x |
| fen_write_native_ep | 1.166x | 0.893x | 1.700x |
| snapshot | 1.093x | 0.868x | 1.078x |
| san_parse_all | 1.058x | 0.983x | 1.707x |
| san_render_native | 1.094x | 0.887x | 2.754x |
| apply_children_full | 1.058x | 0.752x | 1.117x |
| make_unmake_full | 1.089x | 0.905x | 1.304x |
| startpos | 1.007x | 1.007x | 1.521x |
| chess960_materialize | 1.006x | 0.905x | 1.451x |
| chess960_perft_d3 | 1.067x | 0.963x | 1.505x |
| polyglot_key | 1.007x | 0.944x | 37.007x |
| polyglot_recompute | 1.415x | 1.259x | 1.463x |
| repetition_key | 1.055x | 0.915x | 19.590x |
| corpus_count | 1.310x | 1.023x | 1.345x |
| corpus_materialize | 1.007x | 0.997x | 1.079x |
| corpus_uci_sorted | 1.156x | 1.054x | 1.416x |
| replay_trusted_native | 1.125x | 1.071x | 0.689x |
| replay_checked_native | 0.998x | 0.933x | 1.296x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.80x).
