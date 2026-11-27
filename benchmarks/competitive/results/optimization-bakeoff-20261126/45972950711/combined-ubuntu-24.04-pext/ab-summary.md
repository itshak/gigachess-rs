| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.015x | 0.996x | 1.492x |
| materialize | 1.006x | 0.969x | 1.248x |
| perft_bulk_d4 | 1.007x | 0.971x | 1.289x |
| perft_full_d3 | 1.031x | 0.979x | 1.254x |
| fen_parse | 0.919x | 0.849x | 1.141x |
| fen_write_native_ep | 1.001x | 0.967x | 1.555x |
| snapshot | 1.010x | 0.993x | 1.487x |
| san_parse_all | 1.018x | 1.002x | 1.940x |
| san_render_native | 0.988x | 0.952x | 2.255x |
| apply_children_full | 0.840x | 0.789x | 1.129x |
| make_unmake_full | 1.100x | 1.045x | 1.298x |
| startpos | 1.015x | 1.015x | 1.259x |
| chess960_materialize | 1.027x | 1.018x | 1.533x |
| chess960_perft_d3 | 1.012x | 0.996x | 1.515x |
| polyglot_key | 0.888x | 0.775x | 30.885x |
| polyglot_recompute | 1.058x | 0.999x | 0.971x |
| repetition_key | 0.878x | 0.788x | 23.143x |
| corpus_count | 1.024x | 1.015x | 1.176x |
| corpus_materialize | 1.015x | 1.001x | 1.157x |
| corpus_uci_sorted | 1.051x | 1.019x | 1.704x |
| replay_trusted_native | 0.976x | 0.953x | 0.732x |
| replay_checked_native | 0.992x | 0.957x | 1.215x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.11x).
