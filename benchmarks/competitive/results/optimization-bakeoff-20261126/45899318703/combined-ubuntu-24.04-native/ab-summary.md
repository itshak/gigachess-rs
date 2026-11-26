| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.992x | 1.446x |
| materialize | 1.002x | 0.990x | 1.252x |
| perft_bulk_d4 | 1.015x | 1.000x | 1.391x |
| perft_full_d3 | 1.000x | 0.996x | 1.293x |
| fen_parse | 1.092x | 1.065x | 1.214x |
| fen_write_native_ep | 0.998x | 0.980x | 1.741x |
| snapshot | 1.033x | 0.997x | 1.250x |
| san_parse_all | 0.998x | 0.990x | 2.033x |
| san_render_native | 1.011x | 0.984x | 2.224x |
| apply_children_full | 1.011x | 1.001x | 1.337x |
| make_unmake_full | 1.026x | 1.002x | 1.301x |
| startpos | 1.000x | 1.000x | 1.251x |
| chess960_materialize | 0.990x | 0.974x | 1.369x |
| chess960_perft_d3 | 0.978x | 0.963x | 1.495x |
| polyglot_key | 1.089x | 0.998x | 27.150x |
| polyglot_recompute | 1.002x | 1.001x | 1.036x |
| repetition_key | 1.020x | 0.995x | 19.456x |
| corpus_count | 0.999x | 0.993x | 1.376x |
| corpus_materialize | 1.012x | 1.005x | 1.139x |
| corpus_uci_sorted | 1.006x | 1.005x | 1.798x |
| replay_trusted_native | 0.973x | 0.966x | 0.881x |
| replay_checked_native | 1.004x | 1.002x | 1.136x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.01x).
