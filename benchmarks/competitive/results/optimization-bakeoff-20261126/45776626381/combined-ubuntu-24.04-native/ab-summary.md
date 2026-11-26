| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.025x | 1.010x | 1.415x |
| materialize | 1.015x | 0.999x | 1.513x |
| perft_bulk_d4 | 1.014x | 0.995x | 1.193x |
| perft_full_d3 | 1.710x | 1.601x | 1.449x |
| fen_parse | 0.931x | 0.906x | 1.017x |
| fen_write_native_ep | 1.045x | 1.020x | 1.237x |
| snapshot | 0.913x | 0.909x | 1.000x |
| san_parse_all | 0.988x | 0.955x | 1.880x |
| san_render_native | 1.006x | 0.986x | 2.110x |
| apply_children_full | 0.772x | 0.729x | 0.786x |
| make_unmake_full | 1.212x | 1.151x | 1.154x |
| startpos | 0.839x | 0.839x | 1.250x |
| chess960_materialize | 1.061x | 1.042x | 1.460x |
| chess960_perft_d3 | 1.043x | 1.030x | 1.380x |
| polyglot_key | 1.076x | 1.002x | 39.890x |
| polyglot_recompute | 1.049x | 1.033x | 1.016x |
| repetition_key | 0.960x | 0.932x | 21.299x |
| corpus_count | 1.031x | 1.029x | 1.345x |
| corpus_materialize | 0.996x | 0.995x | 1.092x |
| corpus_uci_sorted | 1.600x | 1.590x | 1.551x |
| replay_trusted_native | 1.170x | 1.155x | 0.724x |
| replay_checked_native | 1.100x | 1.066x | 1.014x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.06x).
