| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.973x | 1.212x |
| materialize | 1.002x | 0.988x | 1.316x |
| perft_bulk_d4 | 0.998x | 0.984x | 1.172x |
| perft_full_d3 | 0.922x | 0.871x | 1.073x |
| fen_parse | 0.938x | 0.853x | 1.188x |
| fen_write_native_ep | 1.001x | 0.994x | 1.784x |
| snapshot | 1.000x | 0.999x | 1.072x |
| san_parse_all | 1.024x | 1.012x | 2.221x |
| san_render_native | 0.961x | 0.899x | 2.315x |
| apply_children_full | 0.903x | 0.854x | 1.102x |
| make_unmake_full | 0.865x | 0.811x | 1.176x |
| startpos | 0.999x | 0.999x | 1.336x |
| chess960_materialize | 0.929x | 0.865x | 1.516x |
| chess960_perft_d3 | 0.936x | 0.904x | 1.390x |
| polyglot_key | 1.004x | 1.001x | 21.992x |
| polyglot_recompute | 1.000x | 0.999x | 0.977x |
| repetition_key | 0.986x | 0.972x | 16.202x |
| corpus_count | 1.002x | 1.000x | 1.145x |
| corpus_materialize | 1.027x | 1.015x | 1.311x |
| corpus_uci_sorted | 0.982x | 0.981x | 1.550x |
| replay_trusted_native | 0.903x | 0.851x | 0.733x |
| replay_checked_native | 0.984x | 0.983x | 1.000x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.12x).
