| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.937x | 0.902x | 1.412x |
| materialize | 0.993x | 0.952x | 1.231x |
| perft_bulk_d4 | 0.938x | 0.880x | 1.309x |
| perft_full_d3 | 0.877x | 0.778x | 0.960x |
| fen_parse | 1.142x | 1.093x | 1.285x |
| fen_write_native_ep | 0.996x | 0.990x | 1.562x |
| snapshot | 0.667x | 0.658x | 1.001x |
| san_parse_all | 0.903x | 0.884x | 1.865x |
| san_render_native | 0.852x | 0.802x | 3.186x |
| apply_children_full | 0.968x | 0.890x | 1.291x |
| make_unmake_full | 0.906x | 0.881x | 1.273x |
| startpos | 0.859x | 0.859x | 2.275x |
| chess960_materialize | 0.983x | 0.850x | 1.234x |
| chess960_perft_d3 | 0.887x | 0.827x | 1.362x |
| polyglot_key | 1.039x | 0.999x | 27.744x |
| polyglot_recompute | 0.993x | 0.987x | 0.987x |
| repetition_key | 1.001x | 0.984x | 21.571x |
| corpus_count | 0.990x | 0.974x | 1.371x |
| corpus_materialize | 1.034x | 1.025x | 1.184x |
| corpus_uci_sorted | 0.992x | 0.989x | 1.420x |
| replay_trusted_native | 0.953x | 0.923x | 0.774x |
| replay_checked_native | 0.896x | 0.854x | 0.906x |

Cold first legal move list: 0.022 ms -> 0.021 ms (1.05x).
