| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.987x | 0.932x | 1.485x |
| materialize | 0.958x | 0.905x | 1.238x |
| perft_bulk_d4 | 0.965x | 0.823x | 1.362x |
| perft_full_d3 | 0.974x | 0.901x | 1.344x |
| fen_parse | 1.017x | 0.921x | 1.319x |
| fen_write_native_ep | 1.023x | 0.915x | 1.577x |
| snapshot | 1.027x | 0.986x | 1.006x |
| san_parse_all | 0.999x | 0.953x | 2.120x |
| san_render_native | 1.004x | 0.951x | 3.757x |
| apply_children_full | 1.050x | 0.945x | 1.401x |
| make_unmake_full | 0.959x | 0.893x | 1.334x |
| startpos | 1.003x | 1.003x | 1.252x |
| chess960_materialize | 0.989x | 0.962x | 1.423x |
| chess960_perft_d3 | 0.890x | 0.889x | 1.480x |
| polyglot_key | 0.935x | 0.919x | 30.522x |
| polyglot_recompute | 0.876x | 0.834x | 0.955x |
| repetition_key | 0.996x | 0.978x | 23.930x |
| corpus_count | 0.963x | 0.923x | 1.390x |
| corpus_materialize | 0.952x | 0.945x | 1.136x |
| corpus_uci_sorted | 1.000x | 0.988x | 1.491x |
| replay_trusted_native | 1.024x | 0.945x | 0.860x |
| replay_checked_native | 0.979x | 0.973x | 1.067x |

Cold first legal move list: 0.018 ms -> 0.018 ms (0.97x).
