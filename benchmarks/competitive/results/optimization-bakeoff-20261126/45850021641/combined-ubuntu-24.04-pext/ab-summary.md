| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 1.000x | 1.508x |
| materialize | 1.007x | 0.985x | 1.270x |
| perft_bulk_d4 | 0.998x | 0.978x | 1.413x |
| perft_full_d3 | 1.000x | 0.993x | 1.327x |
| fen_parse | 0.975x | 0.953x | 1.163x |
| fen_write_native_ep | 0.973x | 0.914x | 1.693x |
| snapshot | 0.996x | 0.986x | 1.245x |
| san_parse_all | 1.040x | 1.030x | 2.078x |
| san_render_native | 1.004x | 0.998x | 2.231x |
| apply_children_full | 1.027x | 1.006x | 1.331x |
| make_unmake_full | 1.027x | 0.992x | 1.306x |
| startpos | 1.000x | 1.000x | 1.258x |
| chess960_materialize | 1.011x | 0.995x | 1.415x |
| chess960_perft_d3 | 0.997x | 0.992x | 1.495x |
| polyglot_key | 0.905x | 0.853x | 25.121x |
| polyglot_recompute | 1.004x | 1.000x | 1.041x |
| repetition_key | 0.972x | 0.930x | 19.500x |
| corpus_count | 0.993x | 0.987x | 1.421x |
| corpus_materialize | 0.975x | 0.969x | 1.168x |
| corpus_uci_sorted | 0.991x | 0.981x | 1.751x |
| replay_trusted_native | 0.993x | 0.983x | 0.857x |
| replay_checked_native | 1.018x | 1.015x | 1.023x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.90x).
