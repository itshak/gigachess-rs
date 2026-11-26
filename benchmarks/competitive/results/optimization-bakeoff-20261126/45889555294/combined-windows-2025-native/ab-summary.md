| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.919x | 0.840x | 1.349x |
| materialize | 0.942x | 0.811x | 1.135x |
| perft_bulk_d4 | 0.916x | 0.859x | 1.215x |
| perft_full_d3 | 0.782x | 0.664x | 0.980x |
| fen_parse | 0.982x | 0.946x | 1.364x |
| fen_write_native_ep | 0.943x | 0.863x | 1.389x |
| snapshot | 0.802x | 0.780x | 1.199x |
| san_parse_all | 1.001x | 0.988x | 1.903x |
| san_render_native | 0.861x | 0.785x | 3.371x |
| apply_children_full | 0.675x | 0.574x | 0.922x |
| make_unmake_full | 0.935x | 0.892x | 1.243x |
| startpos | 0.859x | 0.859x | 0.863x |
| chess960_materialize | 0.844x | 0.814x | 1.183x |
| chess960_perft_d3 | 0.921x | 0.900x | 1.339x |
| polyglot_key | 0.991x | 0.902x | 35.163x |
| polyglot_recompute | 1.003x | 0.964x | 0.962x |
| repetition_key | 0.951x | 0.856x | 26.870x |
| corpus_count | 0.922x | 0.896x | 1.076x |
| corpus_materialize | 1.070x | 1.015x | 1.214x |
| corpus_uci_sorted | 1.097x | 1.087x | 1.485x |
| replay_trusted_native | 1.104x | 1.025x | 0.906x |
| replay_checked_native | 0.905x | 0.858x | 1.086x |

Cold first legal move list: 0.009 ms -> 0.011 ms (0.90x).
