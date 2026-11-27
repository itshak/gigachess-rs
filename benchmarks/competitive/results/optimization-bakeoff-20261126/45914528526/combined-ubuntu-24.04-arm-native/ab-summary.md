| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.550x |
| materialize | 1.000x | 0.995x | 1.317x |
| perft_bulk_d4 | 1.001x | 0.998x | 1.393x |
| perft_full_d3 | 1.001x | 0.999x | 1.330x |
| fen_parse | 1.004x | 1.001x | 1.233x |
| fen_write_native_ep | 0.999x | 0.998x | 1.564x |
| snapshot | 0.989x | 0.982x | 0.940x |
| san_parse_all | 0.995x | 0.986x | 1.814x |
| san_render_native | 1.002x | 0.996x | 2.663x |
| apply_children_full | 0.981x | 0.970x | 1.142x |
| make_unmake_full | 1.016x | 1.002x | 1.576x |
| startpos | 0.999x | 0.999x | 1.307x |
| chess960_materialize | 1.003x | 0.996x | 1.493x |
| chess960_perft_d3 | 0.999x | 0.997x | 1.495x |
| polyglot_key | 0.995x | 0.986x | 25.784x |
| polyglot_recompute | 1.000x | 0.997x | 1.087x |
| repetition_key | 0.993x | 0.989x | 19.146x |
| corpus_count | 1.014x | 1.000x | 1.374x |
| corpus_materialize | 0.998x | 0.990x | 1.227x |
| corpus_uci_sorted | 0.997x | 0.996x | 1.456x |
| replay_trusted_native | 0.989x | 0.984x | 0.634x |
| replay_checked_native | 1.017x | 1.016x | 1.139x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.98x).
