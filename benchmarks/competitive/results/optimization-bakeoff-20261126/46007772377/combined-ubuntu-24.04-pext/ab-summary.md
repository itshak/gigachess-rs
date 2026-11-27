| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.939x | 1.569x |
| materialize | 1.013x | 0.975x | 1.235x |
| perft_bulk_d4 | 1.014x | 0.966x | 1.406x |
| perft_full_d3 | 1.348x | 1.073x | 1.603x |
| fen_parse | 0.865x | 0.847x | 1.061x |
| fen_write_native_ep | 0.999x | 0.993x | 1.876x |
| snapshot | 1.001x | 0.999x | 1.249x |
| san_parse_all | 0.997x | 0.953x | 2.177x |
| san_render_native | 1.006x | 0.995x | 2.137x |
| apply_children_full | 0.972x | 0.912x | 1.283x |
| make_unmake_full | 1.016x | 0.991x | 1.226x |
| startpos | 1.002x | 1.002x | 1.202x |
| chess960_materialize | 1.024x | 1.007x | 1.536x |
| chess960_perft_d3 | 1.007x | 1.000x | 1.631x |
| polyglot_key | 1.004x | 1.000x | 20.779x |
| polyglot_recompute | 0.967x | 0.956x | 0.972x |
| repetition_key | 1.004x | 1.000x | 19.013x |
| corpus_count | 1.073x | 1.044x | 1.478x |
| corpus_materialize | 0.992x | 0.974x | 1.263x |
| corpus_uci_sorted | 0.998x | 0.995x | 1.673x |
| replay_trusted_native | 1.006x | 1.005x | 1.196x |
| replay_checked_native | 1.060x | 1.040x | 1.124x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.90x).
