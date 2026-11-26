| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.942x | 1.314x |
| materialize | 0.992x | 0.954x | 1.172x |
| perft_bulk_d4 | 0.976x | 0.918x | 1.251x |
| perft_full_d3 | 1.023x | 0.988x | 1.233x |
| fen_parse | 0.984x | 0.914x | 1.154x |
| fen_write_native_ep | 0.992x | 0.973x | 1.576x |
| snapshot | 0.788x | 0.745x | 1.035x |
| san_parse_all | 1.022x | 0.974x | 1.991x |
| san_render_native | 0.984x | 0.875x | 3.639x |
| apply_children_full | 1.069x | 1.028x | 1.537x |
| make_unmake_full | 0.995x | 0.911x | 1.249x |
| startpos | 1.086x | 1.086x | 1.196x |
| chess960_materialize | 1.066x | 1.044x | 1.284x |
| chess960_perft_d3 | 1.197x | 1.126x | 1.427x |
| polyglot_key | 0.965x | 0.942x | 29.445x |
| polyglot_recompute | 0.957x | 0.925x | 0.987x |
| repetition_key | 0.996x | 0.951x | 20.325x |
| corpus_count | 0.971x | 0.756x | 1.159x |
| corpus_materialize | 0.964x | 0.935x | 1.087x |
| corpus_uci_sorted | 1.035x | 0.869x | 1.333x |
| replay_trusted_native | 1.074x | 1.004x | 0.751x |
| replay_checked_native | 0.918x | 0.916x | 0.925x |

Cold first legal move list: 0.030 ms -> 0.017 ms (1.83x).
