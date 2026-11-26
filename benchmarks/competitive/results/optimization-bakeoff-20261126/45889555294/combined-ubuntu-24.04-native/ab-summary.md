| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.939x | 0.885x | 1.359x |
| materialize | 0.969x | 0.933x | 1.493x |
| perft_bulk_d4 | 0.926x | 0.888x | 1.080x |
| perft_full_d3 | 0.906x | 0.892x | 1.249x |
| fen_parse | 0.925x | 0.889x | 1.018x |
| fen_write_native_ep | 0.921x | 0.843x | 1.951x |
| snapshot | 0.750x | 0.741x | 1.225x |
| san_parse_all | 0.942x | 0.910x | 1.680x |
| san_render_native | 0.969x | 0.906x | 2.092x |
| apply_children_full | 0.945x | 0.919x | 1.290x |
| make_unmake_full | 0.960x | 0.921x | 1.168x |
| startpos | 0.996x | 0.996x | 1.574x |
| chess960_materialize | 0.963x | 0.882x | 1.377x |
| chess960_perft_d3 | 0.923x | 0.906x | 1.374x |
| polyglot_key | 1.058x | 1.007x | 40.950x |
| polyglot_recompute | 1.010x | 0.988x | 1.020x |
| repetition_key | 1.006x | 1.002x | 25.228x |
| corpus_count | 0.979x | 0.960x | 0.925x |
| corpus_materialize | 1.011x | 1.009x | 1.052x |
| corpus_uci_sorted | 1.031x | 1.025x | 1.538x |
| replay_trusted_native | 1.002x | 0.963x | 0.847x |
| replay_checked_native | 0.896x | 0.878x | 1.029x |

Cold first legal move list: 0.005 ms -> 0.008 ms (0.70x).
