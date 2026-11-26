| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.996x | 1.541x |
| materialize | 1.031x | 0.992x | 1.143x |
| perft_bulk_d4 | 1.045x | 1.007x | 1.422x |
| perft_full_d3 | 1.001x | 0.987x | 1.287x |
| fen_parse | 1.064x | 1.020x | 1.447x |
| fen_write_native_ep | 1.024x | 1.009x | 1.592x |
| snapshot | 0.972x | 0.866x | 1.085x |
| san_parse_all | 0.999x | 0.984x | 1.990x |
| san_render_native | 0.996x | 0.982x | 3.303x |
| apply_children_full | 1.008x | 0.990x | 1.352x |
| make_unmake_full | 0.993x | 0.971x | 1.299x |
| startpos | 1.003x | 1.003x | 1.006x |
| chess960_materialize | 0.998x | 0.994x | 1.344x |
| chess960_perft_d3 | 1.000x | 0.997x | 1.586x |
| polyglot_key | 0.990x | 0.978x | 19.757x |
| polyglot_recompute | 0.807x | 0.804x | 0.819x |
| repetition_key | 0.998x | 0.993x | 19.027x |
| corpus_count | 0.995x | 0.994x | 1.307x |
| corpus_materialize | 0.988x | 0.976x | 1.178x |
| corpus_uci_sorted | 0.989x | 0.982x | 1.496x |
| replay_trusted_native | 0.979x | 0.961x | 1.072x |
| replay_checked_native | 1.004x | 1.002x | 1.105x |

Cold first legal move list: 0.016 ms -> 0.017 ms (0.94x).
