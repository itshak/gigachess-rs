| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.992x | 0.967x | 1.458x |
| materialize | 1.001x | 0.994x | 1.539x |
| perft_bulk_d4 | 0.998x | 0.981x | 1.261x |
| perft_full_d3 | 1.011x | 0.981x | 1.467x |
| fen_parse | 1.079x | 1.016x | 1.123x |
| fen_write_native_ep | 0.963x | 0.931x | 1.172x |
| snapshot | 1.290x | 1.276x | 0.995x |
| san_parse_all | 1.010x | 0.994x | 1.905x |
| san_render_native | 1.001x | 0.995x | 2.160x |
| apply_children_full | 1.046x | 1.014x | 1.109x |
| make_unmake_full | 1.009x | 0.990x | 1.126x |
| startpos | 1.096x | 1.096x | 1.014x |
| chess960_materialize | 0.940x | 0.934x | 1.343x |
| chess960_perft_d3 | 1.002x | 0.996x | 1.393x |
| polyglot_key | 0.968x | 0.936x | 38.096x |
| polyglot_recompute | 1.010x | 1.009x | 1.045x |
| repetition_key | 0.725x | 0.718x | 16.469x |
| corpus_count | 0.993x | 0.990x | 1.396x |
| corpus_materialize | 0.996x | 0.996x | 1.113x |
| corpus_uci_sorted | 0.992x | 0.988x | 1.571x |
| replay_trusted_native | 1.008x | 0.992x | 0.738x |
| replay_checked_native | 1.004x | 0.995x | 1.028x |

Cold first legal move list: 0.008 ms -> 0.010 ms (0.77x).
