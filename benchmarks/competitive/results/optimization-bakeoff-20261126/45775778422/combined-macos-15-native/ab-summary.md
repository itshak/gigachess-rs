| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.020x | 0.975x | 1.238x |
| materialize | 1.029x | 0.974x | 1.086x |
| perft_bulk_d4 | 0.972x | 0.890x | 1.252x |
| perft_full_d3 | 1.016x | 0.883x | 0.775x |
| fen_parse | 1.005x | 0.947x | 1.294x |
| fen_write_native_ep | 0.987x | 0.882x | 1.270x |
| snapshot | 0.981x | 0.853x | 0.938x |
| san_parse_all | 1.111x | 0.880x | 1.637x |
| san_render_native | 1.036x | 0.977x | 2.590x |
| apply_children_full | 1.062x | 0.959x | 0.779x |
| make_unmake_full | 1.181x | 1.061x | 1.084x |
| startpos | 1.593x | 1.593x | 1.207x |
| chess960_materialize | 1.057x | 1.057x | 1.299x |
| chess960_perft_d3 | 1.007x | 0.985x | 1.346x |
| polyglot_key | 1.023x | 0.994x | 37.066x |
| polyglot_recompute | 1.026x | 0.984x | 0.968x |
| repetition_key | 1.086x | 1.085x | 19.422x |
| corpus_count | 0.992x | 0.936x | 1.278x |
| corpus_materialize | 1.023x | 0.978x | 1.096x |
| corpus_uci_sorted | 1.051x | 1.018x | 1.042x |
| replay_trusted_native | 1.022x | 1.013x | 0.437x |
| replay_checked_native | 1.007x | 0.969x | 0.932x |

Cold first legal move list: 0.013 ms -> 0.018 ms (0.71x).
