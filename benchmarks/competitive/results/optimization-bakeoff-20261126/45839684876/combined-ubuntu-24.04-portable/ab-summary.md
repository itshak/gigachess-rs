| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.978x | 1.297x |
| materialize | 0.987x | 0.922x | 1.346x |
| perft_bulk_d4 | 0.955x | 0.939x | 1.234x |
| perft_full_d3 | 0.967x | 0.893x | 1.085x |
| fen_parse | 0.860x | 0.814x | 1.051x |
| fen_write_native_ep | 1.016x | 1.005x | 1.790x |
| snapshot | 1.004x | 0.999x | 1.020x |
| san_parse_all | 0.996x | 0.985x | 2.006x |
| san_render_native | 0.979x | 0.957x | 2.221x |
| apply_children_full | 0.903x | 0.827x | 1.239x |
| make_unmake_full | 0.917x | 0.822x | 1.258x |
| startpos | 0.998x | 0.998x | 1.336x |
| chess960_materialize | 0.990x | 0.984x | 1.453x |
| chess960_perft_d3 | 0.962x | 0.949x | 1.479x |
| polyglot_key | 0.830x | 0.696x | 22.841x |
| polyglot_recompute | 0.986x | 0.981x | 1.015x |
| repetition_key | 0.959x | 0.876x | 17.920x |
| corpus_count | 1.007x | 1.007x | 1.304x |
| corpus_materialize | 0.999x | 0.994x | 1.208x |
| corpus_uci_sorted | 1.012x | 1.001x | 1.749x |
| replay_trusted_native | 0.996x | 0.899x | 0.865x |
| replay_checked_native | 0.998x | 0.998x | 1.051x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.13x).
