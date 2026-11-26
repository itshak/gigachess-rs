| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.040x | 0.997x | 1.372x |
| materialize | 0.971x | 0.953x | 1.055x |
| perft_bulk_d4 | 1.047x | 0.957x | 1.310x |
| perft_full_d3 | 0.885x | 0.825x | 0.964x |
| fen_parse | 0.990x | 0.973x | 1.246x |
| fen_write_native_ep | 1.003x | 0.997x | 1.605x |
| snapshot | 1.016x | 0.995x | 1.133x |
| san_parse_all | 1.006x | 0.986x | 1.969x |
| san_render_native | 0.963x | 0.923x | 3.233x |
| apply_children_full | 0.674x | 0.618x | 0.872x |
| make_unmake_full | 0.894x | 0.864x | 1.163x |
| startpos | 0.998x | 0.998x | 0.997x |
| chess960_materialize | 0.965x | 0.927x | 1.208x |
| chess960_perft_d3 | 0.923x | 0.874x | 1.362x |
| polyglot_key | 0.996x | 0.979x | 20.102x |
| polyglot_recompute | 1.003x | 1.001x | 0.957x |
| repetition_key | 0.985x | 0.978x | 18.872x |
| corpus_count | 1.006x | 1.002x | 1.199x |
| corpus_materialize | 1.009x | 1.006x | 1.135x |
| corpus_uci_sorted | 0.993x | 0.989x | 1.504x |
| replay_trusted_native | 0.803x | 0.715x | 0.803x |
| replay_checked_native | 1.041x | 0.978x | 0.988x |

Cold first legal move list: 0.012 ms -> 0.013 ms (0.98x).
