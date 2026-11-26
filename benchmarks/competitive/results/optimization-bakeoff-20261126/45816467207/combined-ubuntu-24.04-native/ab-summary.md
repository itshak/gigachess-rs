| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.991x | 1.370x |
| materialize | 1.007x | 0.993x | 1.518x |
| perft_bulk_d4 | 1.002x | 0.967x | 1.034x |
| perft_full_d3 | 1.014x | 0.973x | 1.446x |
| fen_parse | 0.926x | 0.829x | 1.103x |
| fen_write_native_ep | 1.043x | 1.019x | 2.027x |
| snapshot | 0.995x | 0.985x | 0.994x |
| san_parse_all | 1.134x | 1.114x | 1.894x |
| san_render_native | 1.029x | 1.021x | 2.158x |
| apply_children_full | 0.992x | 0.975x | 1.175x |
| make_unmake_full | 1.010x | 0.999x | 1.119x |
| startpos | 1.232x | 1.232x | 1.254x |
| chess960_materialize | 1.004x | 1.002x | 1.445x |
| chess960_perft_d3 | 0.991x | 0.990x | 1.416x |
| polyglot_key | 0.984x | 0.974x | 40.555x |
| polyglot_recompute | 0.972x | 0.962x | 1.022x |
| repetition_key | 0.980x | 0.970x | 19.521x |
| corpus_count | 0.999x | 0.993x | 0.853x |
| corpus_materialize | 1.003x | 1.002x | 1.026x |
| corpus_uci_sorted | 0.988x | 0.979x | 1.545x |
| replay_trusted_native | 0.993x | 0.987x | 0.812x |
| replay_checked_native | 1.000x | 0.996x | 1.153x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.96x).
