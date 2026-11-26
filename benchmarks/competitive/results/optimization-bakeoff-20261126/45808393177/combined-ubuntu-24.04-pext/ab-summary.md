| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.960x | 0.921x | 1.412x |
| materialize | 1.010x | 0.968x | 1.471x |
| perft_bulk_d4 | 0.836x | 0.720x | 1.049x |
| perft_full_d3 | 0.951x | 0.916x | 1.466x |
| fen_parse | 0.942x | 0.859x | 1.048x |
| fen_write_native_ep | 1.000x | 0.986x | 1.902x |
| snapshot | 1.415x | 1.406x | 1.331x |
| san_parse_all | 0.972x | 0.959x | 1.868x |
| san_render_native | 1.007x | 1.000x | 2.145x |
| apply_children_full | 1.082x | 1.022x | 1.194x |
| make_unmake_full | 1.021x | 0.955x | 1.139x |
| startpos | 1.226x | 1.226x | 1.251x |
| chess960_materialize | 0.969x | 0.948x | 1.356x |
| chess960_perft_d3 | 0.979x | 0.974x | 1.353x |
| polyglot_key | 1.086x | 1.000x | 39.982x |
| polyglot_recompute | 1.016x | 1.011x | 1.027x |
| repetition_key | 1.017x | 0.843x | 19.022x |
| corpus_count | 0.686x | 0.626x | 0.964x |
| corpus_materialize | 0.951x | 0.939x | 1.063x |
| corpus_uci_sorted | 0.979x | 0.975x | 1.542x |
| replay_trusted_native | 0.957x | 0.938x | 0.800x |
| replay_checked_native | 0.873x | 0.813x | 0.945x |

Cold first legal move list: 0.008 ms -> 0.005 ms (1.51x).
