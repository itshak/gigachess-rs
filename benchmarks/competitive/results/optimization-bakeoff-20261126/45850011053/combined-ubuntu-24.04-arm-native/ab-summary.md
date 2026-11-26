| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.986x | 0.978x | 1.525x |
| materialize | 1.003x | 0.992x | 1.321x |
| perft_bulk_d4 | 0.968x | 0.959x | 1.348x |
| perft_full_d3 | 0.915x | 0.863x | 1.217x |
| fen_parse | 1.016x | 0.999x | 1.232x |
| fen_write_native_ep | 1.000x | 0.994x | 1.577x |
| snapshot | 0.981x | 0.978x | 0.955x |
| san_parse_all | 0.947x | 0.933x | 1.737x |
| san_render_native | 0.915x | 0.894x | 2.313x |
| apply_children_full | 1.037x | 1.016x | 1.206x |
| make_unmake_full | 0.964x | 0.933x | 1.462x |
| startpos | 0.931x | 0.931x | 1.238x |
| chess960_materialize | 0.998x | 0.997x | 1.495x |
| chess960_perft_d3 | 0.968x | 0.956x | 1.452x |
| polyglot_key | 1.016x | 1.001x | 25.510x |
| polyglot_recompute | 1.010x | 1.006x | 1.100x |
| repetition_key | 1.023x | 0.992x | 19.076x |
| corpus_count | 0.990x | 0.986x | 1.348x |
| corpus_materialize | 0.988x | 0.987x | 1.214x |
| corpus_uci_sorted | 1.000x | 0.998x | 1.463x |
| replay_trusted_native | 0.925x | 0.906x | 0.576x |
| replay_checked_native | 0.937x | 0.931x | 0.979x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.85x).
