| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.923x | 1.327x |
| materialize | 1.023x | 0.975x | 1.094x |
| perft_bulk_d4 | 0.971x | 0.910x | 1.207x |
| perft_full_d3 | 1.026x | 0.998x | 0.994x |
| fen_parse | 0.961x | 0.897x | 1.115x |
| fen_write_native_ep | 1.054x | 1.037x | 1.813x |
| snapshot | 1.232x | 1.222x | 1.249x |
| san_parse_all | 1.025x | 1.022x | 2.202x |
| san_render_native | 0.990x | 0.966x | 2.114x |
| apply_children_full | 1.748x | 1.599x | 1.227x |
| make_unmake_full | 0.946x | 0.889x | 1.176x |
| startpos | 1.097x | 1.097x | 1.202x |
| chess960_materialize | 1.044x | 1.000x | 1.443x |
| chess960_perft_d3 | 1.056x | 0.989x | 1.494x |
| polyglot_key | 1.003x | 1.001x | 20.304x |
| polyglot_recompute | 0.952x | 0.937x | 0.977x |
| repetition_key | 0.813x | 0.781x | 15.511x |
| corpus_count | 0.901x | 0.805x | 1.176x |
| corpus_materialize | 0.960x | 0.950x | 1.146x |
| corpus_uci_sorted | 0.960x | 0.951x | 1.642x |
| replay_trusted_native | 1.373x | 1.113x | 1.082x |
| replay_checked_native | 1.094x | 1.070x | 1.077x |

Cold first legal move list: 0.015 ms -> 0.008 ms (1.84x).
