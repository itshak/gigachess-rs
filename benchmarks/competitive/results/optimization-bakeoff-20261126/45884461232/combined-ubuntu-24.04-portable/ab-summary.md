| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.973x | 1.279x |
| materialize | 0.996x | 0.960x | 1.277x |
| perft_bulk_d4 | 0.973x | 0.940x | 1.241x |
| perft_full_d3 | 0.979x | 0.970x | 1.252x |
| fen_parse | 1.171x | 1.130x | 1.319x |
| fen_write_native_ep | 0.992x | 0.979x | 1.692x |
| snapshot | 1.000x | 0.999x | 0.981x |
| san_parse_all | 1.003x | 0.995x | 1.977x |
| san_render_native | 0.998x | 0.988x | 2.222x |
| apply_children_full | 0.783x | 0.747x | 1.108x |
| make_unmake_full | 0.894x | 0.859x | 1.240x |
| startpos | 0.999x | 0.999x | 1.332x |
| chess960_materialize | 0.949x | 0.945x | 1.439x |
| chess960_perft_d3 | 1.002x | 0.990x | 1.572x |
| polyglot_key | 0.999x | 0.996x | 21.797x |
| polyglot_recompute | 1.009x | 1.002x | 0.985x |
| repetition_key | 1.001x | 0.994x | 20.579x |
| corpus_count | 0.998x | 0.996x | 1.230x |
| corpus_materialize | 0.985x | 0.974x | 1.284x |
| corpus_uci_sorted | 1.002x | 0.998x | 1.784x |
| replay_trusted_native | 0.932x | 0.929x | 0.983x |
| replay_checked_native | 0.968x | 0.967x | 1.120x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.02x).
