| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.966x | 1.569x |
| materialize | 0.998x | 0.930x | 1.099x |
| perft_bulk_d4 | 1.035x | 1.001x | 1.293x |
| perft_full_d3 | 1.110x | 1.048x | 1.508x |
| fen_parse | 0.920x | 0.831x | 1.200x |
| fen_write_native_ep | 0.991x | 0.946x | 1.754x |
| snapshot | 1.015x | 0.960x | 1.006x |
| san_parse_all | 0.943x | 0.866x | 1.672x |
| san_render_native | 1.004x | 0.967x | 2.839x |
| apply_children_full | 0.982x | 0.889x | 1.129x |
| make_unmake_full | 0.998x | 0.951x | 1.291x |
| startpos | 1.037x | 1.037x | 1.209x |
| chess960_materialize | 1.060x | 1.018x | 1.582x |
| chess960_perft_d3 | 0.974x | 0.968x | 1.533x |
| polyglot_key | 1.045x | 1.015x | 39.137x |
| polyglot_recompute | 0.983x | 0.959x | 1.001x |
| repetition_key | 0.980x | 0.971x | 19.657x |
| corpus_count | 1.033x | 1.009x | 1.293x |
| corpus_materialize | 0.983x | 0.965x | 1.136x |
| corpus_uci_sorted | 1.009x | 0.970x | 1.365x |
| replay_trusted_native | 1.022x | 0.962x | 0.673x |
| replay_checked_native | 0.909x | 0.880x | 1.209x |

Cold first legal move list: 0.003 ms -> 0.003 ms (1.00x).
