| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.964x | 0.911x | 1.397x |
| materialize | 0.980x | 0.938x | 1.143x |
| perft_bulk_d4 | 0.982x | 0.943x | 1.278x |
| perft_full_d3 | 1.085x | 1.033x | 1.015x |
| fen_parse | 1.024x | 1.001x | 1.236x |
| fen_write_native_ep | 1.004x | 0.999x | 1.853x |
| snapshot | 0.529x | 0.510x | 0.654x |
| san_parse_all | 1.020x | 0.977x | 2.284x |
| san_render_native | 0.976x | 0.904x | 2.101x |
| apply_children_full | 1.119x | 1.028x | 1.177x |
| make_unmake_full | 0.805x | 0.670x | 0.977x |
| startpos | 1.000x | 1.000x | 1.198x |
| chess960_materialize | 1.033x | 0.987x | 1.527x |
| chess960_perft_d3 | 1.004x | 0.980x | 1.496x |
| polyglot_key | 1.015x | 0.999x | 20.908x |
| polyglot_recompute | 1.113x | 0.999x | 1.132x |
| repetition_key | 0.882x | 0.875x | 15.667x |
| corpus_count | 0.958x | 0.838x | 1.257x |
| corpus_materialize | 1.000x | 0.958x | 1.215x |
| corpus_uci_sorted | 1.000x | 0.990x | 1.649x |
| replay_trusted_native | 0.985x | 0.939x | 0.743x |
| replay_checked_native | 1.060x | 1.033x | 0.995x |

Cold first legal move list: 0.011 ms -> 0.007 ms (1.62x).
