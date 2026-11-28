| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.136x | 0.889x | 1.518x |
| materialize | 1.026x | 0.756x | 1.137x |
| perft_bulk_d4 | 1.088x | 1.011x | 1.353x |
| perft_full_d3 | 1.168x | 0.954x | 1.434x |
| fen_parse | 1.002x | 0.906x | 1.205x |
| fen_write_native_ep | 1.066x | 0.821x | 1.727x |
| snapshot | 0.984x | 0.908x | 0.981x |
| san_parse_all | 0.963x | 0.851x | 1.752x |
| san_render_native | 1.072x | 0.915x | 2.934x |
| apply_children_full | 1.077x | 0.911x | 1.077x |
| make_unmake_full | 1.079x | 0.933x | 1.289x |
| startpos | 1.031x | 1.031x | 1.347x |
| chess960_materialize | 1.048x | 1.013x | 1.461x |
| chess960_perft_d3 | 0.990x | 0.922x | 1.650x |
| polyglot_key | 0.967x | 0.966x | 36.131x |
| polyglot_recompute | 1.515x | 1.459x | 1.467x |
| repetition_key | 1.024x | 1.007x | 19.234x |
| corpus_count | 1.031x | 1.029x | 1.233x |
| corpus_materialize | 1.000x | 0.983x | 1.183x |
| corpus_uci_sorted | 1.041x | 0.932x | 1.410x |
| replay_trusted_native | 1.025x | 0.993x | 0.633x |
| replay_checked_native | 0.997x | 0.940x | 1.270x |

Cold first legal move list: 0.003 ms -> 0.003 ms (1.19x).
