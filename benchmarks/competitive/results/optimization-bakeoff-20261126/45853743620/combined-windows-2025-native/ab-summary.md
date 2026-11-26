| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.909x | 1.478x |
| materialize | 1.056x | 0.993x | 1.263x |
| perft_bulk_d4 | 0.989x | 0.787x | 1.302x |
| perft_full_d3 | 0.999x | 0.929x | 1.332x |
| fen_parse | 0.993x | 0.927x | 1.265x |
| fen_write_native_ep | 1.032x | 0.908x | 1.600x |
| snapshot | 0.935x | 0.656x | 0.909x |
| san_parse_all | 1.001x | 0.977x | 2.048x |
| san_render_native | 1.037x | 0.997x | 3.758x |
| apply_children_full | 1.034x | 0.972x | 1.402x |
| make_unmake_full | 0.992x | 0.911x | 1.314x |
| startpos | 1.000x | 1.000x | 1.248x |
| chess960_materialize | 0.981x | 0.977x | 1.323x |
| chess960_perft_d3 | 0.995x | 0.988x | 1.551x |
| polyglot_key | 0.994x | 0.982x | 29.222x |
| polyglot_recompute | 0.988x | 0.981x | 1.011x |
| repetition_key | 0.955x | 0.928x | 21.096x |
| corpus_count | 0.998x | 0.995x | 1.348x |
| corpus_materialize | 1.008x | 1.007x | 1.153x |
| corpus_uci_sorted | 1.001x | 0.999x | 1.428x |
| replay_trusted_native | 0.965x | 0.924x | 0.799x |
| replay_checked_native | 0.954x | 0.913x | 0.962x |

Cold first legal move list: 0.020 ms -> 0.020 ms (1.00x).
