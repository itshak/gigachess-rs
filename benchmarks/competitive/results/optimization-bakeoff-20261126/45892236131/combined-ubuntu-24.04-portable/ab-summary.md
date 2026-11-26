| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.929x | 0.888x | 1.180x |
| materialize | 1.053x | 1.009x | 1.665x |
| perft_bulk_d4 | 0.864x | 0.777x | 1.020x |
| perft_full_d3 | 0.938x | 0.918x | 1.254x |
| fen_parse | 0.974x | 0.900x | 1.022x |
| fen_write_native_ep | 0.943x | 0.884x | 1.699x |
| snapshot | 1.002x | 0.990x | 1.121x |
| san_parse_all | 0.946x | 0.925x | 1.926x |
| san_render_native | 0.940x | 0.872x | 2.064x |
| apply_children_full | 0.913x | 0.814x | 1.270x |
| make_unmake_full | 0.789x | 0.683x | 0.973x |
| startpos | 1.398x | 1.398x | 1.783x |
| chess960_materialize | 0.937x | 0.914x | 1.455x |
| chess960_perft_d3 | 0.841x | 0.806x | 1.267x |
| polyglot_key | 0.970x | 0.948x | 39.279x |
| polyglot_recompute | 1.064x | 1.047x | 1.010x |
| repetition_key | 0.942x | 0.933x | 19.288x |
| corpus_count | 0.953x | 0.951x | 1.077x |
| corpus_materialize | 0.959x | 0.948x | 1.095x |
| corpus_uci_sorted | 1.010x | 1.004x | 1.541x |
| replay_trusted_native | 0.966x | 0.881x | 0.840x |
| replay_checked_native | 0.919x | 0.913x | 1.061x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.74x).
