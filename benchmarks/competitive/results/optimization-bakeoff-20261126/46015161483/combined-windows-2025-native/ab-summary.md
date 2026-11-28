| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.092x | 1.058x | 1.549x |
| materialize | 1.034x | 0.976x | 1.295x |
| perft_bulk_d4 | 1.108x | 0.988x | 1.413x |
| perft_full_d3 | 1.290x | 1.051x | 1.633x |
| fen_parse | 1.011x | 0.867x | 1.312x |
| fen_write_native_ep | 1.013x | 0.843x | 1.519x |
| snapshot | 1.004x | 0.984x | 1.000x |
| san_parse_all | 1.021x | 0.951x | 2.228x |
| san_render_native | 1.033x | 0.965x | 3.873x |
| apply_children_full | 1.060x | 1.004x | 1.439x |
| make_unmake_full | 1.033x | 0.990x | 1.406x |
| startpos | 1.005x | 1.005x | 1.250x |
| chess960_materialize | 1.005x | 0.990x | 1.376x |
| chess960_perft_d3 | 1.038x | 1.021x | 1.529x |
| polyglot_key | 1.112x | 1.033x | 27.424x |
| polyglot_recompute | 1.224x | 1.133x | 1.244x |
| repetition_key | 0.867x | 0.768x | 19.356x |
| corpus_count | 1.091x | 1.083x | 1.496x |
| corpus_materialize | 0.998x | 0.997x | 1.170x |
| corpus_uci_sorted | 1.011x | 1.009x | 1.409x |
| replay_trusted_native | 1.034x | 1.023x | 0.873x |
| replay_checked_native | 0.993x | 0.991x | 1.096x |

Cold first legal move list: 0.017 ms -> 0.016 ms (1.01x).
