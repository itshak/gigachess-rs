| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.993x | 0.967x | 1.497x |
| materialize | 0.917x | 0.885x | 1.560x |
| perft_bulk_d4 | 0.993x | 0.979x | 1.184x |
| perft_full_d3 | 0.998x | 0.994x | 1.352x |
| fen_parse | 0.990x | 0.971x | 1.044x |
| fen_write_native_ep | 0.953x | 0.894x | 1.795x |
| snapshot | 0.994x | 0.989x | 1.152x |
| san_parse_all | 0.995x | 0.986x | 1.719x |
| san_render_native | 0.998x | 0.997x | 2.099x |
| apply_children_full | 0.989x | 0.974x | 1.289x |
| make_unmake_full | 0.956x | 0.900x | 1.129x |
| startpos | 0.996x | 0.996x | 1.115x |
| chess960_materialize | 1.105x | 1.080x | 1.519x |
| chess960_perft_d3 | 1.003x | 1.001x | 1.518x |
| polyglot_key | 1.026x | 0.997x | 39.325x |
| polyglot_recompute | 0.994x | 0.994x | 1.011x |
| repetition_key | 0.937x | 0.903x | 21.871x |
| corpus_count | 0.998x | 0.995x | 1.108x |
| corpus_materialize | 0.996x | 0.994x | 1.060x |
| corpus_uci_sorted | 0.982x | 0.980x | 1.562x |
| replay_trusted_native | 1.004x | 0.996x | 0.893x |
| replay_checked_native | 1.014x | 0.997x | 1.075x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.94x).
