| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.902x | 1.544x |
| materialize | 0.920x | 0.837x | 1.069x |
| perft_bulk_d4 | 0.928x | 0.755x | 1.268x |
| perft_full_d3 | 1.049x | 0.882x | 1.530x |
| fen_parse | 0.965x | 0.849x | 1.179x |
| fen_write_native_ep | 1.001x | 0.874x | 1.777x |
| snapshot | 0.994x | 0.704x | 0.963x |
| san_parse_all | 1.030x | 0.885x | 1.760x |
| san_render_native | 1.054x | 0.833x | 2.790x |
| apply_children_full | 1.138x | 0.917x | 1.125x |
| make_unmake_full | 1.067x | 0.873x | 1.257x |
| startpos | 1.045x | 1.045x | 1.179x |
| chess960_materialize | 1.295x | 1.265x | 1.562x |
| chess960_perft_d3 | 0.946x | 0.918x | 1.416x |
| polyglot_key | 1.177x | 1.098x | 38.466x |
| polyglot_recompute | 1.121x | 0.936x | 1.023x |
| repetition_key | 0.998x | 0.923x | 20.381x |
| corpus_count | 1.069x | 1.038x | 1.178x |
| corpus_materialize | 0.862x | 0.821x | 1.138x |
| corpus_uci_sorted | 0.988x | 0.938x | 1.380x |
| replay_trusted_native | 0.951x | 0.885x | 0.612x |
| replay_checked_native | 0.880x | 0.757x | 1.127x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.91x).
