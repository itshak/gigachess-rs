| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.941x | 1.384x |
| materialize | 1.007x | 0.969x | 1.243x |
| perft_bulk_d4 | 0.922x | 0.901x | 1.241x |
| perft_full_d3 | 1.011x | 0.980x | 1.074x |
| fen_parse | 0.991x | 0.988x | 1.231x |
| fen_write_native_ep | 1.002x | 0.997x | 1.606x |
| snapshot | 0.921x | 0.915x | 0.936x |
| san_parse_all | 0.994x | 0.986x | 1.818x |
| san_render_native | 0.994x | 0.986x | 2.547x |
| apply_children_full | 1.080x | 1.016x | 1.131x |
| make_unmake_full | 0.993x | 0.983x | 1.418x |
| startpos | 1.073x | 1.073x | 1.274x |
| chess960_materialize | 1.023x | 0.954x | 1.357x |
| chess960_perft_d3 | 1.014x | 0.963x | 1.351x |
| polyglot_key | 0.992x | 0.983x | 25.334x |
| polyglot_recompute | 0.997x | 0.997x | 1.071x |
| repetition_key | 0.961x | 0.920x | 18.281x |
| corpus_count | 0.824x | 0.797x | 1.208x |
| corpus_materialize | 0.975x | 0.966x | 1.170x |
| corpus_uci_sorted | 0.997x | 0.995x | 1.457x |
| replay_trusted_native | 1.013x | 1.010x | 0.512x |
| replay_checked_native | 1.005x | 1.001x | 0.995x |

Cold first legal move list: 0.010 ms -> 0.005 ms (2.24x).
