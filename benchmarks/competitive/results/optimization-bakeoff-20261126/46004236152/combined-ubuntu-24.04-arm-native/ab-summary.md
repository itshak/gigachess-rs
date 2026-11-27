| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.982x | 1.564x |
| materialize | 1.012x | 1.000x | 1.347x |
| perft_bulk_d4 | 1.045x | 1.023x | 1.452x |
| perft_full_d3 | 1.136x | 1.011x | 1.518x |
| fen_parse | 0.983x | 0.975x | 1.199x |
| fen_write_native_ep | 1.000x | 0.998x | 1.578x |
| snapshot | 1.000x | 0.997x | 0.938x |
| san_parse_all | 0.999x | 0.998x | 1.818x |
| san_render_native | 1.001x | 0.993x | 2.698x |
| apply_children_full | 1.001x | 0.982x | 1.165x |
| make_unmake_full | 1.014x | 1.003x | 1.581x |
| startpos | 0.999x | 0.999x | 1.274x |
| chess960_materialize | 1.010x | 1.009x | 1.503x |
| chess960_perft_d3 | 1.017x | 1.002x | 1.522x |
| polyglot_key | 0.999x | 0.987x | 25.194x |
| polyglot_recompute | 0.995x | 0.992x | 1.085x |
| repetition_key | 1.000x | 1.000x | 19.040x |
| corpus_count | 1.105x | 1.086x | 1.489x |
| corpus_materialize | 1.011x | 1.006x | 1.243x |
| corpus_uci_sorted | 0.983x | 0.981x | 1.430x |
| replay_trusted_native | 1.009x | 1.004x | 0.642x |
| replay_checked_native | 1.018x | 1.010x | 1.149x |

Cold first legal move list: 0.006 ms -> 0.010 ms (0.64x).
