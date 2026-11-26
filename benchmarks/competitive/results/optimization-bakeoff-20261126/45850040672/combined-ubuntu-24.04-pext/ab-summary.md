| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.991x | 1.544x |
| materialize | 0.997x | 0.991x | 1.208x |
| perft_bulk_d4 | 1.015x | 0.996x | 1.397x |
| perft_full_d3 | 1.003x | 0.990x | 1.196x |
| fen_parse | 1.018x | 0.975x | 1.191x |
| fen_write_native_ep | 0.995x | 0.985x | 1.862x |
| snapshot | 1.000x | 0.997x | 1.249x |
| san_parse_all | 1.001x | 0.994x | 2.343x |
| san_render_native | 1.016x | 0.994x | 2.130x |
| apply_children_full | 1.019x | 0.981x | 1.306x |
| make_unmake_full | 0.989x | 0.942x | 1.203x |
| startpos | 0.999x | 0.999x | 1.198x |
| chess960_materialize | 1.000x | 0.993x | 1.567x |
| chess960_perft_d3 | 0.997x | 0.996x | 1.610x |
| polyglot_key | 1.016x | 1.003x | 20.829x |
| polyglot_recompute | 0.971x | 0.955x | 0.999x |
| repetition_key | 1.004x | 1.003x | 19.004x |
| corpus_count | 0.999x | 0.998x | 1.369x |
| corpus_materialize | 0.998x | 0.997x | 1.269x |
| corpus_uci_sorted | 0.997x | 0.992x | 1.672x |
| replay_trusted_native | 1.091x | 1.055x | 1.176x |
| replay_checked_native | 1.083x | 1.080x | 1.092x |

Cold first legal move list: 0.009 ms -> 0.007 ms (1.42x).
