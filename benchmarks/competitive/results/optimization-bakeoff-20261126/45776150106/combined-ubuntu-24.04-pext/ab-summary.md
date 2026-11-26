| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.987x | 0.956x | 1.365x |
| materialize | 0.967x | 0.932x | 1.164x |
| perft_bulk_d4 | 0.901x | 0.883x | 1.205x |
| perft_full_d3 | 0.962x | 0.936x | 0.642x |
| fen_parse | 1.005x | 0.961x | 1.070x |
| fen_write_native_ep | 0.990x | 0.952x | 1.190x |
| snapshot | 0.811x | 0.778x | 0.913x |
| san_parse_all | 0.885x | 0.854x | 1.825x |
| san_render_native | 0.895x | 0.863x | 1.992x |
| apply_children_full | 0.867x | 0.830x | 0.792x |
| make_unmake_full | 0.791x | 0.747x | 0.767x |
| startpos | 0.756x | 0.756x | 0.894x |
| chess960_materialize | 1.038x | 1.036x | 1.399x |
| chess960_perft_d3 | 0.902x | 0.880x | 1.286x |
| polyglot_key | 1.031x | 0.999x | 21.150x |
| polyglot_recompute | 1.004x | 0.991x | 1.029x |
| repetition_key | 0.838x | 0.733x | 15.943x |
| corpus_count | 0.989x | 0.987x | 1.389x |
| corpus_materialize | 0.985x | 0.981x | 1.261x |
| corpus_uci_sorted | 1.002x | 1.000x | 0.963x |
| replay_trusted_native | 0.907x | 0.735x | 0.576x |
| replay_checked_native | 0.843x | 0.705x | 0.774x |

Cold first legal move list: 0.011 ms -> 0.011 ms (1.00x).
