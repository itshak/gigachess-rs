| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.953x | 0.892x | 1.242x |
| materialize | 0.988x | 0.969x | 1.337x |
| perft_bulk_d4 | 0.944x | 0.911x | 1.235x |
| perft_full_d3 | 0.945x | 0.929x | 1.183x |
| fen_parse | 0.797x | 0.772x | 0.968x |
| fen_write_native_ep | 0.992x | 0.987x | 1.647x |
| snapshot | 1.001x | 0.999x | 1.002x |
| san_parse_all | 0.922x | 0.907x | 1.876x |
| san_render_native | 0.942x | 0.879x | 2.110x |
| apply_children_full | 0.933x | 0.880x | 1.291x |
| make_unmake_full | 0.903x | 0.880x | 1.255x |
| startpos | 1.000x | 1.000x | 1.360x |
| chess960_materialize | 0.951x | 0.911x | 1.413x |
| chess960_perft_d3 | 0.939x | 0.888x | 1.427x |
| polyglot_key | 0.928x | 0.864x | 23.903x |
| polyglot_recompute | 1.019x | 1.012x | 1.031x |
| repetition_key | 0.954x | 0.929x | 17.961x |
| corpus_count | 0.957x | 0.943x | 1.248x |
| corpus_materialize | 0.965x | 0.957x | 1.140x |
| corpus_uci_sorted | 1.006x | 1.003x | 1.755x |
| replay_trusted_native | 0.864x | 0.842x | 0.758x |
| replay_checked_native | 0.954x | 0.945x | 1.004x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.03x).
