| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.987x | 0.943x | 1.173x |
| materialize | 1.052x | 1.022x | 1.284x |
| perft_bulk_d4 | 0.988x | 0.952x | 1.158x |
| perft_full_d3 | 0.999x | 0.979x | 1.168x |
| fen_parse | 1.109x | 1.082x | 1.148x |
| fen_write_native_ep | 0.999x | 0.997x | 1.799x |
| snapshot | 0.999x | 0.992x | 0.977x |
| san_parse_all | 1.026x | 1.015x | 2.178x |
| san_render_native | 1.000x | 0.985x | 2.342x |
| apply_children_full | 1.003x | 0.957x | 1.255x |
| make_unmake_full | 0.968x | 0.936x | 1.350x |
| startpos | 1.002x | 1.002x | 1.335x |
| chess960_materialize | 1.006x | 0.998x | 1.586x |
| chess960_perft_d3 | 1.004x | 1.003x | 1.473x |
| polyglot_key | 0.989x | 0.973x | 21.442x |
| polyglot_recompute | 1.025x | 1.024x | 0.992x |
| repetition_key | 0.985x | 0.967x | 20.083x |
| corpus_count | 1.000x | 1.000x | 1.144x |
| corpus_materialize | 0.988x | 0.977x | 1.257x |
| corpus_uci_sorted | 0.981x | 0.979x | 1.566x |
| replay_trusted_native | 0.991x | 0.970x | 0.956x |
| replay_checked_native | 1.088x | 1.073x | 1.171x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.99x).
