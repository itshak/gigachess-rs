| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.978x | 1.281x |
| materialize | 1.001x | 0.991x | 1.176x |
| perft_bulk_d4 | 1.004x | 0.980x | 1.343x |
| perft_full_d3 | 1.018x | 0.971x | 1.260x |
| fen_parse | 0.984x | 0.931x | 1.172x |
| fen_write_native_ep | 0.995x | 0.968x | 1.689x |
| snapshot | 1.304x | 1.245x | 1.057x |
| san_parse_all | 0.984x | 0.919x | 1.952x |
| san_render_native | 0.961x | 0.898x | 3.619x |
| apply_children_full | 0.990x | 0.938x | 1.200x |
| make_unmake_full | 0.997x | 0.962x | 1.252x |
| startpos | 0.491x | 0.491x | 0.546x |
| chess960_materialize | 1.000x | 0.996x | 1.240x |
| chess960_perft_d3 | 1.004x | 0.999x | 1.326x |
| polyglot_key | 0.891x | 0.787x | 27.683x |
| polyglot_recompute | 0.996x | 0.993x | 1.020x |
| repetition_key | 1.041x | 0.998x | 21.697x |
| corpus_count | 1.009x | 0.995x | 1.360x |
| corpus_materialize | 1.004x | 0.996x | 1.177x |
| corpus_uci_sorted | 1.135x | 0.990x | 1.417x |
| replay_trusted_native | 0.918x | 0.890x | 0.804x |
| replay_checked_native | 0.955x | 0.947x | 0.930x |

Cold first legal move list: 0.031 ms -> 0.030 ms (1.05x).
