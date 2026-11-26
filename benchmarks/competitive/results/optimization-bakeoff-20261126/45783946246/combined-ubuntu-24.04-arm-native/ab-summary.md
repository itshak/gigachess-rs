| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.356x |
| materialize | 1.034x | 0.955x | 1.279x |
| perft_bulk_d4 | 1.000x | 0.997x | 1.350x |
| perft_full_d3 | 1.011x | 1.001x | 1.076x |
| fen_parse | 1.002x | 0.996x | 1.244x |
| fen_write_native_ep | 1.002x | 0.996x | 1.576x |
| snapshot | 1.046x | 1.044x | 1.000x |
| san_parse_all | 0.998x | 0.986x | 1.833x |
| san_render_native | 0.999x | 0.993x | 2.553x |
| apply_children_full | 1.006x | 1.004x | 1.039x |
| make_unmake_full | 1.002x | 0.995x | 1.430x |
| startpos | 0.968x | 0.968x | 1.169x |
| chess960_materialize | 0.994x | 0.940x | 1.315x |
| chess960_perft_d3 | 0.996x | 0.992x | 1.326x |
| polyglot_key | 1.002x | 0.994x | 25.406x |
| polyglot_recompute | 0.997x | 0.995x | 1.070x |
| repetition_key | 0.989x | 0.977x | 18.689x |
| corpus_count | 1.002x | 1.001x | 1.496x |
| corpus_materialize | 1.046x | 1.044x | 1.257x |
| corpus_uci_sorted | 1.009x | 1.005x | 1.467x |
| replay_trusted_native | 1.004x | 1.001x | 0.497x |
| replay_checked_native | 0.988x | 0.981x | 0.980x |

Cold first legal move list: 0.011 ms -> 0.011 ms (1.03x).
