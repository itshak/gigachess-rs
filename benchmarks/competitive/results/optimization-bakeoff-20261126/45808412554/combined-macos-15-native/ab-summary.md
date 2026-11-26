| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.169x | 0.924x | 1.476x |
| materialize | 1.060x | 1.004x | 1.192x |
| perft_bulk_d4 | 1.173x | 1.021x | 1.396x |
| perft_full_d3 | 1.067x | 0.998x | 1.379x |
| fen_parse | 1.091x | 1.037x | 1.282x |
| fen_write_native_ep | 1.003x | 0.755x | 1.738x |
| snapshot | 1.074x | 0.943x | 1.015x |
| san_parse_all | 1.094x | 0.916x | 1.933x |
| san_render_native | 1.010x | 0.775x | 2.538x |
| apply_children_full | 1.036x | 0.709x | 0.953x |
| make_unmake_full | 0.999x | 0.667x | 1.137x |
| startpos | 1.014x | 1.014x | 1.210x |
| chess960_materialize | 1.104x | 1.078x | 1.470x |
| chess960_perft_d3 | 1.267x | 1.150x | 1.651x |
| polyglot_key | 1.067x | 0.987x | 40.780x |
| polyglot_recompute | 1.083x | 1.069x | 1.011x |
| repetition_key | 0.994x | 0.930x | 18.985x |
| corpus_count | 1.059x | 0.951x | 1.321x |
| corpus_materialize | 1.133x | 1.092x | 1.144x |
| corpus_uci_sorted | 1.010x | 1.007x | 1.533x |
| replay_trusted_native | 0.938x | 0.905x | 0.594x |
| replay_checked_native | 0.969x | 0.832x | 0.996x |

Cold first legal move list: 0.012 ms -> 0.010 ms (1.19x).
