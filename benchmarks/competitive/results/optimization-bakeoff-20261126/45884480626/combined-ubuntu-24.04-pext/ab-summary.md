| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.036x | 0.991x | 1.541x |
| materialize | 0.993x | 0.973x | 1.225x |
| perft_bulk_d4 | 0.997x | 0.964x | 1.310x |
| perft_full_d3 | 1.320x | 1.048x | 1.636x |
| fen_parse | 0.908x | 0.832x | 1.108x |
| fen_write_native_ep | 0.962x | 0.845x | 1.407x |
| snapshot | 1.015x | 0.991x | 1.493x |
| san_parse_all | 0.995x | 0.978x | 1.760x |
| san_render_native | 1.011x | 0.983x | 2.309x |
| apply_children_full | 1.004x | 0.986x | 1.352x |
| make_unmake_full | 1.016x | 0.951x | 1.216x |
| startpos | 0.991x | 0.991x | 1.255x |
| chess960_materialize | 0.980x | 0.973x | 1.521x |
| chess960_perft_d3 | 0.997x | 0.983x | 1.517x |
| polyglot_key | 0.982x | 0.967x | 35.390x |
| polyglot_recompute | 0.977x | 0.963x | 0.947x |
| repetition_key | 0.841x | 0.804x | 22.795x |
| corpus_count | 1.015x | 1.009x | 1.196x |
| corpus_materialize | 0.987x | 0.976x | 1.160x |
| corpus_uci_sorted | 1.035x | 1.002x | 1.690x |
| replay_trusted_native | 1.019x | 0.973x | 0.754x |
| replay_checked_native | 0.970x | 0.951x | 1.164x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.09x).
