| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.080x | 0.980x | 1.324x |
| materialize | 1.102x | 1.057x | 1.083x |
| perft_bulk_d4 | 1.107x | 0.963x | 1.036x |
| perft_full_d3 | 1.119x | 0.971x | 1.261x |
| fen_parse | 1.172x | 1.071x | 1.245x |
| fen_write_native_ep | 1.134x | 0.979x | 1.748x |
| snapshot | 1.211x | 1.028x | 1.015x |
| san_parse_all | 1.068x | 0.866x | 1.648x |
| san_render_native | 1.131x | 0.933x | 2.735x |
| apply_children_full | 1.057x | 0.942x | 1.003x |
| make_unmake_full | 1.052x | 0.967x | 1.172x |
| startpos | 1.016x | 1.016x | 1.201x |
| chess960_materialize | 0.912x | 0.810x | 1.422x |
| chess960_perft_d3 | 0.927x | 0.823x | 1.236x |
| polyglot_key | 1.005x | 1.000x | 40.974x |
| polyglot_recompute | 0.943x | 0.921x | 0.989x |
| repetition_key | 0.947x | 0.900x | 19.171x |
| corpus_count | 0.938x | 0.841x | 0.974x |
| corpus_materialize | 1.281x | 1.237x | 1.062x |
| corpus_uci_sorted | 1.077x | 1.068x | 1.406x |
| replay_trusted_native | 0.979x | 0.946x | 0.581x |
| replay_checked_native | 1.186x | 1.175x | 1.037x |

Cold first legal move list: 0.004 ms -> 0.008 ms (0.46x).
