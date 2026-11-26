| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.978x | 0.960x | 1.458x |
| materialize | 0.947x | 0.939x | 1.133x |
| perft_bulk_d4 | 0.918x | 0.851x | 1.226x |
| perft_full_d3 | 0.713x | 0.661x | 0.661x |
| fen_parse | 0.852x | 0.788x | 1.117x |
| fen_write_native_ep | 0.961x | 0.924x | 1.730x |
| snapshot | 0.730x | 0.728x | 0.999x |
| san_parse_all | 0.869x | 0.858x | 1.887x |
| san_render_native | 0.921x | 0.878x | 1.934x |
| apply_children_full | 0.543x | 0.468x | 0.750x |
| make_unmake_full | 0.918x | 0.874x | 1.122x |
| startpos | 0.745x | 0.745x | 0.965x |
| chess960_materialize | 0.914x | 0.865x | 1.414x |
| chess960_perft_d3 | 0.867x | 0.795x | 1.391x |
| polyglot_key | 1.030x | 0.999x | 21.236x |
| polyglot_recompute | 1.000x | 0.982x | 0.981x |
| repetition_key | 0.988x | 0.917x | 18.687x |
| corpus_count | 0.981x | 0.971x | 1.265x |
| corpus_materialize | 0.992x | 0.979x | 1.269x |
| corpus_uci_sorted | 0.984x | 0.979x | 1.642x |
| replay_trusted_native | 0.870x | 0.807x | 0.986x |
| replay_checked_native | 0.904x | 0.855x | 0.915x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.25x).
