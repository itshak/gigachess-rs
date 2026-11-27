| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.971x | 0.951x | 1.502x |
| materialize | 1.001x | 0.998x | 1.339x |
| perft_bulk_d4 | 1.016x | 0.993x | 1.424x |
| perft_full_d3 | 0.988x | 0.976x | 1.323x |
| fen_parse | 1.002x | 0.995x | 1.221x |
| fen_write_native_ep | 1.007x | 1.001x | 1.640x |
| snapshot | 1.021x | 1.017x | 0.939x |
| san_parse_all | 0.998x | 0.990x | 1.818x |
| san_render_native | 0.999x | 0.984x | 2.665x |
| apply_children_full | 1.004x | 0.996x | 1.162x |
| make_unmake_full | 1.005x | 0.995x | 1.567x |
| startpos | 0.999x | 0.999x | 1.278x |
| chess960_materialize | 1.003x | 1.002x | 1.495x |
| chess960_perft_d3 | 1.007x | 1.006x | 1.505x |
| polyglot_key | 1.012x | 1.006x | 25.956x |
| polyglot_recompute | 0.997x | 0.996x | 1.086x |
| repetition_key | 1.009x | 1.001x | 19.493x |
| corpus_count | 1.074x | 1.042x | 1.446x |
| corpus_materialize | 0.999x | 0.998x | 1.242x |
| corpus_uci_sorted | 0.997x | 0.996x | 1.460x |
| replay_trusted_native | 1.004x | 0.999x | 0.623x |
| replay_checked_native | 0.993x | 0.992x | 1.121x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.97x).
