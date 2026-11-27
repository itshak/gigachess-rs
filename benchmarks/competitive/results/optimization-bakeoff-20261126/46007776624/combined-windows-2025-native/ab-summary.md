| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.077x | 0.998x | 1.564x |
| materialize | 1.025x | 0.946x | 1.143x |
| perft_bulk_d4 | 1.083x | 1.019x | 1.473x |
| perft_full_d3 | 1.335x | 1.158x | 1.698x |
| fen_parse | 0.972x | 0.928x | 1.262x |
| fen_write_native_ep | 1.023x | 0.989x | 1.592x |
| snapshot | 0.980x | 0.904x | 1.187x |
| san_parse_all | 0.977x | 0.939x | 1.950x |
| san_render_native | 1.070x | 1.030x | 3.340x |
| apply_children_full | 1.003x | 0.902x | 1.315x |
| make_unmake_full | 1.020x | 0.999x | 1.288x |
| startpos | 0.977x | 0.977x | 0.973x |
| chess960_materialize | 0.998x | 0.986x | 1.350x |
| chess960_perft_d3 | 1.010x | 0.991x | 1.612x |
| polyglot_key | 1.000x | 0.997x | 21.148x |
| polyglot_recompute | 0.789x | 0.760x | 0.760x |
| repetition_key | 0.999x | 0.974x | 18.902x |
| corpus_count | 1.055x | 1.032x | 1.430x |
| corpus_materialize | 0.994x | 0.980x | 1.247x |
| corpus_uci_sorted | 0.962x | 0.935x | 1.463x |
| replay_trusted_native | 1.016x | 0.889x | 1.124x |
| replay_checked_native | 0.969x | 0.944x | 1.094x |

Cold first legal move list: 0.018 ms -> 0.018 ms (1.00x).
