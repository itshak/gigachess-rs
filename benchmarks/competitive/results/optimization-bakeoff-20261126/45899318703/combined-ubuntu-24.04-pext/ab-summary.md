| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.969x | 1.540x |
| materialize | 1.004x | 0.996x | 1.642x |
| perft_bulk_d4 | 1.007x | 0.990x | 1.196x |
| perft_full_d3 | 1.002x | 0.996x | 1.340x |
| fen_parse | 1.015x | 1.003x | 1.141x |
| fen_write_native_ep | 0.998x | 0.983x | 1.889x |
| snapshot | 1.009x | 1.002x | 0.999x |
| san_parse_all | 1.006x | 0.971x | 1.843x |
| san_render_native | 0.989x | 0.982x | 2.075x |
| apply_children_full | 1.003x | 0.998x | 1.353x |
| make_unmake_full | 0.998x | 0.984x | 1.199x |
| startpos | 0.893x | 0.893x | 0.993x |
| chess960_materialize | 0.945x | 0.943x | 1.440x |
| chess960_perft_d3 | 1.002x | 0.999x | 1.473x |
| polyglot_key | 1.014x | 1.007x | 39.873x |
| polyglot_recompute | 1.013x | 1.003x | 1.006x |
| repetition_key | 0.956x | 0.917x | 23.111x |
| corpus_count | 1.007x | 1.004x | 1.113x |
| corpus_materialize | 1.017x | 1.009x | 1.090x |
| corpus_uci_sorted | 1.008x | 1.008x | 1.571x |
| replay_trusted_native | 0.977x | 0.918x | 0.884x |
| replay_checked_native | 1.004x | 0.997x | 1.199x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.93x).
