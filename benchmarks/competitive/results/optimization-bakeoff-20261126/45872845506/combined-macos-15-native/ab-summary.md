| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.961x | 0.744x | 1.463x |
| materialize | 0.965x | 0.884x | 1.114x |
| perft_bulk_d4 | 0.973x | 0.850x | 1.204x |
| perft_full_d3 | 0.951x | 0.905x | 1.306x |
| fen_parse | 1.039x | 0.954x | 1.280x |
| fen_write_native_ep | 0.976x | 0.891x | 1.762x |
| snapshot | 0.979x | 0.945x | 0.997x |
| san_parse_all | 1.018x | 0.955x | 1.745x |
| san_render_native | 0.970x | 0.854x | 2.766x |
| apply_children_full | 0.992x | 0.947x | 1.122x |
| make_unmake_full | 0.979x | 0.933x | 1.302x |
| startpos | 0.994x | 0.994x | 1.208x |
| chess960_materialize | 0.973x | 0.939x | 1.508x |
| chess960_perft_d3 | 0.983x | 0.971x | 1.510x |
| polyglot_key | 0.962x | 0.956x | 39.577x |
| polyglot_recompute | 0.942x | 0.902x | 1.019x |
| repetition_key | 1.058x | 1.004x | 20.089x |
| corpus_count | 1.031x | 0.977x | 1.123x |
| corpus_materialize | 0.959x | 0.880x | 1.121x |
| corpus_uci_sorted | 0.941x | 0.885x | 1.356x |
| replay_trusted_native | 0.936x | 0.883x | 0.616x |
| replay_checked_native | 0.952x | 0.902x | 1.197x |

Cold first legal move list: 0.003 ms -> 0.003 ms (0.90x).
