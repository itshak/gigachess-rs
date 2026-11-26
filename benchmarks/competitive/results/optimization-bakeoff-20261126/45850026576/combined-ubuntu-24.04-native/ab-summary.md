| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.958x | 1.448x |
| materialize | 0.999x | 0.926x | 1.208x |
| perft_bulk_d4 | 1.012x | 0.951x | 1.256x |
| perft_full_d3 | 1.007x | 0.948x | 1.256x |
| fen_parse | 0.960x | 0.898x | 1.089x |
| fen_write_native_ep | 1.016x | 0.973x | 1.573x |
| snapshot | 0.998x | 0.969x | 1.479x |
| san_parse_all | 1.012x | 0.985x | 1.736x |
| san_render_native | 1.064x | 1.011x | 2.297x |
| apply_children_full | 1.032x | 0.961x | 1.336x |
| make_unmake_full | 0.970x | 0.895x | 1.226x |
| startpos | 1.000x | 1.000x | 1.249x |
| chess960_materialize | 0.979x | 0.961x | 1.555x |
| chess960_perft_d3 | 0.982x | 0.967x | 1.495x |
| polyglot_key | 1.014x | 0.989x | 36.505x |
| polyglot_recompute | 1.048x | 1.001x | 1.071x |
| repetition_key | 0.990x | 0.981x | 27.607x |
| corpus_count | 1.001x | 0.992x | 1.149x |
| corpus_materialize | 0.988x | 0.983x | 1.145x |
| corpus_uci_sorted | 0.990x | 0.983x | 1.704x |
| replay_trusted_native | 0.969x | 0.965x | 0.701x |
| replay_checked_native | 1.010x | 0.975x | 1.131x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.85x).
