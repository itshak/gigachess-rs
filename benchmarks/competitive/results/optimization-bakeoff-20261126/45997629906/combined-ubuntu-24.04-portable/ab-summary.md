| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.993x | 1.272x |
| materialize | 1.203x | 1.137x | 1.759x |
| perft_bulk_d4 | 1.000x | 0.990x | 1.190x |
| perft_full_d3 | 1.006x | 0.991x | 1.331x |
| fen_parse | 1.004x | 0.957x | 1.141x |
| fen_write_native_ep | 1.002x | 0.988x | 1.783x |
| snapshot | 1.001x | 0.998x | 1.001x |
| san_parse_all | 0.986x | 0.975x | 1.876x |
| san_render_native | 0.990x | 0.967x | 2.184x |
| apply_children_full | 0.984x | 0.969x | 1.359x |
| make_unmake_full | 1.028x | 0.958x | 1.270x |
| startpos | 1.012x | 1.012x | 1.288x |
| chess960_materialize | 1.010x | 0.992x | 1.552x |
| chess960_perft_d3 | 0.997x | 0.985x | 1.498x |
| polyglot_key | 1.134x | 1.002x | 40.893x |
| polyglot_recompute | 1.070x | 1.024x | 1.018x |
| repetition_key | 1.072x | 0.998x | 20.984x |
| corpus_count | 1.038x | 0.997x | 1.174x |
| corpus_materialize | 0.998x | 0.997x | 1.135x |
| corpus_uci_sorted | 1.000x | 0.998x | 1.547x |
| replay_trusted_native | 0.924x | 0.895x | 0.816x |
| replay_checked_native | 0.987x | 0.981x | 1.146x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.82x).
