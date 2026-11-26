| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.993x | 1.418x |
| materialize | 1.010x | 0.992x | 1.506x |
| perft_bulk_d4 | 0.999x | 0.992x | 1.211x |
| perft_full_d3 | 0.972x | 0.962x | 1.474x |
| fen_parse | 1.133x | 1.099x | 1.180x |
| fen_write_native_ep | 0.954x | 0.929x | 1.863x |
| snapshot | 0.773x | 0.767x | 0.997x |
| san_parse_all | 0.991x | 0.982x | 1.864x |
| san_render_native | 0.984x | 0.978x | 2.098x |
| apply_children_full | 0.968x | 0.959x | 1.050x |
| make_unmake_full | 0.981x | 0.940x | 1.129x |
| startpos | 0.814x | 0.814x | 1.078x |
| chess960_materialize | 0.986x | 0.982x | 1.451x |
| chess960_perft_d3 | 0.984x | 0.978x | 1.359x |
| polyglot_key | 1.070x | 1.001x | 39.576x |
| polyglot_recompute | 0.968x | 0.957x | 0.941x |
| repetition_key | 1.029x | 0.977x | 18.825x |
| corpus_count | 1.002x | 0.996x | 1.347x |
| corpus_materialize | 1.004x | 0.999x | 1.089x |
| corpus_uci_sorted | 1.001x | 0.998x | 1.572x |
| replay_trusted_native | 0.988x | 0.983x | 0.712x |
| replay_checked_native | 1.011x | 1.000x | 1.059x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.03x).
