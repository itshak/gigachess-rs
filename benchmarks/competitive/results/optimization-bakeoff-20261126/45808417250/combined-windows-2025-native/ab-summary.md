| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.991x | 1.352x |
| materialize | 1.011x | 0.999x | 1.083x |
| perft_bulk_d4 | 0.990x | 0.956x | 1.326x |
| perft_full_d3 | 0.961x | 0.912x | 1.028x |
| fen_parse | 0.992x | 0.982x | 1.430x |
| fen_write_native_ep | 1.011x | 1.000x | 1.605x |
| snapshot | 0.816x | 0.808x | 1.111x |
| san_parse_all | 0.998x | 0.986x | 1.893x |
| san_render_native | 1.026x | 1.007x | 3.453x |
| apply_children_full | 0.932x | 0.895x | 1.063x |
| make_unmake_full | 0.972x | 0.957x | 1.206x |
| startpos | 1.002x | 1.002x | 1.080x |
| chess960_materialize | 0.998x | 0.992x | 1.220x |
| chess960_perft_d3 | 1.008x | 1.005x | 1.410x |
| polyglot_key | 1.006x | 0.988x | 20.273x |
| polyglot_recompute | 0.988x | 0.972x | 0.984x |
| repetition_key | 0.845x | 0.783x | 13.913x |
| corpus_count | 0.998x | 0.996x | 1.314x |
| corpus_materialize | 1.017x | 1.006x | 1.162x |
| corpus_uci_sorted | 1.030x | 1.007x | 1.528x |
| replay_trusted_native | 0.969x | 0.953x | 0.773x |
| replay_checked_native | 1.071x | 1.024x | 0.919x |

Cold first legal move list: 0.033 ms -> 0.026 ms (1.24x).
