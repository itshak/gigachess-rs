| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.065x | 1.040x | 1.575x |
| materialize | 1.016x | 0.995x | 1.286x |
| perft_bulk_d4 | 1.029x | 1.004x | 1.435x |
| perft_full_d3 | 1.279x | 1.034x | 1.597x |
| fen_parse | 1.000x | 0.967x | 1.156x |
| fen_write_native_ep | 0.996x | 0.963x | 1.705x |
| snapshot | 1.293x | 1.282x | 1.247x |
| san_parse_all | 0.987x | 0.970x | 2.077x |
| san_render_native | 0.993x | 0.980x | 2.183x |
| apply_children_full | 1.051x | 0.988x | 1.375x |
| make_unmake_full | 1.049x | 1.016x | 1.348x |
| startpos | 1.003x | 1.003x | 1.252x |
| chess960_materialize | 0.999x | 0.981x | 1.399x |
| chess960_perft_d3 | 1.010x | 1.006x | 1.511x |
| polyglot_key | 1.079x | 0.997x | 28.164x |
| polyglot_recompute | 0.998x | 0.996x | 1.047x |
| repetition_key | 1.024x | 0.979x | 20.312x |
| corpus_count | 1.064x | 1.052x | 1.541x |
| corpus_materialize | 0.994x | 0.986x | 1.195x |
| corpus_uci_sorted | 0.995x | 0.992x | 1.800x |
| replay_trusted_native | 0.989x | 0.981x | 0.947x |
| replay_checked_native | 0.931x | 0.917x | 1.055x |

Cold first legal move list: 0.009 ms -> 0.007 ms (1.18x).
