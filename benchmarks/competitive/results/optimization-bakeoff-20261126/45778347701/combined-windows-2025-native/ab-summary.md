| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.946x | 0.862x | 1.365x |
| materialize | 0.930x | 0.854x | 1.172x |
| perft_bulk_d4 | 0.811x | 0.691x | 1.117x |
| perft_full_d3 | 0.925x | 0.788x | 1.208x |
| fen_parse | 1.035x | 0.972x | 1.288x |
| fen_write_native_ep | 0.945x | 0.847x | 0.973x |
| snapshot | 1.283x | 1.133x | 1.246x |
| san_parse_all | 0.787x | 0.531x | 1.677x |
| san_render_native | 0.874x | 0.766x | 3.466x |
| apply_children_full | 0.993x | 0.771x | 1.021x |
| make_unmake_full | 0.793x | 0.706x | 1.079x |
| startpos | 1.173x | 1.173x | 1.579x |
| chess960_materialize | 0.965x | 0.910x | 1.286x |
| chess960_perft_d3 | 0.803x | 0.734x | 1.110x |
| polyglot_key | 1.009x | 0.969x | 30.470x |
| polyglot_recompute | 0.836x | 0.753x | 0.934x |
| repetition_key | 1.053x | 1.051x | 21.880x |
| corpus_count | 0.986x | 0.949x | 1.456x |
| corpus_materialize | 0.923x | 0.876x | 1.112x |
| corpus_uci_sorted | 0.986x | 0.964x | 1.482x |
| replay_trusted_native | 0.833x | 0.793x | 0.564x |
| replay_checked_native | 0.835x | 0.780x | 0.924x |

Cold first legal move list: 0.027 ms -> 0.037 ms (0.71x).
