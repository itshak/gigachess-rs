| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.091x | 0.852x | 1.301x |
| materialize | 1.078x | 0.888x | 1.076x |
| perft_bulk_d4 | 1.000x | 0.601x | 1.220x |
| perft_full_d3 | 1.118x | 0.920x | 1.383x |
| fen_parse | 1.098x | 0.811x | 1.206x |
| fen_write_native_ep | 1.036x | 0.750x | 1.592x |
| snapshot | 1.092x | 0.915x | 0.994x |
| san_parse_all | 1.066x | 0.821x | 2.056x |
| san_render_native | 1.099x | 0.782x | 2.540x |
| apply_children_full | 1.059x | 0.617x | 0.942x |
| make_unmake_full | 1.205x | 0.868x | 1.214x |
| startpos | 1.007x | 1.007x | 1.257x |
| chess960_materialize | 0.833x | 0.788x | 1.315x |
| chess960_perft_d3 | 0.849x | 0.816x | 1.557x |
| polyglot_key | 0.898x | 0.892x | 39.816x |
| polyglot_recompute | 0.801x | 0.714x | 0.908x |
| repetition_key | 0.927x | 0.879x | 24.621x |
| corpus_count | 0.819x | 0.595x | 1.208x |
| corpus_materialize | 0.800x | 0.692x | 1.133x |
| corpus_uci_sorted | 0.757x | 0.725x | 1.271x |
| replay_trusted_native | 0.770x | 0.718x | 0.570x |
| replay_checked_native | 0.896x | 0.836x | 1.085x |

Cold first legal move list: 0.010 ms -> 0.014 ms (0.73x).
