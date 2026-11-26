| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 1.001x | 1.427x |
| materialize | 1.002x | 0.994x | 1.542x |
| perft_bulk_d4 | 0.979x | 0.966x | 1.028x |
| perft_full_d3 | 1.003x | 0.997x | 1.456x |
| fen_parse | 0.945x | 0.833x | 1.002x |
| fen_write_native_ep | 0.984x | 0.959x | 2.117x |
| snapshot | 1.004x | 0.999x | 1.330x |
| san_parse_all | 1.001x | 0.994x | 2.040x |
| san_render_native | 1.007x | 1.003x | 2.153x |
| apply_children_full | 1.002x | 0.998x | 1.211x |
| make_unmake_full | 1.001x | 0.987x | 1.156x |
| startpos | 1.005x | 1.005x | 1.256x |
| chess960_materialize | 1.037x | 1.013x | 1.404x |
| chess960_perft_d3 | 0.987x | 0.977x | 1.359x |
| polyglot_key | 1.012x | 1.006x | 40.280x |
| polyglot_recompute | 0.948x | 0.933x | 0.970x |
| repetition_key | 1.006x | 0.994x | 18.481x |
| corpus_count | 1.002x | 1.002x | 0.965x |
| corpus_materialize | 1.004x | 1.003x | 1.053x |
| corpus_uci_sorted | 0.990x | 0.990x | 1.545x |
| replay_trusted_native | 1.000x | 0.990x | 0.892x |
| replay_checked_native | 1.023x | 0.995x | 1.093x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.89x).
