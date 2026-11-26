| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.867x | 0.623x | 1.146x |
| materialize | 0.975x | 0.808x | 1.082x |
| perft_bulk_d4 | 0.973x | 0.778x | 1.279x |
| perft_full_d3 | 1.018x | 0.818x | 1.332x |
| fen_parse | 1.022x | 0.913x | 1.244x |
| fen_write_native_ep | 0.906x | 0.699x | 1.240x |
| snapshot | 0.866x | 0.684x | 0.742x |
| san_parse_all | 0.947x | 0.694x | 1.814x |
| san_render_native | 0.897x | 0.757x | 2.709x |
| apply_children_full | 0.793x | 0.599x | 0.883x |
| make_unmake_full | 0.908x | 0.734x | 1.268x |
| startpos | 2.647x | 2.647x | 1.211x |
| chess960_materialize | 0.978x | 0.943x | 1.538x |
| chess960_perft_d3 | 1.087x | 1.007x | 1.271x |
| polyglot_key | 1.177x | 1.125x | 39.505x |
| polyglot_recompute | 0.854x | 0.814x | 0.945x |
| repetition_key | 0.855x | 0.692x | 17.707x |
| corpus_count | 0.926x | 0.880x | 1.266x |
| corpus_materialize | 0.887x | 0.787x | 1.001x |
| corpus_uci_sorted | 0.959x | 0.938x | 1.206x |
| replay_trusted_native | 0.872x | 0.747x | 0.506x |
| replay_checked_native | 1.067x | 1.048x | 1.033x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.05x).
