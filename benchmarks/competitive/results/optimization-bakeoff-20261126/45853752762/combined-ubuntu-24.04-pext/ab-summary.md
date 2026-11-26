| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.997x | 1.546x |
| materialize | 1.001x | 1.000x | 1.206x |
| perft_bulk_d4 | 1.007x | 0.990x | 1.400x |
| perft_full_d3 | 0.998x | 0.988x | 1.211x |
| fen_parse | 0.848x | 0.841x | 1.008x |
| fen_write_native_ep | 1.003x | 0.994x | 1.868x |
| snapshot | 1.000x | 0.997x | 1.248x |
| san_parse_all | 0.985x | 0.973x | 2.185x |
| san_render_native | 1.011x | 0.990x | 2.138x |
| apply_children_full | 0.837x | 0.786x | 1.075x |
| make_unmake_full | 0.965x | 0.890x | 1.158x |
| startpos | 0.999x | 0.999x | 1.201x |
| chess960_materialize | 1.044x | 1.024x | 1.565x |
| chess960_perft_d3 | 0.998x | 0.993x | 1.622x |
| polyglot_key | 0.995x | 0.994x | 20.284x |
| polyglot_recompute | 0.982x | 0.957x | 1.005x |
| repetition_key | 0.992x | 0.979x | 18.583x |
| corpus_count | 1.003x | 1.001x | 1.368x |
| corpus_materialize | 0.999x | 0.997x | 1.269x |
| corpus_uci_sorted | 0.976x | 0.974x | 1.660x |
| replay_trusted_native | 0.821x | 0.793x | 0.859x |
| replay_checked_native | 0.961x | 0.943x | 1.004x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.00x).
