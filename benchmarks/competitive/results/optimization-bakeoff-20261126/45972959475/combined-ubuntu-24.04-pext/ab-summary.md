| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.547x |
| materialize | 1.011x | 0.990x | 1.198x |
| perft_bulk_d4 | 0.989x | 0.974x | 1.380x |
| perft_full_d3 | 1.006x | 0.980x | 1.204x |
| fen_parse | 0.928x | 0.919x | 1.073x |
| fen_write_native_ep | 1.009x | 0.999x | 1.858x |
| snapshot | 0.999x | 0.994x | 1.248x |
| san_parse_all | 1.000x | 0.993x | 2.192x |
| san_render_native | 1.037x | 0.992x | 2.214x |
| apply_children_full | 1.000x | 0.959x | 1.296x |
| make_unmake_full | 1.097x | 0.970x | 1.299x |
| startpos | 1.001x | 1.001x | 1.201x |
| chess960_materialize | 1.019x | 1.009x | 1.575x |
| chess960_perft_d3 | 1.005x | 0.999x | 1.612x |
| polyglot_key | 1.007x | 1.000x | 20.828x |
| polyglot_recompute | 0.986x | 0.962x | 1.018x |
| repetition_key | 0.998x | 0.993x | 18.941x |
| corpus_count | 1.001x | 0.999x | 1.369x |
| corpus_materialize | 1.000x | 0.999x | 1.271x |
| corpus_uci_sorted | 0.997x | 0.992x | 1.665x |
| replay_trusted_native | 0.932x | 0.931x | 1.103x |
| replay_checked_native | 0.990x | 0.985x | 1.115x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.12x).
