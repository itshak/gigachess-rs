| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.997x | 1.424x |
| materialize | 0.994x | 0.977x | 1.486x |
| perft_bulk_d4 | 0.994x | 0.975x | 1.212x |
| perft_full_d3 | 1.001x | 0.962x | 1.544x |
| fen_parse | 1.047x | 1.019x | 1.171x |
| fen_write_native_ep | 1.025x | 0.938x | 2.088x |
| snapshot | 0.726x | 0.719x | 1.128x |
| san_parse_all | 1.110x | 1.063x | 1.834x |
| san_render_native | 1.012x | 1.001x | 2.163x |
| apply_children_full | 0.986x | 0.959x | 1.251x |
| make_unmake_full | 0.957x | 0.935x | 1.072x |
| startpos | 0.814x | 0.814x | 1.291x |
| chess960_materialize | 1.007x | 1.000x | 1.369x |
| chess960_perft_d3 | 1.001x | 1.001x | 1.373x |
| polyglot_key | 1.059x | 1.000x | 40.062x |
| polyglot_recompute | 1.001x | 0.996x | 1.021x |
| repetition_key | 0.649x | 0.632x | 14.421x |
| corpus_count | 0.998x | 0.996x | 1.307x |
| corpus_materialize | 0.988x | 0.988x | 1.074x |
| corpus_uci_sorted | 0.985x | 0.981x | 1.550x |
| replay_trusted_native | 0.956x | 0.954x | 0.774x |
| replay_checked_native | 0.964x | 0.944x | 1.033x |

Cold first legal move list: 0.012 ms -> 0.012 ms (0.96x).
