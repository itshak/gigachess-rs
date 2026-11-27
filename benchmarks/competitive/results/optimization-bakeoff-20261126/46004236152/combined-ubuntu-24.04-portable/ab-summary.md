| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.056x | 0.998x | 1.350x |
| materialize | 1.000x | 0.988x | 1.741x |
| perft_bulk_d4 | 1.065x | 1.039x | 1.268x |
| perft_full_d3 | 1.163x | 1.037x | 1.544x |
| fen_parse | 1.004x | 0.954x | 1.147x |
| fen_write_native_ep | 1.022x | 0.989x | 2.072x |
| snapshot | 0.998x | 0.989x | 0.999x |
| san_parse_all | 1.007x | 0.980x | 1.910x |
| san_render_native | 0.980x | 0.970x | 2.140x |
| apply_children_full | 1.010x | 1.001x | 1.386x |
| make_unmake_full | 1.058x | 1.000x | 1.322x |
| startpos | 1.004x | 1.004x | 1.359x |
| chess960_materialize | 1.056x | 1.053x | 1.546x |
| chess960_perft_d3 | 1.016x | 1.002x | 1.527x |
| polyglot_key | 1.019x | 0.999x | 40.885x |
| polyglot_recompute | 1.040x | 1.015x | 0.995x |
| repetition_key | 1.204x | 1.093x | 24.657x |
| corpus_count | 1.182x | 1.111x | 1.339x |
| corpus_materialize | 0.991x | 0.982x | 1.133x |
| corpus_uci_sorted | 0.975x | 0.971x | 1.549x |
| replay_trusted_native | 1.028x | 1.021x | 0.901x |
| replay_checked_native | 1.005x | 0.991x | 1.179x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.95x).
