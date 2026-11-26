| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.945x | 0.876x | 1.492x |
| materialize | 1.038x | 0.933x | 1.088x |
| perft_bulk_d4 | 0.952x | 0.920x | 1.244x |
| perft_full_d3 | 0.924x | 0.860x | 1.283x |
| fen_parse | 0.997x | 0.944x | 1.192x |
| fen_write_native_ep | 0.974x | 0.936x | 1.758x |
| snapshot | 0.952x | 0.795x | 0.976x |
| san_parse_all | 0.962x | 0.871x | 1.680x |
| san_render_native | 0.965x | 0.943x | 2.796x |
| apply_children_full | 0.912x | 0.708x | 1.093x |
| make_unmake_full | 0.984x | 0.895x | 1.306x |
| startpos | 0.954x | 0.954x | 1.196x |
| chess960_materialize | 1.008x | 0.959x | 1.556x |
| chess960_perft_d3 | 0.968x | 0.941x | 1.508x |
| polyglot_key | 1.011x | 0.934x | 40.146x |
| polyglot_recompute | 0.968x | 0.918x | 0.956x |
| repetition_key | 1.068x | 1.015x | 20.576x |
| corpus_count | 0.872x | 0.846x | 1.065x |
| corpus_materialize | 0.950x | 0.911x | 1.126x |
| corpus_uci_sorted | 1.020x | 1.008x | 1.424x |
| replay_trusted_native | 0.937x | 0.908x | 0.654x |
| replay_checked_native | 1.003x | 0.960x | 1.253x |

Cold first legal move list: 0.003 ms -> 0.003 ms (0.91x).
