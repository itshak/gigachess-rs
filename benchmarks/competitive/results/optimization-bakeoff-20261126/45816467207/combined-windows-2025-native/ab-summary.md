| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.959x | 1.324x |
| materialize | 1.022x | 0.981x | 1.183x |
| perft_bulk_d4 | 0.967x | 0.932x | 1.157x |
| perft_full_d3 | 0.989x | 0.890x | 1.235x |
| fen_parse | 0.876x | 0.842x | 1.126x |
| fen_write_native_ep | 0.989x | 0.981x | 1.649x |
| snapshot | 0.959x | 0.827x | 0.969x |
| san_parse_all | 1.003x | 0.964x | 1.984x |
| san_render_native | 1.024x | 0.897x | 3.658x |
| apply_children_full | 1.017x | 0.973x | 1.263x |
| make_unmake_full | 0.948x | 0.840x | 1.206x |
| startpos | 0.996x | 0.996x | 1.246x |
| chess960_materialize | 1.030x | 0.990x | 1.304x |
| chess960_perft_d3 | 0.998x | 0.920x | 1.363x |
| polyglot_key | 0.979x | 0.938x | 25.274x |
| polyglot_recompute | 0.963x | 0.943x | 1.070x |
| repetition_key | 0.944x | 0.854x | 18.313x |
| corpus_count | 1.069x | 1.034x | 1.264x |
| corpus_materialize | 0.983x | 0.971x | 1.052x |
| corpus_uci_sorted | 1.009x | 0.999x | 1.458x |
| replay_trusted_native | 1.014x | 1.008x | 0.722x |
| replay_checked_native | 0.928x | 0.918x | 0.953x |

Cold first legal move list: 0.022 ms -> 0.022 ms (1.00x).
