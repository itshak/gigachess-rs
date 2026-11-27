| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.988x | 1.576x |
| materialize | 1.012x | 0.999x | 1.349x |
| perft_bulk_d4 | 1.047x | 1.026x | 1.461x |
| perft_full_d3 | 1.125x | 0.995x | 1.499x |
| fen_parse | 0.999x | 0.995x | 1.219x |
| fen_write_native_ep | 1.000x | 0.998x | 1.575x |
| snapshot | 1.001x | 0.997x | 0.956x |
| san_parse_all | 0.999x | 0.993x | 1.829x |
| san_render_native | 1.004x | 0.996x | 2.674x |
| apply_children_full | 0.979x | 0.971x | 1.126x |
| make_unmake_full | 1.011x | 0.995x | 1.588x |
| startpos | 1.001x | 1.001x | 1.334x |
| chess960_materialize | 1.004x | 1.003x | 1.507x |
| chess960_perft_d3 | 1.024x | 1.004x | 1.532x |
| polyglot_key | 0.993x | 0.986x | 24.826x |
| polyglot_recompute | 0.997x | 0.997x | 1.085x |
| repetition_key | 0.987x | 0.973x | 18.706x |
| corpus_count | 1.114x | 1.100x | 1.542x |
| corpus_materialize | 1.013x | 1.008x | 1.262x |
| corpus_uci_sorted | 1.005x | 0.999x | 1.459x |
| replay_trusted_native | 0.994x | 0.992x | 0.623x |
| replay_checked_native | 1.006x | 1.003x | 1.134x |

Cold first legal move list: 0.006 ms -> 0.008 ms (0.72x).
