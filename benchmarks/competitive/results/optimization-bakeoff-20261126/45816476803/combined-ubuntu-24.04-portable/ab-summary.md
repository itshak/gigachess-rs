| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.207x |
| materialize | 1.167x | 1.121x | 1.621x |
| perft_bulk_d4 | 0.994x | 0.983x | 1.072x |
| perft_full_d3 | 1.000x | 0.999x | 1.451x |
| fen_parse | 0.984x | 0.947x | 1.151x |
| fen_write_native_ep | 0.992x | 0.978x | 1.834x |
| snapshot | 1.003x | 0.998x | 1.003x |
| san_parse_all | 1.078x | 1.052x | 1.870x |
| san_render_native | 0.988x | 0.970x | 2.109x |
| apply_children_full | 1.012x | 1.006x | 1.213x |
| make_unmake_full | 1.000x | 0.989x | 1.151x |
| startpos | 1.002x | 1.002x | 1.432x |
| chess960_materialize | 1.024x | 1.004x | 1.416x |
| chess960_perft_d3 | 0.999x | 0.998x | 1.392x |
| polyglot_key | 1.092x | 0.998x | 41.372x |
| polyglot_recompute | 0.988x | 0.981x | 0.947x |
| repetition_key | 1.035x | 1.002x | 17.231x |
| corpus_count | 0.999x | 0.999x | 1.035x |
| corpus_materialize | 0.999x | 0.998x | 1.085x |
| corpus_uci_sorted | 1.003x | 1.001x | 1.539x |
| replay_trusted_native | 1.011x | 0.995x | 0.824x |
| replay_checked_native | 0.986x | 0.978x | 1.122x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.10x).
