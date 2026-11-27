| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.988x | 1.486x |
| materialize | 1.005x | 0.993x | 1.586x |
| perft_bulk_d4 | 0.995x | 0.986x | 1.159x |
| perft_full_d3 | 1.001x | 0.998x | 1.338x |
| fen_parse | 0.929x | 0.830x | 0.997x |
| fen_write_native_ep | 0.960x | 0.904x | 2.041x |
| snapshot | 1.005x | 0.999x | 1.330x |
| san_parse_all | 1.003x | 0.988x | 1.888x |
| san_render_native | 1.002x | 0.991x | 2.171x |
| apply_children_full | 1.009x | 1.000x | 1.268x |
| make_unmake_full | 0.955x | 0.916x | 1.139x |
| startpos | 1.000x | 1.000x | 1.254x |
| chess960_materialize | 0.997x | 0.993x | 1.551x |
| chess960_perft_d3 | 1.006x | 0.998x | 1.518x |
| polyglot_key | 0.997x | 0.995x | 39.191x |
| polyglot_recompute | 0.999x | 0.999x | 1.003x |
| repetition_key | 1.194x | 1.001x | 23.511x |
| corpus_count | 0.999x | 0.996x | 1.080x |
| corpus_materialize | 1.003x | 1.003x | 1.069x |
| corpus_uci_sorted | 1.003x | 1.001x | 1.558x |
| replay_trusted_native | 0.995x | 0.989x | 0.867x |
| replay_checked_native | 1.012x | 0.982x | 1.151x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.92x).
