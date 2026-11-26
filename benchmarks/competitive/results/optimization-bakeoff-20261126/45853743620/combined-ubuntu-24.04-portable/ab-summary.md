| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.993x | 1.297x |
| materialize | 1.094x | 1.055x | 1.737x |
| perft_bulk_d4 | 0.999x | 0.993x | 1.187x |
| perft_full_d3 | 1.000x | 0.995x | 1.338x |
| fen_parse | 0.997x | 0.979x | 1.183x |
| fen_write_native_ep | 0.982x | 0.966x | 1.754x |
| snapshot | 1.004x | 0.988x | 1.004x |
| san_parse_all | 0.999x | 0.991x | 1.859x |
| san_render_native | 0.998x | 0.983x | 2.159x |
| apply_children_full | 0.982x | 0.952x | 1.300x |
| make_unmake_full | 0.934x | 0.890x | 1.156x |
| startpos | 1.004x | 1.004x | 1.343x |
| chess960_materialize | 0.998x | 0.994x | 1.578x |
| chess960_perft_d3 | 0.996x | 0.995x | 1.513x |
| polyglot_key | 1.005x | 0.995x | 42.190x |
| polyglot_recompute | 0.979x | 0.974x | 1.008x |
| repetition_key | 0.953x | 0.928x | 19.879x |
| corpus_count | 0.964x | 0.927x | 1.127x |
| corpus_materialize | 1.011x | 1.006x | 1.149x |
| corpus_uci_sorted | 1.010x | 1.006x | 1.549x |
| replay_trusted_native | 0.950x | 0.896x | 0.873x |
| replay_checked_native | 1.042x | 1.010x | 1.089x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.91x).
