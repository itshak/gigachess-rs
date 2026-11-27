| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.080x | 1.038x | 1.582x |
| materialize | 0.993x | 0.956x | 1.461x |
| perft_bulk_d4 | 1.068x | 1.050x | 1.236x |
| perft_full_d3 | 1.257x | 1.051x | 1.698x |
| fen_parse | 0.902x | 0.884x | 1.045x |
| fen_write_native_ep | 0.983x | 0.967x | 2.082x |
| snapshot | 1.001x | 0.999x | 1.155x |
| san_parse_all | 1.023x | 0.957x | 1.806x |
| san_render_native | 1.000x | 0.997x | 2.116x |
| apply_children_full | 1.007x | 1.000x | 1.337x |
| make_unmake_full | 0.982x | 0.913x | 1.175x |
| startpos | 0.998x | 0.998x | 1.117x |
| chess960_materialize | 1.010x | 1.001x | 1.383x |
| chess960_perft_d3 | 1.010x | 0.999x | 1.516x |
| polyglot_key | 0.995x | 0.991x | 39.935x |
| polyglot_recompute | 1.012x | 0.991x | 1.002x |
| repetition_key | 0.991x | 0.946x | 25.442x |
| corpus_count | 1.423x | 1.174x | 1.287x |
| corpus_materialize | 1.024x | 1.021x | 1.060x |
| corpus_uci_sorted | 1.004x | 1.003x | 1.561x |
| replay_trusted_native | 1.008x | 1.001x | 0.853x |
| replay_checked_native | 1.000x | 0.999x | 1.117x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.24x).
