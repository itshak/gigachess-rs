| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.961x | 0.901x | 1.483x |
| materialize | 1.005x | 0.960x | 1.114x |
| perft_bulk_d4 | 0.995x | 0.938x | 1.411x |
| perft_full_d3 | 1.334x | 1.079x | 1.685x |
| fen_parse | 1.065x | 1.054x | 1.534x |
| fen_write_native_ep | 1.012x | 1.004x | 1.619x |
| snapshot | 1.002x | 0.995x | 1.155x |
| san_parse_all | 0.991x | 0.938x | 2.202x |
| san_render_native | 0.997x | 0.962x | 3.309x |
| apply_children_full | 1.011x | 0.988x | 1.360x |
| make_unmake_full | 1.003x | 0.963x | 1.349x |
| startpos | 1.002x | 1.002x | 0.998x |
| chess960_materialize | 0.996x | 0.980x | 1.326x |
| chess960_perft_d3 | 1.042x | 1.020x | 1.589x |
| polyglot_key | 1.000x | 0.996x | 20.085x |
| polyglot_recompute | 1.030x | 1.005x | 1.001x |
| repetition_key | 1.045x | 1.009x | 19.018x |
| corpus_count | 0.956x | 0.928x | 1.206x |
| corpus_materialize | 1.000x | 0.973x | 1.192x |
| corpus_uci_sorted | 1.021x | 0.993x | 1.493x |
| replay_trusted_native | 0.923x | 0.858x | 0.994x |
| replay_checked_native | 1.031x | 1.006x | 1.108x |

Cold first legal move list: 0.018 ms -> 0.018 ms (0.96x).
