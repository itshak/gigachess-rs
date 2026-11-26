| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.979x | 0.942x | 1.457x |
| materialize | 0.976x | 0.934x | 1.203x |
| perft_bulk_d4 | 0.995x | 0.948x | 1.329x |
| perft_full_d3 | 0.988x | 0.931x | 1.325x |
| fen_parse | 1.006x | 0.946x | 1.275x |
| fen_write_native_ep | 1.011x | 1.005x | 1.564x |
| snapshot | 1.021x | 0.994x | 1.001x |
| san_parse_all | 1.011x | 1.003x | 2.090x |
| san_render_native | 1.017x | 0.995x | 3.918x |
| apply_children_full | 1.004x | 0.999x | 1.412x |
| make_unmake_full | 0.990x | 0.966x | 1.350x |
| startpos | 1.003x | 1.003x | 1.246x |
| chess960_materialize | 1.049x | 1.030x | 1.362x |
| chess960_perft_d3 | 0.975x | 0.953x | 1.517x |
| polyglot_key | 1.023x | 1.009x | 25.786x |
| polyglot_recompute | 1.003x | 1.003x | 1.019x |
| repetition_key | 1.014x | 0.999x | 19.981x |
| corpus_count | 0.958x | 0.951x | 1.317x |
| corpus_materialize | 1.041x | 0.996x | 1.140x |
| corpus_uci_sorted | 0.993x | 0.983x | 1.413x |
| replay_trusted_native | 1.004x | 0.993x | 0.849x |
| replay_checked_native | 1.008x | 1.001x | 1.100x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.04x).
