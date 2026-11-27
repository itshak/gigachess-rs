| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.983x | 1.486x |
| materialize | 0.957x | 0.924x | 1.529x |
| perft_bulk_d4 | 0.986x | 0.978x | 1.155x |
| perft_full_d3 | 0.996x | 0.988x | 1.334x |
| fen_parse | 0.926x | 0.839x | 1.042x |
| fen_write_native_ep | 1.001x | 0.985x | 1.916x |
| snapshot | 0.996x | 0.987x | 1.328x |
| san_parse_all | 1.007x | 0.988x | 1.898x |
| san_render_native | 1.014x | 1.006x | 2.149x |
| apply_children_full | 1.006x | 0.996x | 1.268x |
| make_unmake_full | 1.001x | 0.985x | 1.205x |
| startpos | 1.108x | 1.108x | 1.239x |
| chess960_materialize | 0.989x | 0.974x | 1.542x |
| chess960_perft_d3 | 0.991x | 0.978x | 1.508x |
| polyglot_key | 1.002x | 0.990x | 40.530x |
| polyglot_recompute | 1.008x | 1.007x | 1.029x |
| repetition_key | 1.225x | 1.094x | 24.707x |
| corpus_count | 0.994x | 0.993x | 1.086x |
| corpus_materialize | 1.015x | 1.011x | 1.073x |
| corpus_uci_sorted | 0.999x | 0.997x | 1.562x |
| replay_trusted_native | 1.023x | 1.012x | 0.904x |
| replay_checked_native | 0.987x | 0.984x | 1.136x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.82x).
