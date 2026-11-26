| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.985x | 0.977x | 1.523x |
| materialize | 1.000x | 0.991x | 1.326x |
| perft_bulk_d4 | 0.966x | 0.952x | 1.344x |
| perft_full_d3 | 0.941x | 0.920x | 1.263x |
| fen_parse | 1.009x | 0.997x | 1.217x |
| fen_write_native_ep | 0.999x | 0.997x | 1.606x |
| snapshot | 1.094x | 1.089x | 0.915x |
| san_parse_all | 0.948x | 0.939x | 1.746x |
| san_render_native | 0.909x | 0.876x | 2.351x |
| apply_children_full | 0.930x | 0.907x | 1.082x |
| make_unmake_full | 0.952x | 0.921x | 1.446x |
| startpos | 0.812x | 0.812x | 0.961x |
| chess960_materialize | 0.991x | 0.990x | 1.487x |
| chess960_perft_d3 | 0.958x | 0.950x | 1.429x |
| polyglot_key | 1.031x | 1.001x | 26.093x |
| polyglot_recompute | 1.011x | 1.008x | 1.095x |
| repetition_key | 1.058x | 1.000x | 20.065x |
| corpus_count | 0.986x | 0.977x | 1.355x |
| corpus_materialize | 0.981x | 0.978x | 1.213x |
| corpus_uci_sorted | 0.998x | 0.998x | 1.457x |
| replay_trusted_native | 0.940x | 0.938x | 0.581x |
| replay_checked_native | 0.946x | 0.943x | 0.995x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.99x).
