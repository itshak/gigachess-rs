| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 1.003x | 1.194x |
| materialize | 1.009x | 0.995x | 1.367x |
| perft_bulk_d4 | 1.020x | 1.005x | 0.892x |
| perft_full_d3 | 1.000x | 0.996x | 0.619x |
| fen_parse | 0.998x | 0.974x | 1.031x |
| fen_write_native_ep | 1.042x | 1.026x | 1.387x |
| snapshot | 1.417x | 1.407x | 0.790x |
| san_parse_all | 1.002x | 0.997x | 0.906x |
| san_render_native | 0.998x | 0.990x | 1.062x |
| apply_children_full | 1.044x | 1.037x | 0.667x |
| make_unmake_full | 1.005x | 1.000x | 0.754x |
| startpos | 1.121x | 1.121x | 0.663x |
| chess960_materialize | 1.005x | 1.003x | 1.207x |
| chess960_perft_d3 | 1.016x | 1.010x | 0.971x |
| polyglot_key | 1.038x | 0.889x | 36.217x |
| polyglot_recompute | 0.971x | 0.964x | 0.988x |
| repetition_key | 0.851x | 0.789x | 14.444x |
| corpus_count | 1.010x | 1.001x | 1.156x |
| corpus_materialize | 0.992x | 0.989x | 1.073x |
| corpus_uci_sorted | 1.017x | 1.013x | 0.638x |
| replay_trusted_native | 1.002x | 1.002x | 0.511x |
| replay_checked_native | 1.004x | 1.003x | 0.765x |

Cold first legal move list: 138.004 ms → 151.869 ms (0.91x).
