| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.919x | 0.893x | 1.428x |
| materialize | 1.038x | 1.005x | 1.508x |
| perft_bulk_d4 | 0.948x | 0.918x | 1.122x |
| perft_full_d3 | 0.904x | 0.889x | 1.265x |
| fen_parse | 1.072x | 1.033x | 1.145x |
| fen_write_native_ep | 0.908x | 0.850x | 1.915x |
| snapshot | 0.749x | 0.748x | 1.504x |
| san_parse_all | 0.918x | 0.882x | 1.692x |
| san_render_native | 1.001x | 0.948x | 2.133x |
| apply_children_full | 0.935x | 0.886x | 1.239x |
| make_unmake_full | 0.984x | 0.958x | 1.168x |
| startpos | 1.123x | 1.123x | 2.009x |
| chess960_materialize | 0.971x | 0.936x | 1.415x |
| chess960_perft_d3 | 0.941x | 0.916x | 1.387x |
| polyglot_key | 1.017x | 0.999x | 39.958x |
| polyglot_recompute | 0.950x | 0.943x | 0.958x |
| repetition_key | 1.020x | 0.929x | 24.300x |
| corpus_count | 0.978x | 0.977x | 1.095x |
| corpus_materialize | 0.988x | 0.986x | 1.070x |
| corpus_uci_sorted | 1.029x | 1.028x | 1.600x |
| replay_trusted_native | 0.976x | 0.952x | 0.875x |
| replay_checked_native | 0.890x | 0.881x | 1.034x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.90x).
