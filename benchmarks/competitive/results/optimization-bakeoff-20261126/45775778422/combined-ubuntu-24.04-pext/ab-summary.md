| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.992x | 1.436x |
| materialize | 1.004x | 0.996x | 1.554x |
| perft_bulk_d4 | 0.998x | 0.990x | 1.237x |
| perft_full_d3 | 1.024x | 1.017x | 0.875x |
| fen_parse | 1.031x | 1.009x | 1.067x |
| fen_write_native_ep | 0.988x | 0.948x | 1.252x |
| snapshot | 1.284x | 1.280x | 1.146x |
| san_parse_all | 0.940x | 0.917x | 1.690x |
| san_render_native | 1.084x | 0.980x | 2.115x |
| apply_children_full | 1.017x | 0.988x | 0.984x |
| make_unmake_full | 1.040x | 0.995x | 0.977x |
| startpos | 0.910x | 0.910x | 1.072x |
| chess960_materialize | 0.942x | 0.935x | 1.308x |
| chess960_perft_d3 | 1.003x | 1.002x | 1.317x |
| polyglot_key | 0.991x | 0.984x | 40.755x |
| polyglot_recompute | 0.967x | 0.961x | 1.012x |
| repetition_key | 0.995x | 0.985x | 20.696x |
| corpus_count | 1.000x | 0.999x | 1.382x |
| corpus_materialize | 1.001x | 0.998x | 1.127x |
| corpus_uci_sorted | 0.998x | 0.993x | 0.960x |
| replay_trusted_native | 0.996x | 0.994x | 0.604x |
| replay_checked_native | 0.983x | 0.966x | 0.955x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.03x).
