| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.979x | 1.363x |
| materialize | 1.004x | 0.990x | 1.083x |
| perft_bulk_d4 | 0.982x | 0.963x | 1.331x |
| perft_full_d3 | 0.848x | 0.779x | 0.874x |
| fen_parse | 1.013x | 0.982x | 1.476x |
| fen_write_native_ep | 1.006x | 0.999x | 1.241x |
| snapshot | 0.999x | 0.989x | 1.109x |
| san_parse_all | 1.018x | 0.998x | 1.875x |
| san_render_native | 1.037x | 0.989x | 3.473x |
| apply_children_full | 1.014x | 0.969x | 1.046x |
| make_unmake_full | 1.020x | 0.985x | 1.216x |
| startpos | 1.003x | 1.003x | 1.082x |
| chess960_materialize | 0.994x | 0.989x | 1.186x |
| chess960_perft_d3 | 0.999x | 0.998x | 1.406x |
| polyglot_key | 0.992x | 0.983x | 20.254x |
| polyglot_recompute | 0.992x | 0.980x | 0.986x |
| repetition_key | 0.994x | 0.991x | 16.650x |
| corpus_count | 1.004x | 1.002x | 1.322x |
| corpus_materialize | 0.992x | 0.991x | 1.159x |
| corpus_uci_sorted | 1.004x | 1.000x | 1.506x |
| replay_trusted_native | 1.006x | 0.995x | 0.767x |
| replay_checked_native | 0.976x | 0.965x | 0.945x |

Cold first legal move list: 0.024 ms -> 0.026 ms (0.95x).
