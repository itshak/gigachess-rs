| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.035x | 0.829x | 1.320x |
| materialize | 0.892x | 0.704x | 0.896x |
| perft_bulk_d4 | 1.021x | 0.785x | 1.329x |
| perft_full_d3 | 1.742x | 1.525x | 1.335x |
| fen_parse | 1.028x | 0.809x | 1.276x |
| fen_write_native_ep | 0.968x | 0.698x | 1.393x |
| snapshot | 0.934x | 0.515x | 0.891x |
| san_parse_all | 1.151x | 1.026x | 1.916x |
| san_render_native | 1.090x | 0.838x | 2.762x |
| apply_children_full | 1.296x | 1.044x | 0.980x |
| make_unmake_full | 1.196x | 1.080x | 1.248x |
| startpos | 1.082x | 1.082x | 1.260x |
| chess960_materialize | 0.920x | 0.799x | 1.258x |
| chess960_perft_d3 | 1.212x | 1.165x | 1.466x |
| polyglot_key | 1.045x | 1.014x | 41.612x |
| polyglot_recompute | 0.917x | 0.854x | 0.974x |
| repetition_key | 1.041x | 0.962x | 20.062x |
| corpus_count | 0.907x | 0.891x | 1.553x |
| corpus_materialize | 0.977x | 0.862x | 1.027x |
| corpus_uci_sorted | 1.326x | 1.163x | 1.307x |
| replay_trusted_native | 1.499x | 1.309x | 0.603x |
| replay_checked_native | 1.070x | 0.980x | 1.065x |

Cold first legal move list: 0.011 ms -> 0.015 ms (0.72x).
