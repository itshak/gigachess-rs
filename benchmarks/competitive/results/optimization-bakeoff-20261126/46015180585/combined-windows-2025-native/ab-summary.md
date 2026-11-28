| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.918x | 1.507x |
| materialize | 1.021x | 1.004x | 1.189x |
| perft_bulk_d4 | 1.048x | 1.011x | 1.443x |
| perft_full_d3 | 1.307x | 0.969x | 1.658x |
| fen_parse | 1.082x | 0.989x | 1.375x |
| fen_write_native_ep | 1.012x | 0.997x | 1.537x |
| snapshot | 0.996x | 0.967x | 1.503x |
| san_parse_all | 0.966x | 0.935x | 1.920x |
| san_render_native | 1.011x | 0.930x | 4.104x |
| apply_children_full | 1.030x | 0.960x | 1.369x |
| make_unmake_full | 1.004x | 0.969x | 1.301x |
| startpos | 0.965x | 0.965x | 1.571x |
| chess960_materialize | 0.958x | 0.952x | 1.386x |
| chess960_perft_d3 | 0.970x | 0.962x | 1.476x |
| polyglot_key | 1.009x | 0.999x | 35.073x |
| polyglot_recompute | 1.522x | 1.264x | 1.652x |
| repetition_key | 0.975x | 0.932x | 28.706x |
| corpus_count | 1.071x | 1.018x | 1.244x |
| corpus_materialize | 1.005x | 0.992x | 1.088x |
| corpus_uci_sorted | 0.985x | 0.979x | 1.380x |
| replay_trusted_native | 0.993x | 0.988x | 0.717x |
| replay_checked_native | 1.053x | 1.002x | 1.135x |

Cold first legal move list: 0.010 ms -> 0.010 ms (1.01x).
