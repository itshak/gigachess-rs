| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.941x | 1.383x |
| materialize | 1.011x | 0.973x | 1.243x |
| perft_bulk_d4 | 0.920x | 0.897x | 1.243x |
| perft_full_d3 | 1.005x | 0.971x | 1.058x |
| fen_parse | 0.992x | 0.985x | 1.230x |
| fen_write_native_ep | 1.001x | 0.996x | 1.590x |
| snapshot | 0.958x | 0.956x | 0.947x |
| san_parse_all | 0.988x | 0.976x | 1.806x |
| san_render_native | 0.991x | 0.981x | 2.524x |
| apply_children_full | 1.064x | 1.044x | 1.187x |
| make_unmake_full | 0.992x | 0.982x | 1.427x |
| startpos | 1.037x | 1.037x | 1.299x |
| chess960_materialize | 1.041x | 0.986x | 1.381x |
| chess960_perft_d3 | 1.027x | 0.969x | 1.346x |
| polyglot_key | 1.002x | 0.994x | 25.806x |
| polyglot_recompute | 0.998x | 0.998x | 1.069x |
| repetition_key | 0.955x | 0.931x | 18.371x |
| corpus_count | 0.826x | 0.794x | 1.225x |
| corpus_materialize | 0.984x | 0.975x | 1.172x |
| corpus_uci_sorted | 0.996x | 0.992x | 1.455x |
| replay_trusted_native | 0.997x | 0.986x | 0.525x |
| replay_checked_native | 0.996x | 0.996x | 0.978x |

Cold first legal move list: 0.009 ms -> 0.006 ms (1.62x).
