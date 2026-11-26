| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.993x | 1.374x |
| materialize | 0.994x | 0.984x | 1.562x |
| perft_bulk_d4 | 1.000x | 0.998x | 1.008x |
| perft_full_d3 | 0.993x | 0.968x | 1.461x |
| fen_parse | 0.992x | 0.982x | 1.081x |
| fen_write_native_ep | 1.009x | 0.991x | 2.075x |
| snapshot | 1.004x | 0.997x | 0.999x |
| san_parse_all | 1.003x | 0.996x | 1.899x |
| san_render_native | 1.015x | 1.003x | 2.147x |
| apply_children_full | 1.004x | 0.999x | 1.282x |
| make_unmake_full | 1.018x | 0.978x | 1.167x |
| startpos | 1.117x | 1.117x | 1.251x |
| chess960_materialize | 1.012x | 1.006x | 1.437x |
| chess960_perft_d3 | 1.003x | 0.999x | 1.411x |
| polyglot_key | 1.004x | 1.000x | 41.125x |
| polyglot_recompute | 0.924x | 0.917x | 0.989x |
| repetition_key | 0.827x | 0.779x | 16.431x |
| corpus_count | 1.013x | 1.008x | 0.901x |
| corpus_materialize | 1.006x | 1.005x | 1.046x |
| corpus_uci_sorted | 1.008x | 1.004x | 1.567x |
| replay_trusted_native | 1.011x | 1.004x | 0.881x |
| replay_checked_native | 0.998x | 0.997x | 1.026x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.08x).
