| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.959x | 0.938x | 1.400x |
| materialize | 0.943x | 0.849x | 1.184x |
| perft_bulk_d4 | 0.957x | 0.934x | 1.288x |
| perft_full_d3 | 0.817x | 0.683x | 0.906x |
| fen_parse | 1.036x | 1.009x | 1.302x |
| fen_write_native_ep | 0.992x | 0.981x | 1.767x |
| snapshot | 0.995x | 0.984x | 1.243x |
| san_parse_all | 0.930x | 0.919x | 1.855x |
| san_render_native | 0.909x | 0.872x | 2.000x |
| apply_children_full | 0.955x | 0.919x | 1.230x |
| make_unmake_full | 0.907x | 0.847x | 1.156x |
| startpos | 0.858x | 0.858x | 1.072x |
| chess960_materialize | 0.922x | 0.870x | 1.296x |
| chess960_perft_d3 | 0.901x | 0.830x | 1.416x |
| polyglot_key | 0.986x | 0.971x | 27.153x |
| polyglot_recompute | 0.965x | 0.961x | 0.996x |
| repetition_key | 1.003x | 0.997x | 19.773x |
| corpus_count | 0.989x | 0.975x | 1.344x |
| corpus_materialize | 1.000x | 0.997x | 1.193x |
| corpus_uci_sorted | 1.022x | 1.016x | 1.739x |
| replay_trusted_native | 0.953x | 0.929x | 0.832x |
| replay_checked_native | 0.912x | 0.878x | 0.929x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.86x).
