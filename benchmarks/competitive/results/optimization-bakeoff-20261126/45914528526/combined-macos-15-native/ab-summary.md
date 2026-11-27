| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.096x | 0.929x | 1.597x |
| materialize | 1.081x | 0.954x | 1.039x |
| perft_bulk_d4 | 1.174x | 0.952x | 1.316x |
| perft_full_d3 | 1.134x | 0.963x | 1.292x |
| fen_parse | 1.065x | 0.976x | 1.195x |
| fen_write_native_ep | 1.009x | 0.893x | 1.763x |
| snapshot | 1.205x | 1.000x | 1.009x |
| san_parse_all | 1.102x | 0.966x | 1.698x |
| san_render_native | 1.096x | 0.956x | 2.906x |
| apply_children_full | 1.049x | 0.912x | 1.108x |
| make_unmake_full | 1.100x | 1.012x | 1.350x |
| startpos | 1.005x | 1.005x | 1.215x |
| chess960_materialize | 1.029x | 1.024x | 1.529x |
| chess960_perft_d3 | 1.018x | 0.973x | 1.551x |
| polyglot_key | 1.144x | 1.072x | 38.447x |
| polyglot_recompute | 1.068x | 1.042x | 0.985x |
| repetition_key | 1.230x | 1.110x | 20.004x |
| corpus_count | 1.177x | 1.112x | 1.211x |
| corpus_materialize | 1.126x | 0.939x | 1.111x |
| corpus_uci_sorted | 1.128x | 0.965x | 1.348x |
| replay_trusted_native | 0.969x | 0.968x | 0.648x |
| replay_checked_native | 1.084x | 0.978x | 1.261x |

Cold first legal move list: 0.004 ms -> 0.006 ms (0.61x).
