| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.946x | 0.787x | 1.436x |
| materialize | 1.017x | 0.908x | 1.100x |
| perft_bulk_d4 | 0.925x | 0.837x | 1.207x |
| perft_full_d3 | 0.737x | 0.606x | 1.032x |
| fen_parse | 0.870x | 0.736x | 1.097x |
| fen_write_native_ep | 1.002x | 0.907x | 1.722x |
| snapshot | 1.013x | 0.733x | 1.008x |
| san_parse_all | 1.014x | 0.843x | 1.787x |
| san_render_native | 0.932x | 0.794x | 2.558x |
| apply_children_full | 1.031x | 0.863x | 1.175x |
| make_unmake_full | 0.950x | 0.812x | 1.162x |
| startpos | 0.693x | 0.693x | 0.987x |
| chess960_materialize | 1.127x | 0.831x | 1.447x |
| chess960_perft_d3 | 1.069x | 1.053x | 1.487x |
| polyglot_key | 1.132x | 1.113x | 43.395x |
| polyglot_recompute | 1.095x | 1.049x | 0.997x |
| repetition_key | 1.028x | 0.997x | 21.722x |
| corpus_count | 0.896x | 0.775x | 1.327x |
| corpus_materialize | 0.987x | 0.971x | 1.099x |
| corpus_uci_sorted | 0.981x | 0.941x | 1.455x |
| replay_trusted_native | 1.005x | 0.937x | 0.683x |
| replay_checked_native | 0.756x | 0.756x | 0.927x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.28x).
