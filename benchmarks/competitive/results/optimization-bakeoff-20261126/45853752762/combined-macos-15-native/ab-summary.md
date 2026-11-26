| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.949x | 1.517x |
| materialize | 0.985x | 0.935x | 1.078x |
| perft_bulk_d4 | 0.979x | 0.929x | 1.216x |
| perft_full_d3 | 0.978x | 0.940x | 1.296x |
| fen_parse | 1.044x | 1.003x | 1.274x |
| fen_write_native_ep | 0.977x | 0.936x | 1.761x |
| snapshot | 0.966x | 0.943x | 0.971x |
| san_parse_all | 1.016x | 0.968x | 1.716x |
| san_render_native | 0.985x | 0.959x | 2.643x |
| apply_children_full | 0.929x | 0.900x | 1.044x |
| make_unmake_full | 0.920x | 0.811x | 1.227x |
| startpos | 0.972x | 0.972x | 1.204x |
| chess960_materialize | 0.972x | 0.959x | 1.555x |
| chess960_perft_d3 | 1.003x | 0.958x | 1.482x |
| polyglot_key | 1.041x | 0.951x | 40.046x |
| polyglot_recompute | 0.958x | 0.932x | 1.007x |
| repetition_key | 0.974x | 0.958x | 21.071x |
| corpus_count | 0.972x | 0.949x | 1.212x |
| corpus_materialize | 0.935x | 0.931x | 1.156x |
| corpus_uci_sorted | 0.951x | 0.935x | 1.368x |
| replay_trusted_native | 1.015x | 1.006x | 0.676x |
| replay_checked_native | 0.955x | 0.944x | 1.138x |

Cold first legal move list: 0.003 ms -> 0.003 ms (1.14x).
