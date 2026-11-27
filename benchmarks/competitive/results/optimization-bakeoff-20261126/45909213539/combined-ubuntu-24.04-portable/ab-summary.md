| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.990x | 1.288x |
| materialize | 0.996x | 0.984x | 1.343x |
| perft_bulk_d4 | 0.995x | 0.974x | 1.274x |
| perft_full_d3 | 1.001x | 0.991x | 1.308x |
| fen_parse | 1.042x | 0.966x | 1.280x |
| fen_write_native_ep | 1.005x | 0.997x | 1.724x |
| snapshot | 0.999x | 0.998x | 0.999x |
| san_parse_all | 0.987x | 0.964x | 2.135x |
| san_render_native | 1.002x | 0.999x | 2.266x |
| apply_children_full | 0.999x | 0.990x | 1.418x |
| make_unmake_full | 1.007x | 0.987x | 1.391x |
| startpos | 1.002x | 1.002x | 1.337x |
| chess960_materialize | 1.079x | 1.063x | 1.615x |
| chess960_perft_d3 | 1.016x | 1.011x | 1.599x |
| polyglot_key | 1.000x | 0.997x | 21.812x |
| polyglot_recompute | 1.001x | 0.999x | 0.982x |
| repetition_key | 1.005x | 1.003x | 20.734x |
| corpus_count | 0.999x | 0.999x | 1.232x |
| corpus_materialize | 0.984x | 0.969x | 1.327x |
| corpus_uci_sorted | 1.010x | 0.999x | 1.593x |
| replay_trusted_native | 1.001x | 0.995x | 1.090x |
| replay_checked_native | 1.001x | 0.996x | 1.162x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.22x).
