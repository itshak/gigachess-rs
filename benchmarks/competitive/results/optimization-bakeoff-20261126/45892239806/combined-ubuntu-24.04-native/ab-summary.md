| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.915x | 0.872x | 1.361x |
| materialize | 0.919x | 0.885x | 1.137x |
| perft_bulk_d4 | 0.934x | 0.857x | 1.227x |
| perft_full_d3 | 0.864x | 0.837x | 1.024x |
| fen_parse | 0.938x | 0.883x | 1.221x |
| fen_write_native_ep | 0.888x | 0.834x | 1.658x |
| snapshot | 0.894x | 0.889x | 1.221x |
| san_parse_all | 0.926x | 0.913x | 2.030x |
| san_render_native | 0.963x | 0.897x | 2.024x |
| apply_children_full | 0.959x | 0.913x | 1.317x |
| make_unmake_full | 0.958x | 0.906x | 1.155x |
| startpos | 1.157x | 1.157x | 1.500x |
| chess960_materialize | 0.893x | 0.888x | 1.440x |
| chess960_perft_d3 | 0.915x | 0.900x | 1.462x |
| polyglot_key | 1.067x | 0.998x | 21.858x |
| polyglot_recompute | 0.960x | 0.959x | 0.984x |
| repetition_key | 0.967x | 0.901x | 18.418x |
| corpus_count | 0.955x | 0.938x | 1.210x |
| corpus_materialize | 0.966x | 0.953x | 1.181x |
| corpus_uci_sorted | 0.950x | 0.944x | 1.610x |
| replay_trusted_native | 1.079x | 0.989x | 1.191x |
| replay_checked_native | 0.882x | 0.875x | 0.940x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.93x).
