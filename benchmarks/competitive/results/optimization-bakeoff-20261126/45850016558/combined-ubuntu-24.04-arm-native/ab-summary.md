| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.996x | 1.552x |
| materialize | 1.004x | 1.000x | 1.321x |
| perft_bulk_d4 | 1.003x | 0.999x | 1.399x |
| perft_full_d3 | 1.001x | 0.987x | 1.331x |
| fen_parse | 0.999x | 0.993x | 1.205x |
| fen_write_native_ep | 1.001x | 0.998x | 1.578x |
| snapshot | 1.085x | 1.080x | 1.015x |
| san_parse_all | 1.007x | 1.000x | 1.830x |
| san_render_native | 1.000x | 0.993x | 2.527x |
| apply_children_full | 0.998x | 0.973x | 1.157x |
| make_unmake_full | 1.018x | 0.990x | 1.561x |
| startpos | 0.901x | 0.901x | 1.129x |
| chess960_materialize | 1.002x | 1.002x | 1.495x |
| chess960_perft_d3 | 0.996x | 0.979x | 1.485x |
| polyglot_key | 0.997x | 0.986x | 25.525x |
| polyglot_recompute | 0.998x | 0.995x | 1.085x |
| repetition_key | 0.951x | 0.930x | 18.454x |
| corpus_count | 0.995x | 0.993x | 1.369x |
| corpus_materialize | 1.000x | 0.999x | 1.218x |
| corpus_uci_sorted | 0.999x | 0.997x | 1.461x |
| replay_trusted_native | 1.011x | 1.003x | 0.628x |
| replay_checked_native | 0.997x | 0.996x | 1.042x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.97x).
