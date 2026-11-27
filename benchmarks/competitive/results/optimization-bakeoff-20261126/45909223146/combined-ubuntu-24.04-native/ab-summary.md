| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.029x | 1.004x | 1.531x |
| materialize | 0.927x | 0.884x | 1.497x |
| perft_bulk_d4 | 1.054x | 1.034x | 1.246x |
| perft_full_d3 | 1.005x | 0.994x | 1.353x |
| fen_parse | 0.987x | 0.960x | 1.133x |
| fen_write_native_ep | 0.991x | 0.961x | 1.885x |
| snapshot | 0.996x | 0.980x | 1.153x |
| san_parse_all | 0.897x | 0.886x | 1.687x |
| san_render_native | 0.981x | 0.967x | 2.131x |
| apply_children_full | 0.997x | 0.988x | 1.334x |
| make_unmake_full | 0.980x | 0.948x | 1.180x |
| startpos | 0.883x | 0.883x | 1.107x |
| chess960_materialize | 0.991x | 0.987x | 1.548x |
| chess960_perft_d3 | 1.011x | 1.003x | 1.526x |
| polyglot_key | 1.004x | 0.998x | 40.007x |
| polyglot_recompute | 0.965x | 0.961x | 0.986x |
| repetition_key | 1.234x | 1.089x | 23.805x |
| corpus_count | 1.222x | 1.123x | 1.333x |
| corpus_materialize | 1.025x | 1.023x | 1.083x |
| corpus_uci_sorted | 1.020x | 1.015x | 1.576x |
| replay_trusted_native | 0.983x | 0.969x | 0.875x |
| replay_checked_native | 1.017x | 1.016x | 1.245x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.15x).
