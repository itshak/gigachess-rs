| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.899x | 0.851x | 1.393x |
| materialize | 0.976x | 0.943x | 1.285x |
| perft_bulk_d4 | 0.930x | 0.904x | 1.294x |
| perft_full_d3 | 0.827x | 0.799x | 1.144x |
| fen_parse | 0.974x | 0.961x | 1.197x |
| fen_write_native_ep | 0.946x | 0.910x | 1.523x |
| snapshot | 1.389x | 1.308x | 1.136x |
| san_parse_all | 0.950x | 0.937x | 1.745x |
| san_render_native | 0.956x | 0.930x | 2.586x |
| apply_children_full | 0.957x | 0.921x | 1.104x |
| make_unmake_full | 0.937x | 0.926x | 1.455x |
| startpos | 1.091x | 1.091x | 1.290x |
| chess960_materialize | 0.956x | 0.952x | 1.425x |
| chess960_perft_d3 | 0.925x | 0.918x | 1.385x |
| polyglot_key | 1.030x | 0.955x | 26.315x |
| polyglot_recompute | 0.998x | 0.996x | 1.092x |
| repetition_key | 1.129x | 1.043x | 21.512x |
| corpus_count | 0.930x | 0.920x | 1.258x |
| corpus_materialize | 0.969x | 0.968x | 1.187x |
| corpus_uci_sorted | 1.021x | 1.020x | 1.491x |
| replay_trusted_native | 0.946x | 0.937x | 0.599x |
| replay_checked_native | 0.906x | 0.902x | 1.019x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.94x).
