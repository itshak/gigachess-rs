| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.021x | 0.833x | 1.330x |
| materialize | 0.970x | 0.689x | 1.059x |
| perft_bulk_d4 | 0.836x | 0.731x | 1.137x |
| perft_full_d3 | 1.001x | 0.849x | 1.384x |
| fen_parse | 0.942x | 0.763x | 1.163x |
| fen_write_native_ep | 1.048x | 0.930x | 1.887x |
| snapshot | 1.201x | 0.617x | 0.802x |
| san_parse_all | 0.959x | 0.838x | 1.745x |
| san_render_native | 1.011x | 0.901x | 2.566x |
| apply_children_full | 1.078x | 0.913x | 1.119x |
| make_unmake_full | 1.106x | 0.954x | 1.352x |
| startpos | 1.197x | 1.197x | 1.206x |
| chess960_materialize | 1.162x | 0.933x | 1.557x |
| chess960_perft_d3 | 0.806x | 0.675x | 1.229x |
| polyglot_key | 0.884x | 0.826x | 36.896x |
| polyglot_recompute | 0.940x | 0.917x | 1.046x |
| repetition_key | 0.953x | 0.808x | 17.656x |
| corpus_count | 0.715x | 0.687x | 1.042x |
| corpus_materialize | 0.923x | 0.862x | 1.072x |
| corpus_uci_sorted | 0.974x | 0.881x | 1.283x |
| replay_trusted_native | 1.116x | 1.033x | 0.633x |
| replay_checked_native | 1.086x | 1.017x | 0.987x |

Cold first legal move list: 0.014 ms -> 0.007 ms (1.96x).
