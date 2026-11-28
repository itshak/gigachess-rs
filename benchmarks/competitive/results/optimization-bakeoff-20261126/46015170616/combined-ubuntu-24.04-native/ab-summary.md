| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.049x | 1.022x | 1.529x |
| materialize | 1.016x | 0.991x | 1.223x |
| perft_bulk_d4 | 1.034x | 0.998x | 1.318x |
| perft_full_d3 | 1.664x | 1.410x | 1.533x |
| fen_parse | 0.915x | 0.880x | 1.118x |
| fen_write_native_ep | 1.003x | 0.971x | 1.484x |
| snapshot | 1.026x | 0.985x | 1.485x |
| san_parse_all | 1.003x | 0.987x | 1.743x |
| san_render_native | 1.017x | 0.982x | 2.312x |
| apply_children_full | 0.998x | 0.935x | 1.364x |
| make_unmake_full | 1.030x | 1.006x | 1.231x |
| startpos | 1.008x | 1.008x | 1.244x |
| chess960_materialize | 0.983x | 0.964x | 1.509x |
| chess960_perft_d3 | 1.013x | 1.011x | 1.502x |
| polyglot_key | 1.037x | 1.004x | 36.956x |
| polyglot_recompute | 0.997x | 0.995x | 0.993x |
| repetition_key | 0.839x | 0.796x | 23.570x |
| corpus_count | 1.072x | 1.051x | 1.245x |
| corpus_materialize | 1.030x | 1.016x | 1.158x |
| corpus_uci_sorted | 0.984x | 0.975x | 1.659x |
| replay_trusted_native | 1.027x | 1.017x | 0.777x |
| replay_checked_native | 1.020x | 1.018x | 1.254x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.16x).
