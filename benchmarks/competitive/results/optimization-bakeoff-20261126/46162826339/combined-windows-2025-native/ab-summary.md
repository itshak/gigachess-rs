| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.061x | 1.009x | 1.558x |
| materialize | 1.044x | 1.004x | 1.315x |
| perft_bulk_d4 | 1.042x | 0.898x | 1.415x |
| perft_full_d3 | 1.262x | 1.071x | 1.618x |
| fen_parse | 0.987x | 0.881x | 1.327x |
| fen_write_native_ep | 1.011x | 0.972x | 1.549x |
| snapshot | 0.977x | 0.834x | 0.995x |
| san_parse_all | 0.974x | 0.937x | 2.080x |
| san_render_native | 1.004x | 0.981x | 4.014x |
| apply_children_full | 1.037x | 0.969x | 1.428x |
| make_unmake_full | 1.030x | 1.007x | 1.432x |
| startpos | 1.019x | 1.019x | 1.247x |
| chess960_materialize | 0.978x | 0.942x | 1.334x |
| chess960_perft_d3 | 0.993x | 0.968x | 1.600x |
| polyglot_key | 0.946x | 0.946x | 29.953x |
| polyglot_recompute | 1.169x | 1.092x | 1.273x |
| repetition_key | 0.867x | 0.839x | 20.525x |
| corpus_count | 1.043x | 1.038x | 1.535x |
| corpus_materialize | 0.982x | 0.972x | 1.211x |
| corpus_uci_sorted | 0.927x | 0.907x | 1.359x |
| replay_trusted_native | 1.020x | 1.011x | 0.921x |
| replay_checked_native | 0.804x | 0.714x | 0.924x |

Cold first legal move list: 0.020 ms -> 0.020 ms (0.99x).
