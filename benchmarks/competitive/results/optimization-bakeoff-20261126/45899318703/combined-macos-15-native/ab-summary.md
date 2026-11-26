| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.166x | 0.838x | 1.517x |
| materialize | 1.186x | 0.884x | 1.089x |
| perft_bulk_d4 | 1.196x | 0.918x | 1.165x |
| perft_full_d3 | 1.396x | 1.258x | 1.288x |
| fen_parse | 1.328x | 1.058x | 1.313x |
| fen_write_native_ep | 1.304x | 1.082x | 1.843x |
| snapshot | 1.078x | 0.777x | 0.935x |
| san_parse_all | 1.155x | 1.001x | 1.762x |
| san_render_native | 1.232x | 0.894x | 2.730x |
| apply_children_full | 1.186x | 0.879x | 1.089x |
| make_unmake_full | 1.199x | 0.929x | 1.287x |
| startpos | 1.605x | 1.605x | 1.990x |
| chess960_materialize | 1.266x | 1.185x | 1.520x |
| chess960_perft_d3 | 1.384x | 1.246x | 1.523x |
| polyglot_key | 1.204x | 0.903x | 34.705x |
| polyglot_recompute | 1.306x | 1.247x | 0.985x |
| repetition_key | 1.146x | 1.132x | 18.361x |
| corpus_count | 1.163x | 1.113x | 1.284x |
| corpus_materialize | 1.146x | 1.116x | 1.115x |
| corpus_uci_sorted | 1.215x | 0.935x | 1.311x |
| replay_trusted_native | 1.287x | 1.097x | 0.668x |
| replay_checked_native | 1.007x | 0.967x | 1.199x |

Cold first legal move list: 0.003 ms -> 0.005 ms (0.65x).
