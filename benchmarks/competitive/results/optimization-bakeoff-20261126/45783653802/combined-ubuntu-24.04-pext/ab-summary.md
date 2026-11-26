| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.967x | 1.482x |
| materialize | 1.150x | 1.047x | 1.553x |
| perft_bulk_d4 | 1.008x | 0.998x | 1.268x |
| perft_full_d3 | 1.003x | 0.961x | 1.531x |
| fen_parse | 1.128x | 1.087x | 1.178x |
| fen_write_native_ep | 1.002x | 0.969x | 1.312x |
| snapshot | 1.826x | 1.813x | 1.329x |
| san_parse_all | 0.994x | 0.966x | 1.899x |
| san_render_native | 1.008x | 1.004x | 2.107x |
| apply_children_full | 1.051x | 1.018x | 1.044x |
| make_unmake_full | 1.018x | 1.002x | 1.149x |
| startpos | 1.503x | 1.503x | 1.247x |
| chess960_materialize | 1.084x | 1.083x | 1.450x |
| chess960_perft_d3 | 0.999x | 0.998x | 1.384x |
| polyglot_key | 0.977x | 0.951x | 38.844x |
| polyglot_recompute | 0.943x | 0.888x | 0.961x |
| repetition_key | 0.875x | 0.777x | 19.255x |
| corpus_count | 1.007x | 0.998x | 1.360x |
| corpus_materialize | 1.007x | 1.003x | 1.143x |
| corpus_uci_sorted | 0.997x | 0.989x | 1.560x |
| replay_trusted_native | 1.008x | 0.995x | 0.704x |
| replay_checked_native | 1.004x | 0.999x | 1.081x |

Cold first legal move list: 0.008 ms -> 0.010 ms (0.80x).
