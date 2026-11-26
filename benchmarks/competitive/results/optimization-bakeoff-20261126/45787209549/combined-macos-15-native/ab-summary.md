| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.898x | 0.728x | 1.415x |
| materialize | 0.823x | 0.582x | 1.081x |
| perft_bulk_d4 | 0.886x | 0.675x | 1.401x |
| perft_full_d3 | 0.829x | 0.611x | 1.364x |
| fen_parse | 0.832x | 0.712x | 1.069x |
| fen_write_native_ep | 0.850x | 0.712x | 1.633x |
| snapshot | 0.932x | 0.750x | 1.025x |
| san_parse_all | 0.889x | 0.772x | 1.944x |
| san_render_native | 0.942x | 0.816x | 2.794x |
| apply_children_full | 0.964x | 0.927x | 0.978x |
| make_unmake_full | 0.904x | 0.787x | 1.226x |
| startpos | 1.111x | 1.111x | 1.324x |
| chess960_materialize | 1.203x | 1.195x | 1.520x |
| chess960_perft_d3 | 0.937x | 0.875x | 1.582x |
| polyglot_key | 1.128x | 1.015x | 46.225x |
| polyglot_recompute | 1.017x | 0.980x | 1.052x |
| repetition_key | 0.962x | 0.900x | 18.689x |
| corpus_count | 0.856x | 0.652x | 1.305x |
| corpus_materialize | 0.851x | 0.787x | 0.936x |
| corpus_uci_sorted | 0.953x | 0.835x | 1.516x |
| replay_trusted_native | 1.019x | 0.795x | 0.512x |
| replay_checked_native | 0.874x | 0.757x | 1.035x |

Cold first legal move list: 0.015 ms -> 0.023 ms (0.64x).
