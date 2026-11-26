| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.834x | 0.748x | 1.549x |
| materialize | 0.814x | 0.655x | 1.018x |
| perft_bulk_d4 | 0.886x | 0.741x | 1.262x |
| perft_full_d3 | 0.806x | 0.673x | 1.070x |
| fen_parse | 0.863x | 0.654x | 1.096x |
| fen_write_native_ep | 1.028x | 0.832x | 1.978x |
| snapshot | 0.959x | 0.633x | 0.843x |
| san_parse_all | 0.905x | 0.769x | 1.750x |
| san_render_native | 0.827x | 0.695x | 2.712x |
| apply_children_full | 0.829x | 0.682x | 1.024x |
| make_unmake_full | 0.829x | 0.758x | 1.273x |
| startpos | 0.714x | 0.714x | 1.310x |
| chess960_materialize | 0.773x | 0.767x | 1.292x |
| chess960_perft_d3 | 0.815x | 0.785x | 1.639x |
| polyglot_key | 0.876x | 0.863x | 36.367x |
| polyglot_recompute | 0.881x | 0.818x | 0.908x |
| repetition_key | 0.945x | 0.870x | 18.423x |
| corpus_count | 0.780x | 0.702x | 1.100x |
| corpus_materialize | 1.000x | 0.895x | 1.163x |
| corpus_uci_sorted | 0.982x | 0.834x | 1.530x |
| replay_trusted_native | 0.920x | 0.899x | 0.594x |
| replay_checked_native | 0.794x | 0.789x | 1.071x |

Cold first legal move list: 0.008 ms -> 0.005 ms (1.43x).
