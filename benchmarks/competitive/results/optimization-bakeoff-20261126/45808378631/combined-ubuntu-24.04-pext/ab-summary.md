| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.979x | 0.907x | 1.328x |
| materialize | 1.057x | 1.002x | 1.170x |
| perft_bulk_d4 | 0.937x | 0.866x | 1.171x |
| perft_full_d3 | 1.111x | 1.050x | 1.179x |
| fen_parse | 0.986x | 0.958x | 1.244x |
| fen_write_native_ep | 0.992x | 0.966x | 1.477x |
| snapshot | 1.079x | 0.795x | 1.400x |
| san_parse_all | 0.992x | 0.968x | 1.940x |
| san_render_native | 1.094x | 0.992x | 2.325x |
| apply_children_full | 1.495x | 1.333x | 1.253x |
| make_unmake_full | 1.019x | 0.975x | 1.311x |
| startpos | 0.991x | 0.991x | 1.246x |
| chess960_materialize | 1.098x | 1.022x | 1.398x |
| chess960_perft_d3 | 0.985x | 0.920x | 1.359x |
| polyglot_key | 1.024x | 0.999x | 36.822x |
| polyglot_recompute | 1.007x | 1.001x | 0.999x |
| repetition_key | 0.545x | 0.506x | 14.403x |
| corpus_count | 0.853x | 0.750x | 1.088x |
| corpus_materialize | 0.843x | 0.764x | 1.073x |
| corpus_uci_sorted | 0.998x | 0.985x | 1.650x |
| replay_trusted_native | 1.010x | 1.005x | 0.656x |
| replay_checked_native | 1.020x | 1.017x | 0.961x |

Cold first legal move list: 0.011 ms -> 0.006 ms (1.91x).
