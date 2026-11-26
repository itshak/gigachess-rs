| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.967x | 0.920x | 1.236x |
| materialize | 0.950x | 0.932x | 1.629x |
| perft_bulk_d4 | 0.953x | 0.918x | 1.130x |
| perft_full_d3 | 0.992x | 0.977x | 1.311x |
| fen_parse | 1.086x | 1.020x | 1.127x |
| fen_write_native_ep | 1.005x | 0.998x | 1.867x |
| snapshot | 0.998x | 0.995x | 1.000x |
| san_parse_all | 1.020x | 1.012x | 1.932x |
| san_render_native | 1.026x | 0.986x | 2.231x |
| apply_children_full | 1.002x | 1.000x | 1.386x |
| make_unmake_full | 1.088x | 1.024x | 1.255x |
| startpos | 0.999x | 0.999x | 1.286x |
| chess960_materialize | 0.926x | 0.922x | 1.475x |
| chess960_perft_d3 | 0.999x | 0.988x | 1.505x |
| polyglot_key | 0.965x | 0.931x | 39.221x |
| polyglot_recompute | 0.992x | 0.990x | 0.937x |
| repetition_key | 0.961x | 0.930x | 19.775x |
| corpus_count | 0.912x | 0.904x | 1.034x |
| corpus_materialize | 0.960x | 0.943x | 1.092x |
| corpus_uci_sorted | 0.989x | 0.989x | 1.532x |
| replay_trusted_native | 0.999x | 0.994x | 0.864x |
| replay_checked_native | 1.006x | 1.002x | 1.158x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.91x).
