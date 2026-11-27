| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.997x | 1.476x |
| materialize | 1.000x | 0.993x | 1.267x |
| perft_bulk_d4 | 0.994x | 0.986x | 1.400x |
| perft_full_d3 | 1.012x | 0.998x | 1.293x |
| fen_parse | 0.958x | 0.931x | 1.135x |
| fen_write_native_ep | 0.988x | 0.965x | 1.697x |
| snapshot | 0.997x | 0.992x | 1.246x |
| san_parse_all | 1.007x | 0.986x | 2.064x |
| san_render_native | 1.001x | 0.994x | 2.197x |
| apply_children_full | 1.039x | 1.004x | 1.377x |
| make_unmake_full | 0.992x | 0.963x | 1.292x |
| startpos | 1.000x | 1.000x | 1.251x |
| chess960_materialize | 0.973x | 0.968x | 1.368x |
| chess960_perft_d3 | 1.001x | 0.989x | 1.504x |
| polyglot_key | 1.071x | 1.061x | 25.695x |
| polyglot_recompute | 0.969x | 0.964x | 1.007x |
| repetition_key | 0.975x | 0.963x | 19.068x |
| corpus_count | 0.984x | 0.975x | 1.434x |
| corpus_materialize | 0.992x | 0.987x | 1.184x |
| corpus_uci_sorted | 0.994x | 0.992x | 1.824x |
| replay_trusted_native | 0.977x | 0.964x | 0.871x |
| replay_checked_native | 0.970x | 0.965x | 1.113x |

Cold first legal move list: 0.007 ms -> 0.009 ms (0.83x).
