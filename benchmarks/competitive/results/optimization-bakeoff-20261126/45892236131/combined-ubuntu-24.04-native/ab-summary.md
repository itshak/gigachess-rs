| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.957x | 0.927x | 1.430x |
| materialize | 0.964x | 0.924x | 1.140x |
| perft_bulk_d4 | 0.768x | 0.707x | 1.029x |
| perft_full_d3 | 0.788x | 0.717x | 0.935x |
| fen_parse | 0.992x | 0.922x | 1.204x |
| fen_write_native_ep | 0.875x | 0.819x | 1.592x |
| snapshot | 0.994x | 0.990x | 1.241x |
| san_parse_all | 0.845x | 0.833x | 1.846x |
| san_render_native | 0.924x | 0.803x | 1.932x |
| apply_children_full | 0.849x | 0.793x | 1.296x |
| make_unmake_full | 0.747x | 0.680x | 0.890x |
| startpos | 1.250x | 1.250x | 1.502x |
| chess960_materialize | 0.952x | 0.935x | 1.542x |
| chess960_perft_d3 | 0.815x | 0.747x | 1.302x |
| polyglot_key | 1.085x | 1.002x | 22.105x |
| polyglot_recompute | 0.974x | 0.967x | 0.989x |
| repetition_key | 0.963x | 0.897x | 18.039x |
| corpus_count | 0.953x | 0.938x | 1.232x |
| corpus_materialize | 0.983x | 0.976x | 1.255x |
| corpus_uci_sorted | 0.987x | 0.974x | 1.676x |
| replay_trusted_native | 0.764x | 0.689x | 0.846x |
| replay_checked_native | 0.861x | 0.821x | 0.933x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.10x).
