| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.993x | 1.356x |
| materialize | 0.995x | 0.989x | 1.230x |
| perft_bulk_d4 | 1.002x | 1.000x | 1.348x |
| perft_full_d3 | 0.994x | 0.991x | 1.065x |
| fen_parse | 0.992x | 0.984x | 1.225x |
| fen_write_native_ep | 1.511x | 1.408x | 1.586x |
| snapshot | 0.981x | 0.968x | 0.987x |
| san_parse_all | 1.000x | 0.994x | 1.813x |
| san_render_native | 0.997x | 0.982x | 2.523x |
| apply_children_full | 0.994x | 0.978x | 1.041x |
| make_unmake_full | 1.008x | 0.998x | 1.444x |
| startpos | 0.931x | 0.931x | 1.169x |
| chess960_materialize | 1.009x | 1.005x | 1.327x |
| chess960_perft_d3 | 1.005x | 1.002x | 1.324x |
| polyglot_key | 0.995x | 0.982x | 25.430x |
| polyglot_recompute | 0.994x | 0.986x | 1.070x |
| repetition_key | 0.994x | 0.987x | 18.903x |
| corpus_count | 0.986x | 0.986x | 1.470x |
| corpus_materialize | 1.000x | 1.000x | 1.209x |
| corpus_uci_sorted | 0.990x | 0.989x | 1.453x |
| replay_trusted_native | 1.013x | 1.001x | 0.516x |
| replay_checked_native | 0.988x | 0.983x | 0.986x |

Cold first legal move list: 0.011 ms -> 0.011 ms (1.02x).
