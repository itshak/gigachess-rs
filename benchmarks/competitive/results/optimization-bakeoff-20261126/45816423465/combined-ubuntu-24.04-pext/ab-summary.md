| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.939x | 0.893x | 1.390x |
| materialize | 0.967x | 0.944x | 1.453x |
| perft_bulk_d4 | 0.851x | 0.766x | 1.062x |
| perft_full_d3 | 0.980x | 0.913x | 1.456x |
| fen_parse | 0.935x | 0.854x | 1.058x |
| fen_write_native_ep | 1.011x | 0.995x | 2.125x |
| snapshot | 0.999x | 0.988x | 1.151x |
| san_parse_all | 0.982x | 0.963x | 2.066x |
| san_render_native | 0.998x | 0.986x | 2.172x |
| apply_children_full | 0.996x | 0.915x | 1.185x |
| make_unmake_full | 0.987x | 0.953x | 1.097x |
| startpos | 1.014x | 1.014x | 1.118x |
| chess960_materialize | 1.037x | 1.013x | 1.332x |
| chess960_perft_d3 | 1.001x | 0.979x | 1.379x |
| polyglot_key | 0.978x | 0.959x | 38.751x |
| polyglot_recompute | 1.011x | 1.009x | 1.018x |
| repetition_key | 0.850x | 0.746x | 16.449x |
| corpus_count | 0.662x | 0.592x | 0.929x |
| corpus_materialize | 0.956x | 0.949x | 1.073x |
| corpus_uci_sorted | 0.985x | 0.979x | 1.575x |
| replay_trusted_native | 1.000x | 0.972x | 0.837x |
| replay_checked_native | 0.981x | 0.963x | 1.130x |

Cold first legal move list: 0.007 ms -> 0.005 ms (1.43x).
