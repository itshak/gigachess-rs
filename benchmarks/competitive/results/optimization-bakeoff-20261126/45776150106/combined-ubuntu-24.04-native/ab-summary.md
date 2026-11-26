| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.998x | 1.373x |
| materialize | 0.982x | 0.916x | 1.448x |
| perft_bulk_d4 | 0.896x | 0.863x | 1.050x |
| perft_full_d3 | 0.824x | 0.774x | 0.724x |
| fen_parse | 0.855x | 0.832x | 1.041x |
| fen_write_native_ep | 1.038x | 1.015x | 1.296x |
| snapshot | 0.654x | 0.649x | 0.845x |
| san_parse_all | 0.976x | 0.938x | 1.758x |
| san_render_native | 0.944x | 0.921x | 1.941x |
| apply_children_full | 0.896x | 0.818x | 0.880x |
| make_unmake_full | 0.783x | 0.715x | 0.734x |
| startpos | 0.757x | 0.757x | 0.751x |
| chess960_materialize | 1.004x | 1.001x | 1.395x |
| chess960_perft_d3 | 0.905x | 0.895x | 1.196x |
| polyglot_key | 0.373x | 0.208x | 14.632x |
| polyglot_recompute | 1.039x | 1.017x | 1.008x |
| repetition_key | 0.557x | 0.424x | 11.980x |
| corpus_count | 0.991x | 0.985x | 1.290x |
| corpus_materialize | 0.993x | 0.983x | 1.099x |
| corpus_uci_sorted | 0.987x | 0.987x | 0.973x |
| replay_trusted_native | 1.012x | 0.884x | 0.609x |
| replay_checked_native | 0.814x | 0.717x | 0.819x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.89x).
