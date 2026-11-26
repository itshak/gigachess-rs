| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.015x | 0.915x | 1.308x |
| materialize | 1.015x | 0.986x | 1.189x |
| perft_bulk_d4 | 0.933x | 0.873x | 1.198x |
| perft_full_d3 | 0.991x | 0.949x | 1.256x |
| fen_parse | 1.041x | 0.991x | 1.297x |
| fen_write_native_ep | 1.004x | 0.975x | 1.612x |
| snapshot | 1.778x | 1.751x | 1.002x |
| san_parse_all | 0.990x | 0.948x | 2.033x |
| san_render_native | 0.985x | 0.920x | 3.464x |
| apply_children_full | 1.051x | 1.024x | 1.283x |
| make_unmake_full | 1.017x | 0.996x | 1.293x |
| startpos | 1.161x | 1.161x | 1.246x |
| chess960_materialize | 1.033x | 0.970x | 1.239x |
| chess960_perft_d3 | 1.028x | 0.987x | 1.392x |
| polyglot_key | 0.896x | 0.838x | 28.001x |
| polyglot_recompute | 0.940x | 0.870x | 0.922x |
| repetition_key | 1.072x | 0.937x | 21.163x |
| corpus_count | 0.953x | 0.876x | 1.250x |
| corpus_materialize | 0.927x | 0.899x | 1.106x |
| corpus_uci_sorted | 1.037x | 1.023x | 1.400x |
| replay_trusted_native | 1.048x | 1.010x | 0.719x |
| replay_checked_native | 1.066x | 1.015x | 0.977x |

Cold first legal move list: 0.034 ms -> 0.022 ms (1.55x).
