| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.550x |
| materialize | 0.999x | 0.997x | 1.338x |
| perft_bulk_d4 | 1.005x | 0.995x | 1.398x |
| perft_full_d3 | 0.999x | 0.990x | 1.327x |
| fen_parse | 0.997x | 0.988x | 1.203x |
| fen_write_native_ep | 1.002x | 1.000x | 1.600x |
| snapshot | 1.001x | 0.996x | 0.948x |
| san_parse_all | 1.002x | 0.999x | 1.818x |
| san_render_native | 1.010x | 1.005x | 2.540x |
| apply_children_full | 1.003x | 0.982x | 1.158x |
| make_unmake_full | 1.027x | 0.984x | 1.549x |
| startpos | 1.000x | 1.000x | 1.290x |
| chess960_materialize | 1.012x | 1.004x | 1.501x |
| chess960_perft_d3 | 1.010x | 1.009x | 1.503x |
| polyglot_key | 0.973x | 0.951x | 24.682x |
| polyglot_recompute | 1.047x | 1.029x | 1.135x |
| repetition_key | 0.979x | 0.959x | 18.675x |
| corpus_count | 1.002x | 1.000x | 1.371x |
| corpus_materialize | 0.998x | 0.997x | 1.236x |
| corpus_uci_sorted | 1.007x | 1.006x | 1.462x |
| replay_trusted_native | 0.993x | 0.993x | 0.608x |
| replay_checked_native | 0.982x | 0.980x | 1.031x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.03x).
