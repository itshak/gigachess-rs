| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.992x | 0.955x | 1.309x |
| materialize | 0.981x | 0.808x | 1.091x |
| perft_bulk_d4 | 0.980x | 0.963x | 1.116x |
| perft_full_d3 | 0.980x | 0.964x | 1.193x |
| fen_parse | 1.003x | 0.986x | 1.315x |
| fen_write_native_ep | 1.076x | 0.961x | 1.553x |
| snapshot | 0.981x | 0.951x | 1.489x |
| san_parse_all | 0.991x | 0.962x | 1.773x |
| san_render_native | 0.918x | 0.847x | 2.132x |
| apply_children_full | 1.007x | 0.969x | 1.325x |
| make_unmake_full | 0.994x | 0.961x | 1.230x |
| startpos | 0.985x | 0.985x | 1.252x |
| chess960_materialize | 0.944x | 0.938x | 1.361x |
| chess960_perft_d3 | 0.991x | 0.990x | 1.395x |
| polyglot_key | 0.981x | 0.980x | 34.770x |
| polyglot_recompute | 0.986x | 0.979x | 0.980x |
| repetition_key | 0.816x | 0.790x | 22.735x |
| corpus_count | 0.978x | 0.975x | 1.154x |
| corpus_materialize | 0.982x | 0.969x | 1.060x |
| corpus_uci_sorted | 0.995x | 0.988x | 1.649x |
| replay_trusted_native | 0.976x | 0.968x | 0.713x |
| replay_checked_native | 1.037x | 1.031x | 1.148x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.96x).
