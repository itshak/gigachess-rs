| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.984x | 1.154x |
| materialize | 0.922x | 0.880x | 1.212x |
| perft_bulk_d4 | 1.001x | 0.994x | 1.165x |
| perft_full_d3 | 0.992x | 0.951x | 1.320x |
| fen_parse | 1.021x | 0.975x | 1.233x |
| fen_write_native_ep | 1.005x | 0.994x | 1.563x |
| snapshot | 1.002x | 0.987x | 1.000x |
| san_parse_all | 0.977x | 0.962x | 1.768x |
| san_render_native | 0.967x | 0.927x | 2.268x |
| apply_children_full | 0.925x | 0.870x | 0.952x |
| make_unmake_full | 0.999x | 0.949x | 1.158x |
| startpos | 0.977x | 0.977x | 1.282x |
| chess960_materialize | 1.110x | 0.994x | 1.332x |
| chess960_perft_d3 | 0.957x | 0.923x | 1.302x |
| polyglot_key | 0.967x | 0.961x | 31.078x |
| polyglot_recompute | 1.013x | 0.999x | 1.029x |
| repetition_key | 1.071x | 1.004x | 25.539x |
| corpus_count | 0.982x | 0.965x | 1.127x |
| corpus_materialize | 0.954x | 0.912x | 1.232x |
| corpus_uci_sorted | 0.984x | 0.968x | 1.592x |
| replay_trusted_native | 0.985x | 0.966x | 0.553x |
| replay_checked_native | 1.018x | 1.000x | 1.032x |

Cold first legal move list: 0.011 ms -> 0.013 ms (0.84x).
