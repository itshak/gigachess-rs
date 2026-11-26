| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.380x |
| materialize | 1.017x | 1.008x | 1.251x |
| perft_bulk_d4 | 1.003x | 0.996x | 1.241x |
| perft_full_d3 | 0.981x | 0.961x | 1.044x |
| fen_parse | 0.983x | 0.966x | 1.208x |
| fen_write_native_ep | 1.002x | 1.000x | 1.581x |
| snapshot | 0.993x | 0.986x | 0.960x |
| san_parse_all | 1.005x | 0.997x | 1.831x |
| san_render_native | 0.998x | 0.992x | 2.511x |
| apply_children_full | 0.892x | 0.849x | 1.149x |
| make_unmake_full | 1.040x | 1.013x | 1.572x |
| startpos | 1.000x | 1.000x | 1.330x |
| chess960_materialize | 1.011x | 1.006x | 1.396x |
| chess960_perft_d3 | 0.988x | 0.980x | 1.337x |
| polyglot_key | 1.009x | 1.002x | 25.945x |
| polyglot_recompute | 1.015x | 1.011x | 1.084x |
| repetition_key | 1.049x | 1.021x | 19.229x |
| corpus_count | 0.999x | 0.999x | 1.220x |
| corpus_materialize | 1.009x | 1.008x | 1.175x |
| corpus_uci_sorted | 0.998x | 0.998x | 1.450x |
| replay_trusted_native | 1.004x | 0.994x | 0.621x |
| replay_checked_native | 1.008x | 1.008x | 1.068x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.07x).
