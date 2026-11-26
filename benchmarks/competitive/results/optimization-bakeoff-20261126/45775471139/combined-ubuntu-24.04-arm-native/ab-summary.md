| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.991x | 1.297x |
| materialize | 1.001x | 0.996x | 1.205x |
| perft_bulk_d4 | 1.006x | 0.996x | 1.297x |
| perft_full_d3 | 0.972x | 0.951x | 0.732x |
| fen_parse | 1.036x | 1.008x | 1.273x |
| fen_write_native_ep | 0.997x | 0.987x | 1.036x |
| snapshot | 1.006x | 1.001x | 0.989x |
| san_parse_all | 0.977x | 0.968x | 1.750x |
| san_render_native | 0.999x | 0.985x | 2.391x |
| apply_children_full | 1.007x | 0.986x | 0.820x |
| make_unmake_full | 0.968x | 0.960x | 1.101x |
| startpos | 0.896x | 0.896x | 1.043x |
| chess960_materialize | 0.998x | 0.993x | 1.227x |
| chess960_perft_d3 | 0.981x | 0.978x | 1.209x |
| polyglot_key | 1.035x | 1.000x | 26.178x |
| polyglot_recompute | 1.014x | 1.012x | 1.084x |
| repetition_key | 1.090x | 1.052x | 19.996x |
| corpus_count | 0.991x | 0.982x | 1.428x |
| corpus_materialize | 0.983x | 0.977x | 1.182x |
| corpus_uci_sorted | 0.995x | 0.994x | 0.969x |
| replay_trusted_native | 0.981x | 0.976x | 0.414x |
| replay_checked_native | 0.983x | 0.978x | 0.870x |

Cold first legal move list: 0.014 ms -> 0.012 ms (1.17x).
