| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.992x | 0.977x | 1.388x |
| materialize | 0.999x | 0.990x | 1.499x |
| perft_bulk_d4 | 0.979x | 0.960x | 1.178x |
| perft_full_d3 | 0.902x | 0.874x | 1.343x |
| fen_parse | 0.882x | 0.857x | 0.931x |
| fen_write_native_ep | 0.990x | 0.987x | 1.921x |
| snapshot | 0.968x | 0.966x | 1.501x |
| san_parse_all | 0.963x | 0.921x | 1.864x |
| san_render_native | 0.806x | 0.695x | 1.692x |
| apply_children_full | 0.965x | 0.925x | 0.943x |
| make_unmake_full | 0.911x | 0.862x | 1.025x |
| startpos | 0.907x | 0.907x | 1.328x |
| chess960_materialize | 0.945x | 0.888x | 1.373x |
| chess960_perft_d3 | 0.945x | 0.920x | 1.314x |
| polyglot_key | 0.984x | 0.974x | 40.775x |
| polyglot_recompute | 0.998x | 0.990x | 1.066x |
| repetition_key | 0.924x | 0.736x | 20.623x |
| corpus_count | 1.005x | 0.985x | 1.331x |
| corpus_materialize | 1.005x | 1.002x | 1.097x |
| corpus_uci_sorted | 1.016x | 1.014x | 1.571x |
| replay_trusted_native | 0.781x | 0.696x | 0.566x |
| replay_checked_native | 0.725x | 0.619x | 0.823x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.85x).
