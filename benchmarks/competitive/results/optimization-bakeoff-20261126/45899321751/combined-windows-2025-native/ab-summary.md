| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.961x | 0.949x | 1.478x |
| materialize | 0.992x | 0.977x | 1.147x |
| perft_bulk_d4 | 1.016x | 0.968x | 1.439x |
| perft_full_d3 | 0.997x | 0.982x | 1.247x |
| fen_parse | 0.945x | 0.924x | 1.295x |
| fen_write_native_ep | 1.005x | 0.971x | 1.619x |
| snapshot | 0.998x | 0.986x | 1.158x |
| san_parse_all | 0.976x | 0.962x | 1.954x |
| san_render_native | 1.017x | 0.991x | 3.446x |
| apply_children_full | 1.005x | 0.988x | 1.311x |
| make_unmake_full | 0.994x | 0.975x | 1.301x |
| startpos | 0.999x | 0.999x | 1.000x |
| chess960_materialize | 1.003x | 0.998x | 1.354x |
| chess960_perft_d3 | 0.989x | 0.972x | 1.661x |
| polyglot_key | 1.006x | 1.002x | 20.441x |
| polyglot_recompute | 0.988x | 0.988x | 0.962x |
| repetition_key | 0.982x | 0.970x | 18.856x |
| corpus_count | 1.039x | 0.983x | 1.347x |
| corpus_materialize | 1.007x | 0.999x | 1.194x |
| corpus_uci_sorted | 1.004x | 0.997x | 1.502x |
| replay_trusted_native | 0.986x | 0.984x | 1.071x |
| replay_checked_native | 1.011x | 1.002x | 1.093x |

Cold first legal move list: 0.013 ms -> 0.018 ms (0.76x).
