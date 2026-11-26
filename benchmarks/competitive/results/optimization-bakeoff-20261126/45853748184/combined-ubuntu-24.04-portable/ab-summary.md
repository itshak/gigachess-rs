| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.992x | 1.300x |
| materialize | 1.001x | 0.985x | 1.775x |
| perft_bulk_d4 | 0.987x | 0.976x | 1.179x |
| perft_full_d3 | 0.997x | 0.987x | 1.329x |
| fen_parse | 1.021x | 0.997x | 1.231x |
| fen_write_native_ep | 0.999x | 0.986x | 1.799x |
| snapshot | 0.996x | 0.986x | 1.002x |
| san_parse_all | 0.956x | 0.946x | 1.790x |
| san_render_native | 1.014x | 1.001x | 2.187x |
| apply_children_full | 0.957x | 0.922x | 1.292x |
| make_unmake_full | 0.961x | 0.888x | 1.199x |
| startpos | 0.999x | 0.999x | 1.294x |
| chess960_materialize | 0.999x | 0.998x | 1.592x |
| chess960_perft_d3 | 0.998x | 0.992x | 1.508x |
| polyglot_key | 0.972x | 0.953x | 43.026x |
| polyglot_recompute | 1.044x | 1.029x | 1.075x |
| repetition_key | 1.048x | 0.834x | 23.019x |
| corpus_count | 0.951x | 0.911x | 1.126x |
| corpus_materialize | 1.004x | 1.002x | 1.144x |
| corpus_uci_sorted | 0.998x | 0.997x | 1.561x |
| replay_trusted_native | 0.938x | 0.926x | 0.863x |
| replay_checked_native | 0.986x | 0.967x | 1.092x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.91x).
