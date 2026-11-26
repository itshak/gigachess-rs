| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.964x | 0.871x | 1.543x |
| materialize | 0.945x | 0.902x | 1.243x |
| perft_bulk_d4 | 0.917x | 0.808x | 1.328x |
| perft_full_d3 | 1.129x | 1.008x | 1.491x |
| fen_parse | 0.854x | 0.786x | 1.115x |
| fen_write_native_ep | 0.945x | 0.894x | 1.540x |
| snapshot | 0.994x | 0.982x | 1.006x |
| san_parse_all | 0.919x | 0.707x | 2.062x |
| san_render_native | 0.969x | 0.782x | 3.896x |
| apply_children_full | 0.976x | 0.878x | 1.422x |
| make_unmake_full | 0.924x | 0.863x | 1.276x |
| startpos | 0.999x | 0.999x | 1.247x |
| chess960_materialize | 0.818x | 0.753x | 1.097x |
| chess960_perft_d3 | 0.943x | 0.879x | 1.455x |
| polyglot_key | 0.986x | 0.943x | 28.893x |
| polyglot_recompute | 1.002x | 0.955x | 1.015x |
| repetition_key | 0.995x | 0.986x | 20.888x |
| corpus_count | 1.051x | 0.978x | 1.473x |
| corpus_materialize | 1.004x | 0.994x | 1.147x |
| corpus_uci_sorted | 0.968x | 0.936x | 1.411x |
| replay_trusted_native | 0.924x | 0.884x | 0.796x |
| replay_checked_native | 0.923x | 0.884x | 1.013x |

Cold first legal move list: 0.018 ms -> 0.018 ms (1.02x).
