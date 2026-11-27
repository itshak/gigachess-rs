| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.995x | 1.287x |
| materialize | 1.071x | 1.039x | 1.764x |
| perft_bulk_d4 | 0.998x | 0.994x | 1.189x |
| perft_full_d3 | 0.994x | 0.966x | 1.325x |
| fen_parse | 1.021x | 0.998x | 1.165x |
| fen_write_native_ep | 1.012x | 0.981x | 1.916x |
| snapshot | 1.000x | 0.993x | 0.999x |
| san_parse_all | 0.979x | 0.965x | 1.879x |
| san_render_native | 0.970x | 0.953x | 2.145x |
| apply_children_full | 1.002x | 0.987x | 1.357x |
| make_unmake_full | 1.026x | 0.948x | 1.227x |
| startpos | 0.993x | 0.993x | 1.429x |
| chess960_materialize | 1.092x | 1.079x | 1.575x |
| chess960_perft_d3 | 1.004x | 1.001x | 1.508x |
| polyglot_key | 0.982x | 0.965x | 42.631x |
| polyglot_recompute | 1.004x | 0.993x | 1.017x |
| repetition_key | 0.987x | 0.963x | 21.247x |
| corpus_count | 1.035x | 0.998x | 1.174x |
| corpus_materialize | 1.004x | 1.002x | 1.148x |
| corpus_uci_sorted | 0.992x | 0.985x | 1.551x |
| replay_trusted_native | 0.953x | 0.940x | 0.833x |
| replay_checked_native | 0.995x | 0.995x | 1.146x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.90x).
