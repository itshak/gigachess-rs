| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.053x | 0.928x | 1.431x |
| materialize | 1.051x | 0.917x | 1.295x |
| perft_bulk_d4 | 0.963x | 0.901x | 1.306x |
| perft_full_d3 | 1.044x | 1.025x | 1.099x |
| fen_parse | 0.980x | 0.974x | 1.212x |
| fen_write_native_ep | 1.000x | 0.994x | 1.588x |
| snapshot | 0.956x | 0.952x | 0.949x |
| san_parse_all | 0.996x | 0.986x | 1.831x |
| san_render_native | 0.987x | 0.979x | 2.520x |
| apply_children_full | 1.066x | 1.047x | 1.196x |
| make_unmake_full | 1.002x | 0.990x | 1.447x |
| startpos | 1.038x | 1.038x | 1.302x |
| chess960_materialize | 1.094x | 1.059x | 1.464x |
| chess960_perft_d3 | 1.104x | 0.986x | 1.454x |
| polyglot_key | 0.999x | 0.993x | 24.763x |
| polyglot_recompute | 1.000x | 0.999x | 1.069x |
| repetition_key | 0.972x | 0.934x | 18.190x |
| corpus_count | 0.844x | 0.823x | 1.276x |
| corpus_materialize | 1.023x | 1.022x | 1.226x |
| corpus_uci_sorted | 0.999x | 0.998x | 1.460x |
| replay_trusted_native | 1.012x | 1.006x | 0.519x |
| replay_checked_native | 0.993x | 0.988x | 0.975x |

Cold first legal move list: 0.011 ms -> 0.005 ms (2.08x).
