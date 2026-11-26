| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.097x | 0.992x | 1.517x |
| materialize | 1.028x | 0.985x | 1.139x |
| perft_bulk_d4 | 0.916x | 0.891x | 1.297x |
| perft_full_d3 | 0.993x | 0.978x | 1.294x |
| fen_parse | 0.968x | 0.960x | 1.343x |
| fen_write_native_ep | 1.006x | 0.983x | 1.589x |
| snapshot | 0.975x | 0.873x | 1.123x |
| san_parse_all | 1.025x | 1.005x | 1.988x |
| san_render_native | 1.032x | 0.995x | 3.442x |
| apply_children_full | 0.874x | 0.803x | 1.136x |
| make_unmake_full | 0.917x | 0.846x | 1.189x |
| startpos | 0.999x | 0.999x | 1.001x |
| chess960_materialize | 1.000x | 0.996x | 1.357x |
| chess960_perft_d3 | 0.949x | 0.935x | 1.508x |
| polyglot_key | 0.999x | 0.995x | 20.168x |
| polyglot_recompute | 0.986x | 0.979x | 0.954x |
| repetition_key | 0.980x | 0.962x | 18.798x |
| corpus_count | 0.991x | 0.983x | 1.302x |
| corpus_materialize | 0.992x | 0.986x | 1.181x |
| corpus_uci_sorted | 1.001x | 1.000x | 1.505x |
| replay_trusted_native | 0.843x | 0.725x | 0.845x |
| replay_checked_native | 0.997x | 0.993x | 1.090x |

Cold first legal move list: 0.017 ms -> 0.016 ms (1.02x).
