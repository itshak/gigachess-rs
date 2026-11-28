| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.082x | 1.029x | 1.576x |
| materialize | 1.024x | 0.964x | 1.543x |
| perft_bulk_d4 | 1.069x | 1.053x | 1.243x |
| perft_full_d3 | 1.254x | 1.055x | 1.850x |
| fen_parse | 0.913x | 0.852x | 1.154x |
| fen_write_native_ep | 0.943x | 0.897x | 1.943x |
| snapshot | 1.003x | 0.993x | 1.157x |
| san_parse_all | 1.043x | 1.000x | 1.815x |
| san_render_native | 0.988x | 0.981x | 2.131x |
| apply_children_full | 1.004x | 0.994x | 1.343x |
| make_unmake_full | 0.971x | 0.905x | 1.174x |
| startpos | 0.895x | 0.895x | 1.119x |
| chess960_materialize | 1.037x | 0.996x | 1.429x |
| chess960_perft_d3 | 1.009x | 0.991x | 1.523x |
| polyglot_key | 0.958x | 0.921x | 39.982x |
| polyglot_recompute | 1.571x | 1.296x | 1.625x |
| repetition_key | 1.003x | 1.000x | 25.997x |
| corpus_count | 1.262x | 1.125x | 1.185x |
| corpus_materialize | 1.009x | 1.008x | 1.060x |
| corpus_uci_sorted | 1.001x | 1.000x | 1.534x |
| replay_trusted_native | 1.024x | 0.998x | 0.858x |
| replay_checked_native | 0.998x | 0.995x | 1.215x |

Cold first legal move list: 0.007 ms -> 0.009 ms (0.79x).
