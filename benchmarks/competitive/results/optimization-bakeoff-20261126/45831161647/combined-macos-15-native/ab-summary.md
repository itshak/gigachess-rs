| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.732x | 1.245x |
| materialize | 1.011x | 0.849x | 1.033x |
| perft_bulk_d4 | 1.032x | 0.919x | 1.055x |
| perft_full_d3 | 1.067x | 0.905x | 1.294x |
| fen_parse | 1.044x | 0.944x | 1.157x |
| fen_write_native_ep | 0.984x | 0.789x | 1.757x |
| snapshot | 0.953x | 0.740x | 0.951x |
| san_parse_all | 1.056x | 0.834x | 1.670x |
| san_render_native | 1.033x | 0.805x | 2.506x |
| apply_children_full | 1.047x | 0.892x | 1.075x |
| make_unmake_full | 0.933x | 0.767x | 1.242x |
| startpos | 0.743x | 0.743x | 1.232x |
| chess960_materialize | 0.966x | 0.907x | 1.422x |
| chess960_perft_d3 | 0.893x | 0.842x | 1.169x |
| polyglot_key | 0.964x | 0.847x | 38.416x |
| polyglot_recompute | 0.881x | 0.841x | 0.999x |
| repetition_key | 0.950x | 0.807x | 18.116x |
| corpus_count | 0.967x | 0.957x | 1.091x |
| corpus_materialize | 0.832x | 0.750x | 0.973x |
| corpus_uci_sorted | 0.935x | 0.913x | 1.479x |
| replay_trusted_native | 0.895x | 0.714x | 0.747x |
| replay_checked_native | 0.937x | 0.917x | 1.233x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.95x).
