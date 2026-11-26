| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.984x | 0.881x | 1.141x |
| materialize | 1.011x | 0.994x | 1.261x |
| perft_bulk_d4 | 1.008x | 0.985x | 1.176x |
| perft_full_d3 | 1.007x | 0.976x | 1.362x |
| fen_parse | 1.013x | 0.970x | 1.278x |
| fen_write_native_ep | 1.027x | 1.005x | 1.460x |
| snapshot | 1.015x | 1.006x | 1.018x |
| san_parse_all | 0.986x | 0.944x | 1.930x |
| san_render_native | 0.977x | 0.922x | 2.279x |
| apply_children_full | 0.985x | 0.943x | 0.999x |
| make_unmake_full | 1.022x | 0.987x | 1.270x |
| startpos | 1.004x | 1.004x | 1.306x |
| chess960_materialize | 1.040x | 1.026x | 1.383x |
| chess960_perft_d3 | 1.010x | 1.008x | 1.327x |
| polyglot_key | 0.989x | 0.988x | 31.832x |
| polyglot_recompute | 0.998x | 0.998x | 1.028x |
| repetition_key | 1.134x | 1.114x | 23.545x |
| corpus_count | 1.044x | 1.029x | 1.122x |
| corpus_materialize | 1.071x | 1.046x | 1.280x |
| corpus_uci_sorted | 1.064x | 1.015x | 1.616x |
| replay_trusted_native | 1.004x | 0.992x | 0.605x |
| replay_checked_native | 1.014x | 1.006x | 1.009x |

Cold first legal move list: 0.017 ms -> 0.012 ms (1.36x).
