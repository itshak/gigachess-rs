| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.054x | 0.999x | 1.578x |
| materialize | 1.002x | 0.990x | 1.607x |
| perft_bulk_d4 | 1.067x | 1.054x | 1.239x |
| perft_full_d3 | 1.223x | 1.038x | 1.644x |
| fen_parse | 0.940x | 0.798x | 1.052x |
| fen_write_native_ep | 0.992x | 0.967x | 2.101x |
| snapshot | 1.001x | 0.997x | 1.001x |
| san_parse_all | 1.012x | 0.984x | 1.894x |
| san_render_native | 0.982x | 0.966x | 2.079x |
| apply_children_full | 0.985x | 0.953x | 1.331x |
| make_unmake_full | 1.003x | 0.981x | 1.205x |
| startpos | 0.893x | 0.893x | 1.003x |
| chess960_materialize | 0.876x | 0.869x | 1.362x |
| chess960_perft_d3 | 1.023x | 1.001x | 1.534x |
| polyglot_key | 1.052x | 0.999x | 39.393x |
| polyglot_recompute | 1.622x | 1.310x | 1.615x |
| repetition_key | 1.289x | 1.173x | 23.892x |
| corpus_count | 1.184x | 1.112x | 1.276x |
| corpus_materialize | 1.005x | 1.001x | 1.078x |
| corpus_uci_sorted | 0.998x | 0.992x | 1.545x |
| replay_trusted_native | 1.105x | 1.069x | 0.973x |
| replay_checked_native | 0.973x | 0.962x | 1.181x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.87x).
