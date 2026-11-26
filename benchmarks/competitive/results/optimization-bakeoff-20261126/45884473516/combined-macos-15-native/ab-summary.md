| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.969x | 0.793x | 1.502x |
| materialize | 0.826x | 0.588x | 0.959x |
| perft_bulk_d4 | 0.927x | 0.837x | 1.206x |
| perft_full_d3 | 0.965x | 0.856x | 1.383x |
| fen_parse | 1.044x | 0.864x | 1.194x |
| fen_write_native_ep | 0.949x | 0.708x | 1.656x |
| snapshot | 0.939x | 0.785x | 0.914x |
| san_parse_all | 0.959x | 0.854x | 1.673x |
| san_render_native | 1.040x | 0.693x | 3.031x |
| apply_children_full | 0.944x | 0.780x | 1.095x |
| make_unmake_full | 1.024x | 0.800x | 1.291x |
| startpos | 1.466x | 1.466x | 1.247x |
| chess960_materialize | 1.030x | 1.001x | 1.534x |
| chess960_perft_d3 | 1.035x | 1.007x | 1.426x |
| polyglot_key | 1.105x | 1.104x | 47.797x |
| polyglot_recompute | 0.953x | 0.946x | 0.953x |
| repetition_key | 1.082x | 1.021x | 19.956x |
| corpus_count | 1.024x | 1.010x | 1.432x |
| corpus_materialize | 0.993x | 0.990x | 1.240x |
| corpus_uci_sorted | 1.202x | 1.137x | 1.592x |
| replay_trusted_native | 0.825x | 0.778x | 0.527x |
| replay_checked_native | 1.089x | 1.033x | 1.216x |

Cold first legal move list: 0.004 ms -> 0.006 ms (0.74x).
