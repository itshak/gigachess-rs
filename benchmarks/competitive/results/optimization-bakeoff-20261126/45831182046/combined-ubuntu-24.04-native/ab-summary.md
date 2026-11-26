| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.052x | 1.002x | 1.333x |
| materialize | 1.004x | 0.975x | 1.187x |
| perft_bulk_d4 | 1.042x | 0.981x | 1.264x |
| perft_full_d3 | 1.037x | 0.999x | 1.273x |
| fen_parse | 0.890x | 0.854x | 1.004x |
| fen_write_native_ep | 1.005x | 0.997x | 1.736x |
| snapshot | 1.020x | 0.999x | 1.251x |
| san_parse_all | 1.029x | 1.018x | 2.030x |
| san_render_native | 1.014x | 0.993x | 2.170x |
| apply_children_full | 1.040x | 1.018x | 1.340x |
| make_unmake_full | 1.036x | 0.997x | 1.307x |
| startpos | 1.000x | 1.000x | 1.252x |
| chess960_materialize | 1.042x | 0.994x | 1.333x |
| chess960_perft_d3 | 1.034x | 0.990x | 1.466x |
| polyglot_key | 0.923x | 0.853x | 25.053x |
| polyglot_recompute | 1.005x | 0.994x | 1.037x |
| repetition_key | 0.975x | 0.927x | 19.289x |
| corpus_count | 1.022x | 1.009x | 1.250x |
| corpus_materialize | 0.996x | 0.988x | 1.119x |
| corpus_uci_sorted | 0.998x | 0.993x | 1.796x |
| replay_trusted_native | 1.024x | 1.022x | 0.889x |
| replay_checked_native | 1.068x | 1.049x | 1.095x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.17x).
