| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.014x | 1.002x | 1.351x |
| materialize | 1.016x | 0.995x | 1.116x |
| perft_bulk_d4 | 1.019x | 0.990x | 1.224x |
| perft_full_d3 | 0.992x | 0.969x | 0.971x |
| fen_parse | 1.000x | 0.991x | 1.208x |
| fen_write_native_ep | 1.002x | 0.978x | 1.810x |
| snapshot | 0.999x | 0.997x | 1.249x |
| san_parse_all | 1.001x | 0.995x | 2.220x |
| san_render_native | 1.000x | 0.997x | 2.069x |
| apply_children_full | 1.000x | 0.998x | 1.256x |
| make_unmake_full | 0.981x | 0.896x | 1.170x |
| startpos | 1.001x | 1.001x | 1.201x |
| chess960_materialize | 0.997x | 0.991x | 1.479x |
| chess960_perft_d3 | 1.005x | 1.004x | 1.496x |
| polyglot_key | 1.008x | 1.004x | 20.508x |
| polyglot_recompute | 0.996x | 0.992x | 0.973x |
| repetition_key | 1.004x | 1.001x | 15.618x |
| corpus_count | 1.002x | 0.998x | 1.199x |
| corpus_materialize | 1.013x | 1.004x | 1.182x |
| corpus_uci_sorted | 0.997x | 0.985x | 1.653x |
| replay_trusted_native | 1.000x | 0.997x | 1.130x |
| replay_checked_native | 0.999x | 0.998x | 1.070x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.28x).
