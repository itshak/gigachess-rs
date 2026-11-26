| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.953x | 1.509x |
| materialize | 1.014x | 0.991x | 1.545x |
| perft_bulk_d4 | 0.996x | 0.984x | 1.161x |
| perft_full_d3 | 1.009x | 0.994x | 1.359x |
| fen_parse | 0.931x | 0.847x | 1.048x |
| fen_write_native_ep | 0.997x | 0.985x | 2.119x |
| snapshot | 1.000x | 0.993x | 0.997x |
| san_parse_all | 1.002x | 0.984x | 1.872x |
| san_render_native | 0.969x | 0.956x | 2.143x |
| apply_children_full | 0.997x | 0.990x | 1.360x |
| make_unmake_full | 0.999x | 0.972x | 1.202x |
| startpos | 0.801x | 0.801x | 0.986x |
| chess960_materialize | 0.993x | 0.985x | 1.556x |
| chess960_perft_d3 | 1.016x | 1.015x | 1.513x |
| polyglot_key | 0.980x | 0.959x | 38.480x |
| polyglot_recompute | 1.029x | 1.023x | 1.044x |
| repetition_key | 0.976x | 0.956x | 19.445x |
| corpus_count | 0.955x | 0.906x | 1.051x |
| corpus_materialize | 1.014x | 1.011x | 1.075x |
| corpus_uci_sorted | 1.026x | 1.026x | 1.553x |
| replay_trusted_native | 0.990x | 0.952x | 0.879x |
| replay_checked_native | 1.014x | 1.012x | 1.129x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.94x).
