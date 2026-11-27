| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.994x | 1.296x |
| materialize | 0.909x | 0.873x | 1.680x |
| perft_bulk_d4 | 0.989x | 0.979x | 1.181x |
| perft_full_d3 | 0.999x | 0.987x | 1.328x |
| fen_parse | 1.026x | 1.009x | 1.221x |
| fen_write_native_ep | 1.022x | 0.982x | 1.810x |
| snapshot | 1.003x | 0.999x | 1.003x |
| san_parse_all | 1.003x | 0.994x | 1.909x |
| san_render_native | 0.982x | 0.970x | 2.189x |
| apply_children_full | 1.011x | 0.995x | 1.379x |
| make_unmake_full | 1.061x | 1.004x | 1.275x |
| startpos | 1.000x | 1.000x | 1.358x |
| chess960_materialize | 1.002x | 1.000x | 1.565x |
| chess960_perft_d3 | 1.009x | 1.001x | 1.503x |
| polyglot_key | 1.010x | 1.000x | 41.125x |
| polyglot_recompute | 1.061x | 1.048x | 1.024x |
| repetition_key | 1.041x | 1.012x | 21.011x |
| corpus_count | 1.034x | 0.999x | 1.169x |
| corpus_materialize | 0.999x | 0.992x | 1.142x |
| corpus_uci_sorted | 1.009x | 1.007x | 1.568x |
| replay_trusted_native | 0.955x | 0.939x | 0.887x |
| replay_checked_native | 1.011x | 0.993x | 1.204x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.98x).
