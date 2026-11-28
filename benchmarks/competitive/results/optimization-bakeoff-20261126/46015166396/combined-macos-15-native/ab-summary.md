| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.044x | 0.879x | 1.539x |
| materialize | 1.151x | 0.929x | 1.100x |
| perft_bulk_d4 | 1.164x | 1.042x | 1.470x |
| perft_full_d3 | 1.194x | 0.970x | 1.530x |
| fen_parse | 0.907x | 0.674x | 1.135x |
| fen_write_native_ep | 0.878x | 0.741x | 1.721x |
| snapshot | 1.056x | 0.729x | 0.792x |
| san_parse_all | 0.942x | 0.823x | 1.818x |
| san_render_native | 1.073x | 0.986x | 2.886x |
| apply_children_full | 1.007x | 0.792x | 1.153x |
| make_unmake_full | 1.010x | 0.917x | 1.272x |
| startpos | 1.948x | 1.948x | 1.187x |
| chess960_materialize | 1.191x | 1.129x | 1.539x |
| chess960_perft_d3 | 1.001x | 0.945x | 1.629x |
| polyglot_key | 1.079x | 1.018x | 42.510x |
| polyglot_recompute | 1.592x | 1.296x | 1.637x |
| repetition_key | 0.922x | 0.885x | 20.033x |
| corpus_count | 1.242x | 1.198x | 1.306x |
| corpus_materialize | 0.775x | 0.597x | 0.957x |
| corpus_uci_sorted | 1.116x | 0.880x | 1.562x |
| replay_trusted_native | 1.004x | 0.839x | 0.636x |
| replay_checked_native | 1.118x | 0.979x | 1.407x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.90x).
