| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.985x | 1.158x |
| materialize | 0.949x | 0.917x | 1.228x |
| perft_bulk_d4 | 0.994x | 0.980x | 1.179x |
| perft_full_d3 | 1.018x | 0.998x | 1.142x |
| fen_parse | 0.843x | 0.805x | 1.067x |
| fen_write_native_ep | 1.003x | 0.995x | 1.196x |
| snapshot | 1.000x | 0.999x | 0.920x |
| san_parse_all | 0.990x | 0.974x | 2.158x |
| san_render_native | 1.005x | 0.976x | 2.392x |
| apply_children_full | 1.001x | 0.994x | 1.094x |
| make_unmake_full | 1.020x | 0.994x | 1.298x |
| startpos | 1.008x | 1.008x | 1.311x |
| chess960_materialize | 1.005x | 1.003x | 1.484x |
| chess960_perft_d3 | 0.997x | 0.996x | 1.411x |
| polyglot_key | 1.001x | 0.999x | 22.098x |
| polyglot_recompute | 0.908x | 0.876x | 0.900x |
| repetition_key | 0.998x | 0.997x | 21.343x |
| corpus_count | 1.000x | 1.000x | 1.215x |
| corpus_materialize | 0.996x | 0.994x | 1.299x |
| corpus_uci_sorted | 0.996x | 0.992x | 1.614x |
| replay_trusted_native | 1.003x | 0.996x | 0.685x |
| replay_checked_native | 0.993x | 0.989x | 0.980x |

Cold first legal move list: 0.011 ms -> 0.011 ms (0.98x).
