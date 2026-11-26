| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.221x | 0.803x | 1.395x |
| materialize | 1.089x | 0.922x | 1.065x |
| perft_bulk_d4 | 1.036x | 0.979x | 1.156x |
| perft_full_d3 | 0.757x | 0.643x | 0.960x |
| fen_parse | 1.136x | 0.948x | 1.230x |
| fen_write_native_ep | 1.042x | 0.791x | 1.659x |
| snapshot | 1.131x | 0.727x | 1.068x |
| san_parse_all | 1.127x | 1.007x | 1.876x |
| san_render_native | 1.082x | 0.879x | 2.811x |
| apply_children_full | 1.125x | 0.954x | 1.154x |
| make_unmake_full | 0.928x | 0.661x | 1.109x |
| startpos | 2.015x | 2.015x | 2.406x |
| chess960_materialize | 1.130x | 1.114x | 1.488x |
| chess960_perft_d3 | 0.958x | 0.853x | 1.321x |
| polyglot_key | 1.163x | 1.134x | 40.565x |
| polyglot_recompute | 1.109x | 1.059x | 1.023x |
| repetition_key | 1.225x | 1.129x | 21.429x |
| corpus_count | 1.030x | 0.961x | 1.088x |
| corpus_materialize | 1.170x | 1.116x | 1.129x |
| corpus_uci_sorted | 1.111x | 1.059x | 1.511x |
| replay_trusted_native | 1.048x | 1.041x | 0.613x |
| replay_checked_native | 1.107x | 1.038x | 1.182x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.85x).
