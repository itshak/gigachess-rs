| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.993x | 1.287x |
| materialize | 1.035x | 1.025x | 1.343x |
| perft_bulk_d4 | 0.978x | 0.959x | 1.236x |
| perft_full_d3 | 0.998x | 0.971x | 1.300x |
| fen_parse | 0.872x | 0.820x | 1.066x |
| fen_write_native_ep | 0.986x | 0.965x | 1.734x |
| snapshot | 0.997x | 0.990x | 0.976x |
| san_parse_all | 0.984x | 0.974x | 2.156x |
| san_render_native | 1.018x | 0.984x | 2.314x |
| apply_children_full | 1.052x | 1.011x | 1.313x |
| make_unmake_full | 1.004x | 0.976x | 1.377x |
| startpos | 0.995x | 0.995x | 1.332x |
| chess960_materialize | 1.037x | 1.011x | 1.647x |
| chess960_perft_d3 | 0.995x | 0.981x | 1.571x |
| polyglot_key | 0.993x | 0.974x | 21.467x |
| polyglot_recompute | 1.067x | 0.976x | 1.036x |
| repetition_key | 1.004x | 0.997x | 20.228x |
| corpus_count | 1.001x | 0.999x | 1.227x |
| corpus_materialize | 0.999x | 0.996x | 1.386x |
| corpus_uci_sorted | 0.997x | 0.991x | 1.613x |
| replay_trusted_native | 0.993x | 0.950x | 0.941x |
| replay_checked_native | 0.991x | 0.986x | 1.058x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.01x).
