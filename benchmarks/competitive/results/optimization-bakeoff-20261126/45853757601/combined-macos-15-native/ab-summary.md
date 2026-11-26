| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.055x | 0.858x | 1.461x |
| materialize | 1.076x | 0.612x | 0.997x |
| perft_bulk_d4 | 1.156x | 0.992x | 1.222x |
| perft_full_d3 | 1.199x | 1.114x | 1.289x |
| fen_parse | 1.328x | 1.228x | 1.292x |
| fen_write_native_ep | 1.207x | 0.954x | 1.793x |
| snapshot | 1.151x | 1.024x | 1.005x |
| san_parse_all | 1.140x | 0.962x | 1.764x |
| san_render_native | 1.034x | 0.863x | 2.655x |
| apply_children_full | 1.091x | 0.819x | 1.109x |
| make_unmake_full | 1.126x | 0.994x | 1.288x |
| startpos | 1.590x | 1.590x | 1.197x |
| chess960_materialize | 1.068x | 0.939x | 1.590x |
| chess960_perft_d3 | 0.969x | 0.936x | 1.635x |
| polyglot_key | 0.854x | 0.791x | 34.344x |
| polyglot_recompute | 0.891x | 0.768x | 0.960x |
| repetition_key | 1.050x | 0.834x | 21.189x |
| corpus_count | 1.002x | 0.841x | 1.258x |
| corpus_materialize | 0.914x | 0.881x | 1.137x |
| corpus_uci_sorted | 0.893x | 0.835x | 1.341x |
| replay_trusted_native | 0.975x | 0.865x | 0.720x |
| replay_checked_native | 0.961x | 0.902x | 1.097x |

Cold first legal move list: 0.004 ms -> 0.005 ms (0.86x).
