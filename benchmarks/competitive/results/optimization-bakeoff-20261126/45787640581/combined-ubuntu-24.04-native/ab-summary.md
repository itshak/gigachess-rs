| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.970x | 0.920x | 1.363x |
| materialize | 0.987x | 0.936x | 1.479x |
| perft_bulk_d4 | 0.853x | 0.766x | 1.025x |
| perft_full_d3 | 0.964x | 0.888x | 1.467x |
| fen_parse | 0.951x | 0.877x | 1.087x |
| fen_write_native_ep | 1.004x | 0.993x | 1.882x |
| snapshot | 0.774x | 0.766x | 1.149x |
| san_parse_all | 0.970x | 0.959x | 2.001x |
| san_render_native | 0.986x | 0.978x | 2.104x |
| apply_children_full | 0.976x | 0.964x | 1.095x |
| make_unmake_full | 1.022x | 0.997x | 1.146x |
| startpos | 0.912x | 0.912x | 1.212x |
| chess960_materialize | 0.965x | 0.954x | 1.395x |
| chess960_perft_d3 | 1.017x | 0.974x | 1.406x |
| polyglot_key | 1.022x | 0.997x | 40.705x |
| polyglot_recompute | 1.006x | 1.005x | 1.057x |
| repetition_key | 0.939x | 0.737x | 19.839x |
| corpus_count | 0.690x | 0.615x | 0.922x |
| corpus_materialize | 0.952x | 0.942x | 1.041x |
| corpus_uci_sorted | 0.993x | 0.989x | 1.542x |
| replay_trusted_native | 1.007x | 1.005x | 0.704x |
| replay_checked_native | 1.002x | 1.000x | 1.117x |

Cold first legal move list: 0.009 ms -> 0.005 ms (1.89x).
