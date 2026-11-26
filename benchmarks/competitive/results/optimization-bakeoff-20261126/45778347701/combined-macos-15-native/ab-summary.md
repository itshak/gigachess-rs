| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.935x | 0.818x | 1.348x |
| materialize | 0.889x | 0.649x | 0.958x |
| perft_bulk_d4 | 0.953x | 0.836x | 1.163x |
| perft_full_d3 | 0.993x | 0.840x | 1.275x |
| fen_parse | 0.961x | 0.678x | 1.036x |
| fen_write_native_ep | 1.051x | 0.742x | 1.180x |
| snapshot | 1.164x | 0.830x | 1.108x |
| san_parse_all | 0.970x | 0.778x | 1.809x |
| san_render_native | 1.040x | 0.948x | 2.567x |
| apply_children_full | 1.155x | 0.935x | 0.980x |
| make_unmake_full | 0.991x | 0.879x | 1.055x |
| startpos | 0.543x | 0.543x | 0.639x |
| chess960_materialize | 1.038x | 0.959x | 1.461x |
| chess960_perft_d3 | 0.919x | 0.881x | 1.312x |
| polyglot_key | 1.183x | 0.917x | 42.458x |
| polyglot_recompute | 1.182x | 0.965x | 1.003x |
| repetition_key | 1.007x | 0.883x | 20.897x |
| corpus_count | 0.925x | 0.890x | 1.173x |
| corpus_materialize | 1.165x | 1.084x | 1.174x |
| corpus_uci_sorted | 1.183x | 1.180x | 1.346x |
| replay_trusted_native | 1.005x | 0.942x | 0.542x |
| replay_checked_native | 1.206x | 1.043x | 1.040x |

Cold first legal move list: 0.010 ms -> 0.015 ms (0.69x).
