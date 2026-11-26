| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.994x | 1.201x |
| materialize | 1.048x | 1.031x | 1.607x |
| perft_bulk_d4 | 0.998x | 0.968x | 1.057x |
| perft_full_d3 | 0.893x | 0.830x | 1.303x |
| fen_parse | 0.941x | 0.889x | 1.090x |
| fen_write_native_ep | 1.001x | 0.962x | 1.903x |
| snapshot | 0.898x | 0.894x | 1.003x |
| san_parse_all | 1.006x | 0.978x | 1.889x |
| san_render_native | 1.186x | 1.118x | 2.115x |
| apply_children_full | 0.991x | 0.973x | 1.292x |
| make_unmake_full | 0.981x | 0.954x | 1.182x |
| startpos | 1.001x | 1.001x | 1.353x |
| chess960_materialize | 0.890x | 0.816x | 1.249x |
| chess960_perft_d3 | 0.965x | 0.939x | 1.334x |
| polyglot_key | 0.861x | 0.847x | 37.835x |
| polyglot_recompute | 1.021x | 1.020x | 1.085x |
| repetition_key | 1.071x | 0.840x | 20.944x |
| corpus_count | 1.027x | 0.996x | 1.044x |
| corpus_materialize | 1.011x | 1.008x | 1.114x |
| corpus_uci_sorted | 1.000x | 0.997x | 1.516x |
| replay_trusted_native | 0.949x | 0.949x | 0.818x |
| replay_checked_native | 1.001x | 0.987x | 1.059x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.94x).
