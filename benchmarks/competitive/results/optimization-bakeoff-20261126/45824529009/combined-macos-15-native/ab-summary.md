| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.771x | 1.239x |
| materialize | 1.076x | 0.775x | 1.014x |
| perft_bulk_d4 | 1.058x | 0.756x | 1.090x |
| perft_full_d3 | 1.162x | 0.871x | 1.331x |
| fen_parse | 1.118x | 0.913x | 1.143x |
| fen_write_native_ep | 1.048x | 0.734x | 1.844x |
| snapshot | 0.928x | 0.750x | 0.931x |
| san_parse_all | 1.023x | 0.806x | 1.860x |
| san_render_native | 1.076x | 0.753x | 2.524x |
| apply_children_full | 1.167x | 0.962x | 1.117x |
| make_unmake_full | 1.126x | 0.951x | 1.237x |
| startpos | 0.876x | 0.876x | 1.425x |
| chess960_materialize | 0.967x | 0.941x | 1.463x |
| chess960_perft_d3 | 1.083x | 1.080x | 1.299x |
| polyglot_key | 1.097x | 1.078x | 39.092x |
| polyglot_recompute | 0.946x | 0.866x | 0.827x |
| repetition_key | 0.960x | 0.814x | 17.797x |
| corpus_count | 0.817x | 0.716x | 0.893x |
| corpus_materialize | 1.101x | 1.077x | 1.144x |
| corpus_uci_sorted | 0.848x | 0.741x | 1.470x |
| replay_trusted_native | 0.869x | 0.817x | 0.640x |
| replay_checked_native | 1.151x | 0.996x | 1.117x |

Cold first legal move list: 0.005 ms -> 0.004 ms (1.02x).
