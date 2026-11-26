| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.986x | 0.930x | 1.144x |
| materialize | 0.895x | 0.816x | 1.139x |
| perft_bulk_d4 | 0.949x | 0.920x | 1.093x |
| perft_full_d3 | 0.966x | 0.940x | 1.113x |
| fen_parse | 1.152x | 1.122x | 1.206x |
| fen_write_native_ep | 0.923x | 0.865x | 1.635x |
| snapshot | 1.075x | 1.058x | 0.990x |
| san_parse_all | 0.972x | 0.964x | 2.095x |
| san_render_native | 0.918x | 0.889x | 2.138x |
| apply_children_full | 1.156x | 1.107x | 1.437x |
| make_unmake_full | 0.949x | 0.890x | 1.308x |
| startpos | 1.217x | 1.217x | 1.598x |
| chess960_materialize | 0.896x | 0.886x | 1.411x |
| chess960_perft_d3 | 0.944x | 0.936x | 1.391x |
| polyglot_key | 1.031x | 1.001x | 22.273x |
| polyglot_recompute | 0.990x | 0.980x | 0.951x |
| repetition_key | 1.003x | 0.949x | 20.389x |
| corpus_count | 1.033x | 1.032x | 1.107x |
| corpus_materialize | 1.004x | 0.994x | 1.239x |
| corpus_uci_sorted | 1.055x | 1.051x | 1.654x |
| replay_trusted_native | 1.054x | 1.036x | 1.044x |
| replay_checked_native | 0.982x | 0.969x | 1.035x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.03x).
