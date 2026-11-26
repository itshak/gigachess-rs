| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.969x | 1.185x |
| materialize | 1.005x | 0.991x | 1.423x |
| perft_bulk_d4 | 1.002x | 0.985x | 0.881x |
| perft_full_d3 | 1.001x | 0.993x | 0.618x |
| fen_parse | 1.065x | 1.011x | 1.101x |
| fen_write_native_ep | 1.012x | 0.984x | 1.227x |
| snapshot | 1.116x | 1.109x | 0.921x |
| san_parse_all | 1.001x | 0.993x | 0.903x |
| san_render_native | 0.997x | 0.988x | 1.052x |
| apply_children_full | 1.010x | 1.005x | 0.629x |
| make_unmake_full | 0.999x | 0.987x | 0.758x |
| startpos | 1.114x | 1.114x | 0.748x |
| chess960_materialize | 0.998x | 0.993x | 1.204x |
| chess960_perft_d3 | 0.997x | 0.995x | 0.948x |
| polyglot_key | 1.102x | 0.954x | 38.720x |
| polyglot_recompute | 1.030x | 1.019x | 1.048x |
| repetition_key | 0.864x | 0.791x | 15.336x |
| corpus_count | 1.001x | 0.996x | 1.145x |
| corpus_materialize | 0.997x | 0.994x | 1.079x |
| corpus_uci_sorted | 1.002x | 0.999x | 0.622x |
| replay_trusted_native | 1.004x | 0.997x | 0.514x |
| replay_checked_native | 1.002x | 0.995x | 0.736x |

Cold first legal move list: 135.069 ms → 145.164 ms (0.93x).
