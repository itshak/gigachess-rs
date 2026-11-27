| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.995x | 1.487x |
| materialize | 1.016x | 1.005x | 1.167x |
| perft_bulk_d4 | 1.010x | 0.984x | 1.376x |
| perft_full_d3 | 1.032x | 1.008x | 1.300x |
| fen_parse | 1.019x | 0.948x | 1.394x |
| fen_write_native_ep | 1.020x | 1.005x | 1.498x |
| snapshot | 1.001x | 0.990x | 1.503x |
| san_parse_all | 1.037x | 1.005x | 1.962x |
| san_render_native | 1.000x | 0.925x | 4.012x |
| apply_children_full | 1.027x | 0.976x | 1.378x |
| make_unmake_full | 0.953x | 0.893x | 1.205x |
| startpos | 1.001x | 1.001x | 0.998x |
| chess960_materialize | 1.026x | 1.019x | 1.386x |
| chess960_perft_d3 | 1.012x | 1.009x | 1.466x |
| polyglot_key | 1.018x | 1.009x | 35.122x |
| polyglot_recompute | 1.044x | 1.018x | 0.962x |
| repetition_key | 1.046x | 1.041x | 28.006x |
| corpus_count | 1.002x | 1.002x | 1.171x |
| corpus_materialize | 1.020x | 1.011x | 1.121x |
| corpus_uci_sorted | 1.014x | 1.007x | 1.366x |
| replay_trusted_native | 1.086x | 1.035x | 0.762x |
| replay_checked_native | 1.023x | 0.983x | 1.137x |

Cold first legal move list: 0.018 ms -> 0.017 ms (1.06x).
