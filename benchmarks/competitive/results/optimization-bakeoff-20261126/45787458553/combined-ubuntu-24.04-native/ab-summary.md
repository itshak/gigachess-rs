| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.996x | 1.417x |
| materialize | 1.020x | 1.006x | 1.479x |
| perft_bulk_d4 | 0.980x | 0.957x | 1.180x |
| perft_full_d3 | 0.995x | 0.960x | 1.526x |
| fen_parse | 0.991x | 0.976x | 1.107x |
| fen_write_native_ep | 1.015x | 1.001x | 2.083x |
| snapshot | 1.394x | 1.380x | 1.127x |
| san_parse_all | 1.114x | 1.055x | 1.865x |
| san_render_native | 0.961x | 0.921x | 2.081x |
| apply_children_full | 0.928x | 0.905x | 1.323x |
| make_unmake_full | 0.930x | 0.898x | 1.060x |
| startpos | 0.806x | 0.806x | 0.839x |
| chess960_materialize | 1.012x | 1.007x | 1.384x |
| chess960_perft_d3 | 0.994x | 0.981x | 1.350x |
| polyglot_key | 0.992x | 0.985x | 39.407x |
| polyglot_recompute | 0.970x | 0.966x | 0.956x |
| repetition_key | 0.984x | 0.969x | 23.609x |
| corpus_count | 1.006x | 1.003x | 1.317x |
| corpus_materialize | 1.000x | 0.993x | 1.104x |
| corpus_uci_sorted | 0.994x | 0.989x | 1.583x |
| replay_trusted_native | 0.987x | 0.964x | 0.698x |
| replay_checked_native | 0.968x | 0.963x | 1.017x |

Cold first legal move list: 0.012 ms -> 0.013 ms (0.89x).
