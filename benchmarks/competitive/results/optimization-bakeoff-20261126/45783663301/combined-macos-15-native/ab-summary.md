| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.927x | 0.771x | 1.318x |
| materialize | 0.984x | 0.946x | 1.027x |
| perft_bulk_d4 | 0.988x | 0.952x | 1.322x |
| perft_full_d3 | 0.970x | 0.877x | 1.330x |
| fen_parse | 0.972x | 0.932x | 1.241x |
| fen_write_native_ep | 0.910x | 0.820x | 1.174x |
| snapshot | 0.921x | 0.665x | 0.931x |
| san_parse_all | 0.954x | 0.807x | 1.781x |
| san_render_native | 0.991x | 0.955x | 2.623x |
| apply_children_full | 0.956x | 0.834x | 0.945x |
| make_unmake_full | 0.941x | 0.760x | 1.202x |
| startpos | 0.998x | 0.998x | 1.232x |
| chess960_materialize | 0.921x | 0.912x | 1.353x |
| chess960_perft_d3 | 0.930x | 0.888x | 1.470x |
| polyglot_key | 0.900x | 0.867x | 36.241x |
| polyglot_recompute | 0.944x | 0.876x | 0.967x |
| repetition_key | 0.975x | 0.895x | 19.209x |
| corpus_count | 0.983x | 0.964x | 1.380x |
| corpus_materialize | 0.995x | 0.919x | 1.094x |
| corpus_uci_sorted | 1.011x | 0.978x | 1.360x |
| replay_trusted_native | 0.993x | 0.963x | 0.585x |
| replay_checked_native | 1.028x | 1.020x | 0.969x |

Cold first legal move list: 0.015 ms -> 0.009 ms (1.62x).
