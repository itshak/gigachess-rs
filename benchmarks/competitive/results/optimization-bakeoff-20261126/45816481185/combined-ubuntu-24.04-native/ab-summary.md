| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.992x | 1.373x |
| materialize | 1.069x | 1.028x | 1.514x |
| perft_bulk_d4 | 1.012x | 0.993x | 1.041x |
| perft_full_d3 | 1.001x | 0.983x | 1.457x |
| fen_parse | 0.926x | 0.828x | 1.074x |
| fen_write_native_ep | 1.036x | 1.001x | 1.925x |
| snapshot | 1.005x | 0.999x | 0.999x |
| san_parse_all | 1.148x | 1.119x | 2.033x |
| san_render_native | 1.030x | 1.024x | 2.190x |
| apply_children_full | 1.000x | 0.988x | 1.171x |
| make_unmake_full | 1.011x | 1.002x | 1.119x |
| startpos | 1.250x | 1.250x | 1.263x |
| chess960_materialize | 1.014x | 1.004x | 1.408x |
| chess960_perft_d3 | 1.009x | 1.006x | 1.417x |
| polyglot_key | 0.955x | 0.906x | 35.186x |
| polyglot_recompute | 1.004x | 1.004x | 1.014x |
| repetition_key | 0.893x | 0.779x | 16.081x |
| corpus_count | 1.010x | 1.007x | 0.856x |
| corpus_materialize | 1.010x | 1.010x | 1.039x |
| corpus_uci_sorted | 0.997x | 0.993x | 1.578x |
| replay_trusted_native | 1.017x | 0.996x | 0.815x |
| replay_checked_native | 0.992x | 0.987x | 1.155x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.94x).
