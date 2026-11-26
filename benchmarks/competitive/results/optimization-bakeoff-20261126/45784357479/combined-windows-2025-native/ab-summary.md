| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.988x | 1.368x |
| materialize | 1.014x | 0.989x | 1.080x |
| perft_bulk_d4 | 0.978x | 0.946x | 1.312x |
| perft_full_d3 | 0.977x | 0.920x | 1.042x |
| fen_parse | 1.197x | 1.172x | 1.448x |
| fen_write_native_ep | 1.009x | 0.984x | 1.630x |
| snapshot | 0.827x | 0.812x | 1.104x |
| san_parse_all | 0.996x | 0.977x | 1.873x |
| san_render_native | 1.026x | 1.000x | 3.442x |
| apply_children_full | 1.002x | 0.980x | 1.036x |
| make_unmake_full | 1.010x | 1.000x | 1.233x |
| startpos | 2.439x | 2.439x | 1.078x |
| chess960_materialize | 1.006x | 0.997x | 1.210x |
| chess960_perft_d3 | 1.000x | 0.995x | 1.410x |
| polyglot_key | 0.998x | 0.995x | 20.398x |
| polyglot_recompute | 0.871x | 0.858x | 0.862x |
| repetition_key | 0.997x | 0.990x | 16.614x |
| corpus_count | 0.989x | 0.984x | 1.319x |
| corpus_materialize | 0.993x | 0.980x | 1.152x |
| corpus_uci_sorted | 0.997x | 0.993x | 1.490x |
| replay_trusted_native | 0.987x | 0.953x | 0.754x |
| replay_checked_native | 0.990x | 0.978x | 0.919x |

Cold first legal move list: 0.030 ms -> 0.028 ms (1.07x).
