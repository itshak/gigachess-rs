| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.974x | 0.915x | 1.370x |
| materialize | 0.986x | 0.951x | 1.506x |
| perft_bulk_d4 | 0.854x | 0.771x | 1.038x |
| perft_full_d3 | 1.012x | 0.905x | 1.517x |
| fen_parse | 1.001x | 0.985x | 1.120x |
| fen_write_native_ep | 0.961x | 0.906x | 1.957x |
| snapshot | 0.917x | 0.907x | 1.079x |
| san_parse_all | 0.989x | 0.980x | 1.877x |
| san_render_native | 1.031x | 1.019x | 2.148x |
| apply_children_full | 1.022x | 1.014x | 1.198x |
| make_unmake_full | 0.983x | 0.974x | 1.105x |
| startpos | 0.889x | 0.889x | 1.152x |
| chess960_materialize | 0.996x | 0.976x | 1.442x |
| chess960_perft_d3 | 1.027x | 0.978x | 1.414x |
| polyglot_key | 1.006x | 1.002x | 39.504x |
| polyglot_recompute | 1.052x | 1.030x | 1.012x |
| repetition_key | 1.086x | 0.920x | 21.290x |
| corpus_count | 0.664x | 0.563x | 0.892x |
| corpus_materialize | 0.950x | 0.932x | 1.041x |
| corpus_uci_sorted | 1.004x | 0.998x | 1.551x |
| replay_trusted_native | 1.003x | 1.000x | 0.796x |
| replay_checked_native | 0.999x | 0.992x | 1.111x |

Cold first legal move list: 0.009 ms -> 0.006 ms (1.53x).
