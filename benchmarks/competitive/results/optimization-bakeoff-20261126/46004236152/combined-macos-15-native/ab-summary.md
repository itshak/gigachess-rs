| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.974x | 0.840x | 1.623x |
| materialize | 0.979x | 0.891x | 1.176x |
| perft_bulk_d4 | 1.080x | 0.981x | 1.318x |
| perft_full_d3 | 1.106x | 0.943x | 1.477x |
| fen_parse | 0.959x | 0.876x | 1.202x |
| fen_write_native_ep | 0.989x | 0.887x | 1.861x |
| snapshot | 0.946x | 0.849x | 0.984x |
| san_parse_all | 0.991x | 0.934x | 1.784x |
| san_render_native | 0.933x | 0.870x | 2.729x |
| apply_children_full | 1.002x | 0.910x | 1.182x |
| make_unmake_full | 0.961x | 0.876x | 1.372x |
| startpos | 0.739x | 0.739x | 1.080x |
| chess960_materialize | 0.858x | 0.836x | 1.566x |
| chess960_perft_d3 | 0.956x | 0.842x | 1.610x |
| polyglot_key | 1.016x | 1.008x | 38.817x |
| polyglot_recompute | 0.952x | 0.940x | 0.983x |
| repetition_key | 1.000x | 0.936x | 19.256x |
| corpus_count | 1.130x | 1.101x | 1.415x |
| corpus_materialize | 1.069x | 1.023x | 1.195x |
| corpus_uci_sorted | 0.936x | 0.833x | 1.353x |
| replay_trusted_native | 1.115x | 1.017x | 0.733x |
| replay_checked_native | 1.003x | 0.998x | 1.254x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.96x).
