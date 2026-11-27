| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.986x | 1.278x |
| materialize | 0.926x | 0.890x | 1.632x |
| perft_bulk_d4 | 0.997x | 0.994x | 1.183x |
| perft_full_d3 | 0.999x | 0.987x | 1.320x |
| fen_parse | 1.025x | 1.003x | 1.161x |
| fen_write_native_ep | 1.012x | 0.991x | 2.022x |
| snapshot | 1.001x | 0.993x | 1.000x |
| san_parse_all | 1.001x | 0.990x | 1.924x |
| san_render_native | 1.008x | 1.004x | 2.152x |
| apply_children_full | 0.990x | 0.984x | 1.348x |
| make_unmake_full | 1.063x | 1.000x | 1.317x |
| startpos | 0.999x | 0.999x | 1.363x |
| chess960_materialize | 1.005x | 0.995x | 1.594x |
| chess960_perft_d3 | 1.002x | 1.002x | 1.504x |
| polyglot_key | 0.990x | 0.984x | 39.923x |
| polyglot_recompute | 1.015x | 1.000x | 0.974x |
| repetition_key | 1.030x | 0.997x | 20.365x |
| corpus_count | 0.992x | 0.985x | 1.127x |
| corpus_materialize | 0.991x | 0.990x | 1.129x |
| corpus_uci_sorted | 0.994x | 0.993x | 1.539x |
| replay_trusted_native | 0.977x | 0.947x | 0.852x |
| replay_checked_native | 1.007x | 0.982x | 1.155x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.87x).
