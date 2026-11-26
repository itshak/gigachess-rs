| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.084x | 0.926x | 1.456x |
| materialize | 0.956x | 0.782x | 0.939x |
| perft_bulk_d4 | 1.048x | 0.958x | 1.248x |
| perft_full_d3 | 0.933x | 0.835x | 1.292x |
| fen_parse | 1.120x | 0.922x | 1.331x |
| fen_write_native_ep | 0.876x | 0.732x | 1.584x |
| snapshot | 1.004x | 0.636x | 1.035x |
| san_parse_all | 1.017x | 0.925x | 1.753x |
| san_render_native | 0.940x | 0.794x | 2.673x |
| apply_children_full | 1.064x | 0.904x | 1.092x |
| make_unmake_full | 1.094x | 0.834x | 1.347x |
| startpos | 0.588x | 0.588x | 0.865x |
| chess960_materialize | 1.119x | 1.083x | 1.305x |
| chess960_perft_d3 | 1.141x | 1.078x | 1.316x |
| polyglot_key | 1.147x | 1.030x | 37.605x |
| polyglot_recompute | 1.156x | 0.948x | 1.017x |
| repetition_key | 1.198x | 1.030x | 24.384x |
| corpus_count | 1.092x | 0.998x | 0.971x |
| corpus_materialize | 1.192x | 1.112x | 1.148x |
| corpus_uci_sorted | 0.974x | 0.798x | 1.365x |
| replay_trusted_native | 1.051x | 0.914x | 0.676x |
| replay_checked_native | 0.882x | 0.705x | 0.979x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.92x).
