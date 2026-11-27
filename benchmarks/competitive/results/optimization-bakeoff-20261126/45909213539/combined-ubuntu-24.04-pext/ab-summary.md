| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.996x | 1.557x |
| materialize | 1.046x | 1.019x | 1.639x |
| perft_bulk_d4 | 0.998x | 0.995x | 1.184x |
| perft_full_d3 | 0.989x | 0.980x | 1.339x |
| fen_parse | 0.999x | 0.994x | 1.074x |
| fen_write_native_ep | 0.994x | 0.980x | 2.112x |
| snapshot | 0.999x | 0.994x | 0.997x |
| san_parse_all | 0.998x | 0.993x | 1.842x |
| san_render_native | 0.969x | 0.958x | 2.102x |
| apply_children_full | 1.000x | 0.996x | 1.352x |
| make_unmake_full | 0.990x | 0.968x | 1.191x |
| startpos | 0.802x | 0.802x | 1.001x |
| chess960_materialize | 1.000x | 0.999x | 1.572x |
| chess960_perft_d3 | 1.002x | 1.002x | 1.473x |
| polyglot_key | 1.053x | 1.010x | 39.779x |
| polyglot_recompute | 0.995x | 0.989x | 0.987x |
| repetition_key | 1.040x | 1.015x | 23.894x |
| corpus_count | 1.011x | 1.010x | 1.117x |
| corpus_materialize | 1.009x | 1.007x | 1.082x |
| corpus_uci_sorted | 1.007x | 1.004x | 1.572x |
| replay_trusted_native | 1.007x | 0.997x | 0.909x |
| replay_checked_native | 1.001x | 1.000x | 1.172x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.01x).
