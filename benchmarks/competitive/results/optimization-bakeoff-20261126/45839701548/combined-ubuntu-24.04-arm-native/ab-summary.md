| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 1.001x | 1.553x |
| materialize | 1.001x | 0.997x | 1.319x |
| perft_bulk_d4 | 1.001x | 0.998x | 1.392x |
| perft_full_d3 | 1.009x | 1.005x | 0.840x |
| fen_parse | 0.966x | 0.945x | 1.169x |
| fen_write_native_ep | 0.999x | 0.996x | 1.584x |
| snapshot | 0.990x | 0.987x | 0.946x |
| san_parse_all | 1.004x | 1.001x | 1.831x |
| san_render_native | 1.004x | 0.993x | 2.559x |
| apply_children_full | 0.999x | 0.987x | 1.156x |
| make_unmake_full | 1.003x | 0.993x | 1.565x |
| startpos | 1.000x | 1.000x | 1.302x |
| chess960_materialize | 1.000x | 0.997x | 1.501x |
| chess960_perft_d3 | 1.000x | 0.995x | 1.501x |
| polyglot_key | 1.015x | 1.005x | 25.012x |
| polyglot_recompute | 0.746x | 0.737x | 0.813x |
| repetition_key | 1.003x | 0.999x | 18.742x |
| corpus_count | 1.001x | 0.999x | 1.342x |
| corpus_materialize | 1.007x | 1.007x | 1.222x |
| corpus_uci_sorted | 1.008x | 0.999x | 1.459x |
| replay_trusted_native | 0.982x | 0.968x | 0.607x |
| replay_checked_native | 0.972x | 0.960x | 1.016x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.07x).
