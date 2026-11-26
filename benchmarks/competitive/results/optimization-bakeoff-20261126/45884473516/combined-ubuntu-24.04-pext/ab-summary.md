| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.958x | 1.544x |
| materialize | 1.006x | 0.990x | 1.585x |
| perft_bulk_d4 | 1.000x | 0.986x | 1.189x |
| perft_full_d3 | 1.015x | 0.993x | 1.411x |
| fen_parse | 1.001x | 0.982x | 1.158x |
| fen_write_native_ep | 0.968x | 0.910x | 1.968x |
| snapshot | 1.001x | 0.997x | 1.332x |
| san_parse_all | 0.993x | 0.969x | 1.833x |
| san_render_native | 0.986x | 0.981x | 2.081x |
| apply_children_full | 0.980x | 0.970x | 1.230x |
| make_unmake_full | 0.965x | 0.944x | 1.151x |
| startpos | 1.126x | 1.126x | 1.251x |
| chess960_materialize | 1.070x | 1.055x | 1.520x |
| chess960_perft_d3 | 1.002x | 0.988x | 1.479x |
| polyglot_key | 1.042x | 1.000x | 38.741x |
| polyglot_recompute | 1.027x | 1.022x | 1.030x |
| repetition_key | 0.814x | 0.683x | 18.922x |
| corpus_count | 0.942x | 0.900x | 1.042x |
| corpus_materialize | 1.000x | 0.993x | 1.073x |
| corpus_uci_sorted | 1.009x | 1.007x | 1.561x |
| replay_trusted_native | 0.924x | 0.921x | 0.815x |
| replay_checked_native | 0.950x | 0.938x | 1.100x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.08x).
