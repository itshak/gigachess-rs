| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.060x | 0.799x | 1.621x |
| materialize | 1.005x | 0.829x | 1.120x |
| perft_bulk_d4 | 1.109x | 0.884x | 1.415x |
| perft_full_d3 | 0.954x | 0.758x | 1.266x |
| fen_parse | 1.080x | 0.864x | 1.199x |
| fen_write_native_ep | 0.998x | 0.830x | 1.786x |
| snapshot | 0.750x | 0.385x | 0.708x |
| san_parse_all | 1.036x | 0.956x | 1.902x |
| san_render_native | 0.958x | 0.853x | 2.871x |
| apply_children_full | 0.956x | 0.715x | 1.167x |
| make_unmake_full | 0.931x | 0.736x | 1.233x |
| startpos | 1.114x | 1.114x | 1.919x |
| chess960_materialize | 0.823x | 0.811x | 1.287x |
| chess960_perft_d3 | 1.197x | 1.161x | 1.715x |
| polyglot_key | 1.031x | 1.005x | 42.580x |
| polyglot_recompute | 0.812x | 0.803x | 0.875x |
| repetition_key | 1.041x | 0.876x | 22.108x |
| corpus_count | 1.164x | 1.100x | 1.285x |
| corpus_materialize | 0.978x | 0.921x | 1.167x |
| corpus_uci_sorted | 1.104x | 1.002x | 1.567x |
| replay_trusted_native | 0.959x | 0.946x | 0.611x |
| replay_checked_native | 1.021x | 0.964x | 1.224x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.84x).
