| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.987x | 1.188x |
| materialize | 1.004x | 0.974x | 1.403x |
| perft_bulk_d4 | 1.006x | 0.999x | 0.890x |
| perft_full_d3 | 0.997x | 0.996x | 0.593x |
| fen_parse | 0.997x | 0.965x | 1.169x |
| fen_write_native_ep | 1.015x | 0.970x | 1.368x |
| snapshot | 1.132x | 1.128x | 0.633x |
| san_parse_all | 1.014x | 1.007x | 0.932x |
| san_render_native | 0.997x | 0.987x | 1.090x |
| apply_children_full | 1.002x | 0.996x | 0.657x |
| make_unmake_full | 0.979x | 0.963x | 0.746x |
| startpos | 0.894x | 0.894x | 0.500x |
| chess960_materialize | 1.010x | 0.998x | 1.227x |
| chess960_perft_d3 | 0.998x | 0.997x | 0.955x |
| polyglot_key | 1.144x | 1.000x | 37.847x |
| polyglot_recompute | 0.953x | 0.949x | 0.966x |
| repetition_key | 1.195x | 1.112x | 16.779x |
| corpus_count | 1.004x | 1.003x | 1.135x |
| corpus_materialize | 0.994x | 0.989x | 1.078x |
| corpus_uci_sorted | 0.993x | 0.991x | 0.615x |
| replay_trusted_native | 0.997x | 0.984x | 0.516x |
| replay_checked_native | 0.998x | 0.993x | 0.764x |

Cold first legal move list: 139.199 ms → 132.443 ms (1.05x).
