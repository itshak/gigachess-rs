| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.077x | 0.595x | 1.278x |
| materialize | 1.024x | 0.772x | 1.035x |
| perft_bulk_d4 | 1.063x | 0.896x | 1.229x |
| perft_full_d3 | 0.975x | 0.796x | 1.306x |
| fen_parse | 1.096x | 0.864x | 1.202x |
| fen_write_native_ep | 1.102x | 0.909x | 1.741x |
| snapshot | 1.099x | 0.661x | 0.855x |
| san_parse_all | 0.981x | 0.822x | 1.921x |
| san_render_native | 1.005x | 0.668x | 2.816x |
| apply_children_full | 0.945x | 0.803x | 0.888x |
| make_unmake_full | 1.080x | 0.877x | 1.157x |
| startpos | 1.538x | 1.538x | 1.174x |
| chess960_materialize | 0.907x | 0.803x | 1.488x |
| chess960_perft_d3 | 0.826x | 0.814x | 1.255x |
| polyglot_key | 1.033x | 1.004x | 42.787x |
| polyglot_recompute | 0.947x | 0.934x | 1.084x |
| repetition_key | 0.971x | 0.903x | 20.321x |
| corpus_count | 0.678x | 0.626x | 1.436x |
| corpus_materialize | 0.767x | 0.642x | 1.155x |
| corpus_uci_sorted | 0.867x | 0.760x | 1.424x |
| replay_trusted_native | 0.902x | 0.780x | 0.453x |
| replay_checked_native | 0.884x | 0.836x | 0.981x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.06x).
