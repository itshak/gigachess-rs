| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.084x | 1.018x | 1.565x |
| materialize | 0.995x | 0.952x | 1.266x |
| perft_bulk_d4 | 1.015x | 0.984x | 1.406x |
| perft_full_d3 | 1.189x | 1.093x | 1.600x |
| fen_parse | 1.077x | 1.010x | 1.369x |
| fen_write_native_ep | 0.935x | 0.863x | 1.492x |
| snapshot | 0.964x | 0.842x | 0.962x |
| san_parse_all | 1.051x | 1.000x | 2.104x |
| san_render_native | 0.997x | 0.962x | 3.874x |
| apply_children_full | 0.978x | 0.931x | 1.403x |
| make_unmake_full | 0.956x | 0.835x | 1.334x |
| startpos | 0.996x | 0.996x | 1.248x |
| chess960_materialize | 0.977x | 0.972x | 1.358x |
| chess960_perft_d3 | 0.968x | 0.920x | 1.567x |
| polyglot_key | 0.933x | 0.912x | 27.198x |
| polyglot_recompute | 0.935x | 0.934x | 1.006x |
| repetition_key | 0.919x | 0.887x | 20.137x |
| corpus_count | 1.030x | 1.028x | 1.449x |
| corpus_materialize | 0.978x | 0.958x | 1.156x |
| corpus_uci_sorted | 1.004x | 0.995x | 1.384x |
| replay_trusted_native | 0.915x | 0.901x | 0.776x |
| replay_checked_native | 0.976x | 0.975x | 1.156x |

Cold first legal move list: 0.015 ms -> 0.019 ms (0.80x).
