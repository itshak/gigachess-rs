| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.120x | 1.029x | 1.626x |
| materialize | 1.035x | 0.884x | 1.263x |
| perft_bulk_d4 | 1.114x | 1.052x | 1.313x |
| perft_full_d3 | 1.076x | 0.684x | 1.496x |
| fen_parse | 0.999x | 0.969x | 1.380x |
| fen_write_native_ep | 0.991x | 0.957x | 1.695x |
| snapshot | 0.960x | 0.935x | 1.149x |
| san_parse_all | 1.038x | 0.989x | 1.906x |
| san_render_native | 1.030x | 0.991x | 4.337x |
| apply_children_full | 1.142x | 1.045x | 1.331x |
| make_unmake_full | 0.993x | 0.937x | 1.376x |
| startpos | 1.192x | 1.192x | 1.194x |
| chess960_materialize | 1.042x | 1.011x | 1.350x |
| chess960_perft_d3 | 1.096x | 1.068x | 1.489x |
| polyglot_key | 1.064x | 1.000x | 37.928x |
| polyglot_recompute | 1.053x | 0.920x | 1.100x |
| repetition_key | 1.001x | 0.976x | 21.881x |
| corpus_count | 1.300x | 1.186x | 1.348x |
| corpus_materialize | 1.038x | 0.967x | 1.087x |
| corpus_uci_sorted | 0.978x | 0.972x | 1.415x |
| replay_trusted_native | 1.029x | 1.024x | 0.796x |
| replay_checked_native | 1.017x | 1.009x | 1.086x |

Cold first legal move list: 0.020 ms -> 0.020 ms (0.98x).
