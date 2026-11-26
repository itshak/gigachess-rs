| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.983x | 0.948x | 1.470x |
| materialize | 1.053x | 1.012x | 1.515x |
| perft_bulk_d4 | 0.976x | 0.963x | 1.234x |
| perft_full_d3 | 0.958x | 0.911x | 1.484x |
| fen_parse | 0.916x | 0.883x | 0.941x |
| fen_write_native_ep | 1.003x | 0.964x | 2.129x |
| snapshot | 0.672x | 0.668x | 1.225x |
| san_parse_all | 1.080x | 1.004x | 1.793x |
| san_render_native | 0.930x | 0.893x | 1.986x |
| apply_children_full | 0.917x | 0.886x | 0.971x |
| make_unmake_full | 0.900x | 0.875x | 1.020x |
| startpos | 0.892x | 0.892x | 1.496x |
| chess960_materialize | 0.923x | 0.871x | 1.345x |
| chess960_perft_d3 | 0.932x | 0.904x | 1.289x |
| polyglot_key | 1.002x | 0.996x | 39.376x |
| polyglot_recompute | 0.986x | 0.980x | 0.994x |
| repetition_key | 1.041x | 0.989x | 20.077x |
| corpus_count | 1.015x | 1.004x | 1.431x |
| corpus_materialize | 1.015x | 1.012x | 1.142x |
| corpus_uci_sorted | 0.977x | 0.974x | 1.571x |
| replay_trusted_native | 0.773x | 0.693x | 0.559x |
| replay_checked_native | 0.730x | 0.619x | 0.794x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.88x).
