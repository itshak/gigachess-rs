| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.547x |
| materialize | 0.999x | 0.995x | 1.333x |
| perft_bulk_d4 | 1.000x | 0.997x | 1.393x |
| perft_full_d3 | 0.984x | 0.957x | 1.314x |
| fen_parse | 0.995x | 0.988x | 1.212x |
| fen_write_native_ep | 0.999x | 0.996x | 1.572x |
| snapshot | 1.010x | 1.007x | 0.948x |
| san_parse_all | 0.974x | 0.967x | 1.782x |
| san_render_native | 1.010x | 0.990x | 2.710x |
| apply_children_full | 0.999x | 0.964x | 1.151x |
| make_unmake_full | 1.005x | 0.977x | 1.579x |
| startpos | 0.999x | 0.999x | 1.300x |
| chess960_materialize | 0.995x | 0.986x | 1.484x |
| chess960_perft_d3 | 0.991x | 0.984x | 1.482x |
| polyglot_key | 0.987x | 0.979x | 25.055x |
| polyglot_recompute | 0.994x | 0.989x | 1.083x |
| repetition_key | 0.988x | 0.978x | 18.798x |
| corpus_count | 0.982x | 0.975x | 1.339x |
| corpus_materialize | 0.986x | 0.984x | 1.224x |
| corpus_uci_sorted | 0.991x | 0.989x | 1.454x |
| replay_trusted_native | 0.931x | 0.922x | 0.601x |
| replay_checked_native | 0.977x | 0.974x | 1.103x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.04x).
