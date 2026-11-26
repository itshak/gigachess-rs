| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.128x | 1.076x | 1.223x |
| materialize | 1.022x | 0.993x | 1.203x |
| perft_bulk_d4 | 1.219x | 1.197x | 1.070x |
| perft_full_d3 | 0.996x | 0.992x | 0.598x |
| fen_parse | 0.991x | 0.990x | 1.184x |
| fen_write_native_ep | 1.005x | 0.999x | 1.061x |
| snapshot | 0.983x | 0.975x | 0.661x |
| san_parse_all | 1.001x | 0.998x | 0.909x |
| san_render_native | 0.990x | 0.960x | 1.137x |
| apply_children_full | 1.002x | 0.959x | 0.654x |
| make_unmake_full | 0.998x | 0.968x | 0.894x |
| startpos | 1.045x | 1.045x | 0.776x |
| chess960_materialize | 1.010x | 1.008x | 1.116x |
| chess960_perft_d3 | 1.150x | 1.144x | 0.970x |
| polyglot_key | 0.991x | 0.988x | 22.947x |
| polyglot_recompute | 1.003x | 0.996x | 1.134x |
| repetition_key | 1.006x | 1.001x | 14.005x |
| corpus_count | 1.071x | 1.031x | 1.293x |
| corpus_materialize | 1.020x | 1.017x | 1.142x |
| corpus_uci_sorted | 1.013x | 1.013x | 0.575x |
| replay_trusted_native | 0.988x | 0.981x | 0.321x |
| replay_checked_native | 1.011x | 1.006x | 0.659x |

Cold first legal move list: 154.014 ms → 153.761 ms (1.00x).
