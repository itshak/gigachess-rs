| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.990x | 1.186x |
| materialize | 1.001x | 0.996x | 1.398x |
| perft_bulk_d4 | 1.121x | 1.088x | 0.993x |
| perft_full_d3 | 1.198x | 1.171x | 0.724x |
| fen_parse | 0.926x | 0.814x | 1.012x |
| fen_write_native_ep | 1.043x | 0.999x | 1.383x |
| snapshot | 0.713x | 0.708x | 0.857x |
| san_parse_all | 0.988x | 0.966x | 0.911x |
| san_render_native | 0.999x | 0.989x | 1.063x |
| apply_children_full | 0.945x | 0.938x | 0.636x |
| make_unmake_full | 1.008x | 0.997x | 0.761x |
| startpos | 1.113x | 1.113x | 0.889x |
| chess960_materialize | 0.992x | 0.988x | 1.216x |
| chess960_perft_d3 | 1.092x | 1.067x | 1.047x |
| polyglot_key | 0.876x | 0.842x | 33.195x |
| polyglot_recompute | 0.972x | 0.967x | 0.986x |
| repetition_key | 1.110x | 0.962x | 16.332x |
| corpus_count | 1.015x | 1.009x | 1.151x |
| corpus_materialize | 0.998x | 0.998x | 1.069x |
| corpus_uci_sorted | 1.016x | 1.007x | 0.639x |
| replay_trusted_native | 0.970x | 0.955x | 0.521x |
| replay_checked_native | 0.989x | 0.981x | 0.758x |

Cold first legal move list: 137.605 ms → 147.502 ms (0.93x).
