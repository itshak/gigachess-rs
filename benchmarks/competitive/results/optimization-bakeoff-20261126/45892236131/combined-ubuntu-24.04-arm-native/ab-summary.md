| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.925x | 0.880x | 1.437x |
| materialize | 0.968x | 0.931x | 1.279x |
| perft_bulk_d4 | 0.835x | 0.821x | 1.176x |
| perft_full_d3 | 0.755x | 0.704x | 1.002x |
| fen_parse | 1.001x | 0.977x | 1.232x |
| fen_write_native_ep | 0.944x | 0.905x | 1.507x |
| snapshot | 1.291x | 1.244x | 1.162x |
| san_parse_all | 0.946x | 0.925x | 1.738x |
| san_render_native | 0.799x | 0.742x | 2.124x |
| apply_children_full | 0.833x | 0.724x | 0.964x |
| make_unmake_full | 0.736x | 0.681x | 1.152x |
| startpos | 1.067x | 1.067x | 1.313x |
| chess960_materialize | 0.971x | 0.963x | 1.447x |
| chess960_perft_d3 | 0.873x | 0.814x | 1.315x |
| polyglot_key | 1.047x | 0.947x | 26.252x |
| polyglot_recompute | 1.008x | 1.004x | 1.101x |
| repetition_key | 1.148x | 1.058x | 21.751x |
| corpus_count | 0.924x | 0.918x | 1.300x |
| corpus_materialize | 0.985x | 0.985x | 1.217x |
| corpus_uci_sorted | 1.024x | 1.021x | 1.491x |
| replay_trusted_native | 0.824x | 0.774x | 0.526x |
| replay_checked_native | 0.879x | 0.873x | 0.996x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.31x).
