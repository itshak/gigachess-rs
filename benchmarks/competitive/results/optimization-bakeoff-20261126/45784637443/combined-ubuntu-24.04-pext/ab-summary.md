| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.027x | 0.984x | 1.474x |
| materialize | 1.001x | 0.980x | 1.179x |
| perft_bulk_d4 | 1.037x | 0.974x | 1.362x |
| perft_full_d3 | 1.003x | 0.964x | 0.971x |
| fen_parse | 1.080x | 1.049x | 1.189x |
| fen_write_native_ep | 0.996x | 0.991x | 1.856x |
| snapshot | 1.225x | 1.221x | 1.253x |
| san_parse_all | 1.002x | 0.992x | 2.166x |
| san_render_native | 1.005x | 0.976x | 2.180x |
| apply_children_full | 1.022x | 0.989x | 1.152x |
| make_unmake_full | 0.998x | 0.986x | 1.193x |
| startpos | 0.913x | 0.913x | 1.002x |
| chess960_materialize | 1.046x | 0.986x | 1.573x |
| chess960_perft_d3 | 1.037x | 1.001x | 1.534x |
| polyglot_key | 0.993x | 0.990x | 20.609x |
| polyglot_recompute | 1.005x | 0.992x | 1.028x |
| repetition_key | 0.948x | 0.884x | 18.151x |
| corpus_count | 1.013x | 1.004x | 1.469x |
| corpus_materialize | 0.992x | 0.977x | 1.295x |
| corpus_uci_sorted | 1.001x | 0.987x | 1.706x |
| replay_trusted_native | 1.013x | 0.990x | 0.750x |
| replay_checked_native | 1.003x | 1.003x | 0.940x |

Cold first legal move list: 0.012 ms -> 0.010 ms (1.22x).
