| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.998x | 1.292x |
| materialize | 0.996x | 0.994x | 1.203x |
| perft_bulk_d4 | 0.998x | 0.995x | 1.274x |
| perft_full_d3 | 0.993x | 0.985x | 0.745x |
| fen_parse | 0.989x | 0.983x | 1.218x |
| fen_write_native_ep | 0.991x | 0.980x | 1.035x |
| snapshot | 0.955x | 0.950x | 0.941x |
| san_parse_all | 0.996x | 0.990x | 1.747x |
| san_render_native | 1.000x | 0.990x | 2.389x |
| apply_children_full | 0.994x | 0.979x | 0.824x |
| make_unmake_full | 0.995x | 0.980x | 1.155x |
| startpos | 0.968x | 0.968x | 1.148x |
| chess960_materialize | 0.998x | 0.993x | 1.233x |
| chess960_perft_d3 | 1.001x | 1.000x | 1.221x |
| polyglot_key | 1.010x | 0.995x | 25.837x |
| polyglot_recompute | 1.004x | 0.998x | 1.078x |
| repetition_key | 1.009x | 1.000x | 19.043x |
| corpus_count | 1.004x | 1.002x | 1.427x |
| corpus_materialize | 1.000x | 0.999x | 1.189x |
| corpus_uci_sorted | 1.497x | 1.487x | 1.456x |
| replay_trusted_native | 0.994x | 0.993x | 0.413x |
| replay_checked_native | 0.992x | 0.986x | 0.904x |

Cold first legal move list: 0.013 ms -> 0.012 ms (1.04x).
