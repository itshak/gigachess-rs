| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.999x | 1.358x |
| materialize | 0.998x | 0.992x | 1.244x |
| perft_bulk_d4 | 1.003x | 0.998x | 1.357x |
| perft_full_d3 | 0.997x | 0.990x | 1.054x |
| fen_parse | 0.995x | 0.991x | 1.236x |
| fen_write_native_ep | 0.999x | 0.997x | 1.572x |
| snapshot | 0.956x | 0.955x | 0.961x |
| san_parse_all | 1.003x | 0.997x | 1.827x |
| san_render_native | 0.998x | 0.994x | 2.540x |
| apply_children_full | 0.936x | 0.903x | 1.040x |
| make_unmake_full | 0.952x | 0.921x | 1.376x |
| startpos | 0.964x | 0.964x | 1.170x |
| chess960_materialize | 1.002x | 1.000x | 1.338x |
| chess960_perft_d3 | 1.003x | 1.001x | 1.327x |
| polyglot_key | 1.015x | 1.014x | 26.114x |
| polyglot_recompute | 1.004x | 1.001x | 1.076x |
| repetition_key | 0.783x | 0.633x | 15.161x |
| corpus_count | 1.016x | 1.004x | 1.461x |
| corpus_materialize | 1.012x | 1.005x | 1.212x |
| corpus_uci_sorted | 1.005x | 1.004x | 1.457x |
| replay_trusted_native | 0.919x | 0.893x | 0.475x |
| replay_checked_native | 0.986x | 0.978x | 0.966x |

Cold first legal move list: 0.011 ms -> 0.011 ms (1.00x).
