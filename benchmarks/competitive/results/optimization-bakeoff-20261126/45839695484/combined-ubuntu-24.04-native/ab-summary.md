| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.962x | 1.442x |
| materialize | 1.011x | 1.001x | 1.261x |
| perft_bulk_d4 | 1.000x | 0.983x | 1.379x |
| perft_full_d3 | 1.044x | 0.976x | 1.100x |
| fen_parse | 1.024x | 1.001x | 1.134x |
| fen_write_native_ep | 0.967x | 0.913x | 1.677x |
| snapshot | 0.985x | 0.913x | 1.232x |
| san_parse_all | 1.004x | 0.999x | 2.035x |
| san_render_native | 1.000x | 0.985x | 2.181x |
| apply_children_full | 1.005x | 0.992x | 1.290x |
| make_unmake_full | 1.026x | 1.011x | 1.290x |
| startpos | 1.001x | 1.001x | 1.249x |
| chess960_materialize | 1.012x | 1.006x | 1.464x |
| chess960_perft_d3 | 0.985x | 0.983x | 1.498x |
| polyglot_key | 0.995x | 0.995x | 29.717x |
| polyglot_recompute | 1.000x | 1.000x | 1.034x |
| repetition_key | 1.000x | 0.979x | 20.490x |
| corpus_count | 0.982x | 0.974x | 1.330x |
| corpus_materialize | 1.016x | 1.016x | 1.157x |
| corpus_uci_sorted | 1.004x | 0.992x | 1.713x |
| replay_trusted_native | 0.996x | 0.986x | 0.871x |
| replay_checked_native | 1.009x | 1.006x | 1.013x |

Cold first legal move list: 0.009 ms -> 0.007 ms (1.17x).
