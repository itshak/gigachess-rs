| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.980x | 0.915x | 1.518x |
| materialize | 0.969x | 0.913x | 1.234x |
| perft_bulk_d4 | 0.996x | 0.983x | 1.353x |
| perft_full_d3 | 1.000x | 0.993x | 1.324x |
| fen_parse | 1.058x | 1.032x | 1.333x |
| fen_write_native_ep | 1.001x | 0.993x | 1.604x |
| snapshot | 1.001x | 0.995x | 0.997x |
| san_parse_all | 0.985x | 0.972x | 2.071x |
| san_render_native | 1.000x | 0.981x | 3.845x |
| apply_children_full | 0.997x | 0.923x | 1.468x |
| make_unmake_full | 0.849x | 0.745x | 1.235x |
| startpos | 1.007x | 1.007x | 1.248x |
| chess960_materialize | 0.929x | 0.873x | 1.216x |
| chess960_perft_d3 | 0.963x | 0.928x | 1.490x |
| polyglot_key | 1.071x | 0.995x | 29.491x |
| polyglot_recompute | 0.995x | 0.994x | 1.011x |
| repetition_key | 1.073x | 1.017x | 21.784x |
| corpus_count | 0.987x | 0.977x | 1.361x |
| corpus_materialize | 0.968x | 0.966x | 1.141x |
| corpus_uci_sorted | 1.021x | 1.019x | 1.431x |
| replay_trusted_native | 0.995x | 0.988x | 0.828x |
| replay_checked_native | 0.986x | 0.984x | 1.058x |

Cold first legal move list: 0.015 ms -> 0.016 ms (0.90x).
