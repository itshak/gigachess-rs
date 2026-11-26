| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.991x | 1.147x |
| materialize | 1.063x | 1.039x | 1.617x |
| perft_bulk_d4 | 1.002x | 0.995x | 1.143x |
| perft_full_d3 | 1.009x | 1.001x | 0.952x |
| fen_parse | 0.917x | 0.890x | 1.073x |
| fen_write_native_ep | 0.995x | 0.982x | 1.234x |
| snapshot | 1.121x | 1.112x | 1.003x |
| san_parse_all | 0.985x | 0.950x | 1.658x |
| san_render_native | 1.018x | 1.007x | 2.088x |
| apply_children_full | 1.005x | 1.001x | 1.008x |
| make_unmake_full | 1.051x | 1.009x | 1.015x |
| startpos | 1.007x | 1.007x | 1.356x |
| chess960_materialize | 1.004x | 1.001x | 1.381x |
| chess960_perft_d3 | 1.000x | 0.998x | 1.295x |
| polyglot_key | 0.975x | 0.953x | 42.024x |
| polyglot_recompute | 1.033x | 1.029x | 1.036x |
| repetition_key | 0.947x | 0.805x | 22.771x |
| corpus_count | 0.995x | 0.995x | 1.230x |
| corpus_materialize | 1.016x | 1.005x | 1.173x |
| corpus_uci_sorted | 0.994x | 0.994x | 0.964x |
| replay_trusted_native | 1.002x | 0.994x | 0.587x |
| replay_checked_native | 1.017x | 1.001x | 0.968x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.02x).
