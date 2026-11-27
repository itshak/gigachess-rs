| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.050x | 1.009x | 1.343x |
| materialize | 0.998x | 0.987x | 1.730x |
| perft_bulk_d4 | 1.063x | 1.044x | 1.261x |
| perft_full_d3 | 0.996x | 0.991x | 1.325x |
| fen_parse | 1.003x | 0.953x | 1.169x |
| fen_write_native_ep | 1.003x | 0.996x | 1.948x |
| snapshot | 1.002x | 1.000x | 1.004x |
| san_parse_all | 0.926x | 0.890x | 1.778x |
| san_render_native | 1.015x | 0.995x | 2.284x |
| apply_children_full | 1.002x | 0.998x | 1.384x |
| make_unmake_full | 1.060x | 1.003x | 1.250x |
| startpos | 1.001x | 1.001x | 1.285x |
| chess960_materialize | 1.004x | 0.999x | 1.555x |
| chess960_perft_d3 | 1.010x | 0.985x | 1.521x |
| polyglot_key | 0.994x | 0.989x | 41.336x |
| polyglot_recompute | 1.056x | 1.045x | 1.058x |
| repetition_key | 1.009x | 0.995x | 21.711x |
| corpus_count | 1.161x | 1.103x | 1.307x |
| corpus_materialize | 0.996x | 0.996x | 1.136x |
| corpus_uci_sorted | 0.994x | 0.988x | 1.577x |
| replay_trusted_native | 1.001x | 0.996x | 0.885x |
| replay_checked_native | 1.012x | 0.999x | 1.168x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.99x).
