| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.910x | 1.409x |
| materialize | 1.049x | 0.919x | 1.573x |
| perft_bulk_d4 | 0.895x | 0.766x | 1.089x |
| perft_full_d3 | 1.027x | 0.979x | 1.461x |
| fen_parse | 0.914x | 0.864x | 1.041x |
| fen_write_native_ep | 1.013x | 0.985x | 1.910x |
| snapshot | 1.663x | 1.658x | 1.153x |
| san_parse_all | 0.984x | 0.974x | 1.897x |
| san_render_native | 1.023x | 1.002x | 2.127x |
| apply_children_full | 1.024x | 0.937x | 1.183x |
| make_unmake_full | 0.999x | 0.969x | 1.114x |
| startpos | 1.344x | 1.344x | 1.116x |
| chess960_materialize | 1.094x | 1.046x | 1.574x |
| chess960_perft_d3 | 1.054x | 0.986x | 1.451x |
| polyglot_key | 1.021x | 0.999x | 39.071x |
| polyglot_recompute | 1.039x | 1.029x | 1.017x |
| repetition_key | 0.947x | 0.734x | 17.998x |
| corpus_count | 0.644x | 0.539x | 0.861x |
| corpus_materialize | 0.992x | 0.985x | 1.080x |
| corpus_uci_sorted | 0.994x | 0.988x | 1.562x |
| replay_trusted_native | 0.990x | 0.959x | 0.792x |
| replay_checked_native | 1.011x | 0.996x | 1.095x |

Cold first legal move list: 0.009 ms -> 0.005 ms (1.78x).
