| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.994x | 1.208x |
| materialize | 0.998x | 0.974x | 1.253x |
| perft_bulk_d4 | 0.999x | 0.988x | 1.218x |
| perft_full_d3 | 1.004x | 0.977x | 1.389x |
| fen_parse | 0.990x | 0.960x | 1.183x |
| fen_write_native_ep | 0.995x | 0.982x | 1.675x |
| snapshot | 0.997x | 0.982x | 1.013x |
| san_parse_all | 1.042x | 1.025x | 2.089x |
| san_render_native | 0.976x | 0.953x | 2.319x |
| apply_children_full | 1.032x | 1.008x | 1.181x |
| make_unmake_full | 1.029x | 1.015x | 1.357x |
| startpos | 1.031x | 1.031x | 1.289x |
| chess960_materialize | 0.982x | 0.975x | 1.287x |
| chess960_perft_d3 | 0.993x | 0.988x | 1.367x |
| polyglot_key | 0.941x | 0.921x | 26.277x |
| polyglot_recompute | 0.998x | 0.996x | 1.057x |
| repetition_key | 0.928x | 0.880x | 19.242x |
| corpus_count | 0.996x | 0.994x | 1.288x |
| corpus_materialize | 1.008x | 1.000x | 1.202x |
| corpus_uci_sorted | 1.001x | 0.999x | 1.824x |
| replay_trusted_native | 1.041x | 1.020x | 0.735x |
| replay_checked_native | 1.016x | 1.015x | 1.077x |

Cold first legal move list: 0.013 ms -> 0.014 ms (0.90x).
