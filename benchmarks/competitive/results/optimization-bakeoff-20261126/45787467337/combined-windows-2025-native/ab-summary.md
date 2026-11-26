| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.965x | 1.300x |
| materialize | 0.999x | 0.981x | 1.167x |
| perft_bulk_d4 | 0.972x | 0.919x | 1.324x |
| perft_full_d3 | 0.980x | 0.917x | 1.213x |
| fen_parse | 1.015x | 0.999x | 1.364x |
| fen_write_native_ep | 1.026x | 1.023x | 1.647x |
| snapshot | 1.305x | 1.277x | 1.022x |
| san_parse_all | 1.064x | 1.044x | 2.099x |
| san_render_native | 1.019x | 0.983x | 3.729x |
| apply_children_full | 0.983x | 0.974x | 1.190x |
| make_unmake_full | 1.002x | 0.968x | 1.286x |
| startpos | 0.998x | 0.998x | 1.107x |
| chess960_materialize | 0.994x | 0.992x | 1.239x |
| chess960_perft_d3 | 1.018x | 1.004x | 1.351x |
| polyglot_key | 0.998x | 0.974x | 31.381x |
| polyglot_recompute | 0.994x | 0.993x | 1.040x |
| repetition_key | 1.014x | 1.006x | 20.993x |
| corpus_count | 1.026x | 1.012x | 1.330x |
| corpus_materialize | 0.978x | 0.968x | 1.147x |
| corpus_uci_sorted | 1.028x | 1.016x | 1.411x |
| replay_trusted_native | 1.017x | 0.990x | 0.691x |
| replay_checked_native | 1.017x | 1.011x | 1.022x |

Cold first legal move list: 0.028 ms -> 0.027 ms (1.01x).
