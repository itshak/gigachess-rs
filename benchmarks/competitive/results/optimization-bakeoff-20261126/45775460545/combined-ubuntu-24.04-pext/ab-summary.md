| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.998x | 1.451x |
| materialize | 1.004x | 0.995x | 1.513x |
| perft_bulk_d4 | 0.993x | 0.981x | 1.248x |
| perft_full_d3 | 1.098x | 1.042x | 0.929x |
| fen_parse | 0.903x | 0.831x | 1.036x |
| fen_write_native_ep | 0.960x | 0.932x | 1.188x |
| snapshot | 1.666x | 1.663x | 1.154x |
| san_parse_all | 0.976x | 0.966x | 1.835x |
| san_render_native | 0.997x | 0.967x | 2.115x |
| apply_children_full | 1.029x | 0.978x | 1.005x |
| make_unmake_full | 1.036x | 0.985x | 0.969x |
| startpos | 0.827x | 0.827x | 0.830x |
| chess960_materialize | 0.987x | 0.952x | 1.370x |
| chess960_perft_d3 | 1.017x | 0.994x | 1.327x |
| polyglot_key | 0.925x | 0.856x | 36.956x |
| polyglot_recompute | 1.006x | 1.004x | 1.025x |
| repetition_key | 0.970x | 0.938x | 18.948x |
| corpus_count | 0.994x | 0.984x | 1.386x |
| corpus_materialize | 1.009x | 1.005x | 1.119x |
| corpus_uci_sorted | 0.995x | 0.994x | 0.955x |
| replay_trusted_native | 1.054x | 1.053x | 0.634x |
| replay_checked_native | 1.073x | 1.018x | 1.018x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.05x).
