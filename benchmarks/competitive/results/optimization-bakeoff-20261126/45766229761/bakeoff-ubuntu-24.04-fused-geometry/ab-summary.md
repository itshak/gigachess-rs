| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.032x | 0.948x | 1.219x |
| materialize | 1.009x | 0.971x | 1.180x |
| perft_bulk_d4 | 1.113x | 1.074x | 1.069x |
| perft_full_d3 | 1.004x | 0.977x | 0.606x |
| fen_parse | 0.972x | 0.949x | 1.119x |
| fen_write_native_ep | 1.004x | 0.994x | 0.938x |
| snapshot | 1.001x | 0.995x | 0.859x |
| san_parse_all | 0.984x | 0.971x | 0.969x |
| san_render_native | 0.982x | 0.974x | 1.219x |
| apply_children_full | 0.978x | 0.956x | 0.726x |
| make_unmake_full | 0.985x | 0.981x | 0.843x |
| startpos | 1.000x | 1.000x | 0.729x |
| chess960_materialize | 1.012x | 0.992x | 1.152x |
| chess960_perft_d3 | 1.091x | 1.084x | 1.053x |
| polyglot_key | 1.007x | 0.998x | 25.875x |
| polyglot_recompute | 0.995x | 0.989x | 1.013x |
| repetition_key | 1.007x | 0.997x | 13.956x |
| corpus_count | 1.013x | 0.999x | 1.201x |
| corpus_materialize | 1.015x | 1.010x | 1.158x |
| corpus_uci_sorted | 1.005x | 1.002x | 0.671x |
| replay_trusted_native | 0.990x | 0.985x | 0.483x |
| replay_checked_native | 1.003x | 0.994x | 0.728x |

Cold first legal move list: 270.006 ms → 271.606 ms (0.99x).
