| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.980x | 1.303x |
| materialize | 1.055x | 1.028x | 1.125x |
| perft_bulk_d4 | 1.005x | 0.951x | 1.195x |
| perft_full_d3 | 1.237x | 1.180x | 1.190x |
| fen_parse | 1.189x | 1.142x | 1.230x |
| fen_write_native_ep | 1.000x | 0.981x | 1.512x |
| snapshot | 1.549x | 1.525x | 1.494x |
| san_parse_all | 0.993x | 0.984x | 1.741x |
| san_render_native | 1.060x | 0.982x | 2.335x |
| apply_children_full | 1.221x | 1.192x | 1.142x |
| make_unmake_full | 0.997x | 0.958x | 1.244x |
| startpos | 1.007x | 1.007x | 1.244x |
| chess960_materialize | 0.999x | 0.990x | 1.351x |
| chess960_perft_d3 | 0.722x | 0.539x | 0.986x |
| polyglot_key | 1.035x | 1.019x | 37.806x |
| polyglot_recompute | 0.970x | 0.964x | 0.988x |
| repetition_key | 0.861x | 0.848x | 23.004x |
| corpus_count | 1.007x | 0.997x | 1.213x |
| corpus_materialize | 1.045x | 0.982x | 1.192x |
| corpus_uci_sorted | 1.000x | 0.998x | 1.671x |
| replay_trusted_native | 0.995x | 0.959x | 0.617x |
| replay_checked_native | 1.013x | 1.011x | 0.902x |

Cold first legal move list: 0.014 ms -> 0.015 ms (0.98x).
