| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.951x | 0.846x | 1.303x |
| materialize | 0.969x | 0.931x | 1.075x |
| perft_bulk_d4 | 0.768x | 0.675x | 1.019x |
| perft_full_d3 | 0.869x | 0.790x | 0.880x |
| fen_parse | 0.855x | 0.742x | 1.050x |
| fen_write_native_ep | 0.977x | 0.973x | 1.604x |
| snapshot | 0.499x | 0.497x | 1.109x |
| san_parse_all | 0.855x | 0.791x | 1.679x |
| san_render_native | 0.777x | 0.697x | 2.704x |
| apply_children_full | 0.888x | 0.834x | 0.885x |
| make_unmake_full | 0.552x | 0.500x | 0.685x |
| startpos | 1.517x | 1.517x | 4.546x |
| chess960_materialize | 0.936x | 0.893x | 1.147x |
| chess960_perft_d3 | 0.704x | 0.653x | 0.990x |
| polyglot_key | 1.183x | 1.002x | 23.883x |
| polyglot_recompute | 0.971x | 0.965x | 0.966x |
| repetition_key | 1.119x | 0.981x | 19.842x |
| corpus_count | 0.993x | 0.990x | 1.307x |
| corpus_materialize | 0.996x | 0.992x | 1.165x |
| corpus_uci_sorted | 0.976x | 0.974x | 1.484x |
| replay_trusted_native | 0.833x | 0.696x | 0.646x |
| replay_checked_native | 0.706x | 0.620x | 0.670x |

Cold first legal move list: 0.022 ms -> 0.022 ms (1.00x).
