| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.954x | 0.928x | 1.487x |
| materialize | 1.049x | 1.009x | 1.542x |
| perft_bulk_d4 | 0.970x | 0.948x | 1.149x |
| perft_full_d3 | 0.908x | 0.895x | 1.231x |
| fen_parse | 0.999x | 0.964x | 1.125x |
| fen_write_native_ep | 0.887x | 0.853x | 1.689x |
| snapshot | 0.750x | 0.745x | 0.867x |
| san_parse_all | 1.015x | 0.984x | 1.853x |
| san_render_native | 0.982x | 0.964x | 2.139x |
| apply_children_full | 0.954x | 0.891x | 1.363x |
| make_unmake_full | 0.942x | 0.890x | 1.131x |
| startpos | 0.731x | 0.731x | 0.913x |
| chess960_materialize | 0.886x | 0.866x | 1.378x |
| chess960_perft_d3 | 0.969x | 0.931x | 1.428x |
| polyglot_key | 0.997x | 0.992x | 39.797x |
| polyglot_recompute | 0.959x | 0.956x | 0.963x |
| repetition_key | 0.947x | 0.934x | 22.188x |
| corpus_count | 0.968x | 0.964x | 1.089x |
| corpus_materialize | 1.007x | 0.996x | 1.073x |
| corpus_uci_sorted | 1.007x | 1.006x | 1.569x |
| replay_trusted_native | 0.943x | 0.891x | 0.830x |
| replay_checked_native | 0.883x | 0.868x | 1.043x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.05x).
