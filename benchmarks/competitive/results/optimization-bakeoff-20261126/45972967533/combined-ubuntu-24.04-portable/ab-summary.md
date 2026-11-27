| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.051x | 0.950x | 1.375x |
| materialize | 1.009x | 0.998x | 1.352x |
| perft_bulk_d4 | 1.048x | 1.000x | 1.374x |
| perft_full_d3 | 1.182x | 1.034x | 1.504x |
| fen_parse | 0.949x | 0.916x | 1.167x |
| fen_write_native_ep | 1.037x | 1.007x | 1.729x |
| snapshot | 0.994x | 0.969x | 0.970x |
| san_parse_all | 0.998x | 0.985x | 2.007x |
| san_render_native | 1.024x | 0.982x | 2.263x |
| apply_children_full | 1.002x | 0.980x | 1.412x |
| make_unmake_full | 0.990x | 0.954x | 1.386x |
| startpos | 1.004x | 1.004x | 1.335x |
| chess960_materialize | 0.981x | 0.971x | 1.478x |
| chess960_perft_d3 | 1.030x | 1.012x | 1.615x |
| polyglot_key | 1.005x | 1.003x | 25.285x |
| polyglot_recompute | 1.016x | 1.010x | 1.031x |
| repetition_key | 0.999x | 0.980x | 18.617x |
| corpus_count | 1.089x | 1.068x | 1.407x |
| corpus_materialize | 0.995x | 0.987x | 1.179x |
| corpus_uci_sorted | 0.980x | 0.979x | 1.662x |
| replay_trusted_native | 0.983x | 0.983x | 0.888x |
| replay_checked_native | 1.012x | 1.008x | 1.169x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.07x).
