| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.377x |
| materialize | 1.003x | 0.998x | 1.252x |
| perft_bulk_d4 | 0.994x | 0.990x | 1.238x |
| perft_full_d3 | 0.895x | 0.860x | 0.938x |
| fen_parse | 1.000x | 0.998x | 1.214x |
| fen_write_native_ep | 0.995x | 0.994x | 1.606x |
| snapshot | 1.001x | 0.997x | 0.949x |
| san_parse_all | 0.991x | 0.978x | 1.806x |
| san_render_native | 0.969x | 0.946x | 2.457x |
| apply_children_full | 0.957x | 0.899x | 1.107x |
| make_unmake_full | 0.910x | 0.884x | 1.397x |
| startpos | 0.998x | 0.998x | 1.299x |
| chess960_materialize | 0.944x | 0.883x | 1.304x |
| chess960_perft_d3 | 0.959x | 0.917x | 1.284x |
| polyglot_key | 0.982x | 0.965x | 25.044x |
| polyglot_recompute | 0.995x | 0.993x | 1.083x |
| repetition_key | 0.979x | 0.955x | 18.886x |
| corpus_count | 1.018x | 1.015x | 1.237x |
| corpus_materialize | 1.001x | 0.999x | 1.173x |
| corpus_uci_sorted | 1.005x | 1.004x | 1.476x |
| replay_trusted_native | 0.848x | 0.791x | 0.519x |
| replay_checked_native | 0.979x | 0.977x | 1.026x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.21x).
