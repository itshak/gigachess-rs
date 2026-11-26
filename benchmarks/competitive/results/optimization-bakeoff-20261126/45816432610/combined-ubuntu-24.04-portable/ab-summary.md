| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.025x | 0.937x | 1.233x |
| materialize | 0.851x | 0.699x | 1.070x |
| perft_bulk_d4 | 1.009x | 0.952x | 1.236x |
| perft_full_d3 | 0.976x | 0.918x | 1.303x |
| fen_parse | 0.976x | 0.949x | 1.211x |
| fen_write_native_ep | 1.008x | 0.994x | 1.707x |
| snapshot | 1.000x | 0.996x | 0.980x |
| san_parse_all | 1.026x | 1.001x | 2.003x |
| san_render_native | 0.999x | 0.976x | 2.339x |
| apply_children_full | 0.983x | 0.928x | 1.230x |
| make_unmake_full | 0.947x | 0.839x | 1.248x |
| startpos | 1.000x | 1.000x | 1.346x |
| chess960_materialize | 1.016x | 1.015x | 1.297x |
| chess960_perft_d3 | 1.092x | 0.991x | 1.522x |
| polyglot_key | 0.930x | 0.870x | 25.600x |
| polyglot_recompute | 0.984x | 0.981x | 1.019x |
| repetition_key | 0.887x | 0.840x | 18.106x |
| corpus_count | 0.936x | 0.865x | 1.229x |
| corpus_materialize | 0.886x | 0.844x | 1.072x |
| corpus_uci_sorted | 0.987x | 0.975x | 1.764x |
| replay_trusted_native | 1.041x | 1.027x | 0.794x |
| replay_checked_native | 1.025x | 1.022x | 1.086x |

Cold first legal move list: 0.014 ms -> 0.008 ms (1.81x).
