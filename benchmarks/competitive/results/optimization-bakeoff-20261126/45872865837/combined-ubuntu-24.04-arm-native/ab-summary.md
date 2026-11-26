| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.993x | 1.552x |
| materialize | 1.002x | 0.998x | 1.328x |
| perft_bulk_d4 | 1.001x | 0.997x | 1.403x |
| perft_full_d3 | 1.142x | 1.028x | 1.519x |
| fen_parse | 1.000x | 0.994x | 1.231x |
| fen_write_native_ep | 0.999x | 0.997x | 1.576x |
| snapshot | 0.991x | 0.985x | 0.957x |
| san_parse_all | 1.000x | 0.997x | 1.844x |
| san_render_native | 1.003x | 0.997x | 2.687x |
| apply_children_full | 0.997x | 0.987x | 1.151x |
| make_unmake_full | 1.046x | 1.019x | 1.644x |
| startpos | 1.001x | 1.001x | 1.333x |
| chess960_materialize | 1.006x | 1.000x | 1.507x |
| chess960_perft_d3 | 1.006x | 1.002x | 1.508x |
| polyglot_key | 0.998x | 0.988x | 25.475x |
| polyglot_recompute | 0.996x | 0.996x | 1.086x |
| repetition_key | 1.001x | 0.999x | 19.188x |
| corpus_count | 1.009x | 1.004x | 1.376x |
| corpus_materialize | 0.999x | 0.999x | 1.222x |
| corpus_uci_sorted | 1.002x | 0.998x | 1.458x |
| replay_trusted_native | 1.071x | 1.027x | 0.689x |
| replay_checked_native | 0.994x | 0.983x | 1.122x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.99x).
