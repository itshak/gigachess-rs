| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.996x | 1.276x |
| materialize | 1.087x | 1.036x | 1.710x |
| perft_bulk_d4 | 0.998x | 0.989x | 1.180x |
| perft_full_d3 | 0.987x | 0.977x | 1.243x |
| fen_parse | 0.894x | 0.790x | 1.031x |
| fen_write_native_ep | 0.988x | 0.967x | 2.009x |
| snapshot | 1.003x | 0.996x | 1.003x |
| san_parse_all | 1.090x | 1.068x | 2.065x |
| san_render_native | 0.979x | 0.971x | 2.108x |
| apply_children_full | 1.007x | 0.977x | 1.309x |
| make_unmake_full | 1.013x | 0.959x | 1.257x |
| startpos | 1.002x | 1.002x | 1.429x |
| chess960_materialize | 1.007x | 1.000x | 1.617x |
| chess960_perft_d3 | 0.996x | 0.991x | 1.502x |
| polyglot_key | 0.930x | 0.863x | 38.345x |
| polyglot_recompute | 0.976x | 0.942x | 1.010x |
| repetition_key | 1.061x | 0.855x | 22.008x |
| corpus_count | 1.036x | 1.003x | 1.173x |
| corpus_materialize | 1.009x | 1.008x | 1.147x |
| corpus_uci_sorted | 1.003x | 0.997x | 1.612x |
| replay_trusted_native | 0.991x | 0.979x | 0.852x |
| replay_checked_native | 0.973x | 0.959x | 1.029x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.96x).
