| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.970x | 1.551x |
| materialize | 1.018x | 1.001x | 1.354x |
| perft_bulk_d4 | 1.035x | 1.012x | 1.442x |
| perft_full_d3 | 1.132x | 1.003x | 1.505x |
| fen_parse | 0.998x | 0.990x | 1.218x |
| fen_write_native_ep | 0.999x | 0.996x | 1.569x |
| snapshot | 0.994x | 0.988x | 0.947x |
| san_parse_all | 0.980x | 0.974x | 1.782x |
| san_render_native | 0.995x | 0.983x | 2.695x |
| apply_children_full | 0.991x | 0.963x | 1.154x |
| make_unmake_full | 1.009x | 1.000x | 1.571x |
| startpos | 1.001x | 1.001x | 1.308x |
| chess960_materialize | 1.011x | 1.010x | 1.502x |
| chess960_perft_d3 | 1.018x | 1.004x | 1.519x |
| polyglot_key | 1.006x | 0.999x | 26.194x |
| polyglot_recompute | 0.997x | 0.997x | 1.087x |
| repetition_key | 1.016x | 1.000x | 19.709x |
| corpus_count | 1.096x | 1.076x | 1.483x |
| corpus_materialize | 1.008x | 1.001x | 1.245x |
| corpus_uci_sorted | 0.995x | 0.993x | 1.459x |
| replay_trusted_native | 0.987x | 0.972x | 0.628x |
| replay_checked_native | 0.999x | 0.997x | 1.127x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.85x).
