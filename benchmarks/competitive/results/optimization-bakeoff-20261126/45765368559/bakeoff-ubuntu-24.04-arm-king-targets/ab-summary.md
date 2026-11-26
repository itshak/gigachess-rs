| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.912x | 0.599x | 0.986x |
| materialize | 0.947x | 0.738x | 1.109x |
| perft_bulk_d4 | 0.968x | 0.915x | 0.876x |
| perft_full_d3 | 0.991x | 0.969x | 0.606x |
| fen_parse | 1.008x | 1.004x | 1.221x |
| fen_write_native_ep | 1.004x | 0.998x | 1.055x |
| snapshot | 1.006x | 0.992x | 0.762x |
| san_parse_all | 1.006x | 1.001x | 0.915x |
| san_render_native | 1.000x | 0.996x | 1.147x |
| apply_children_full | 1.006x | 0.993x | 0.651x |
| make_unmake_full | 1.009x | 1.002x | 0.901x |
| startpos | 1.021x | 1.021x | 0.796x |
| chess960_materialize | 0.995x | 0.994x | 1.098x |
| chess960_perft_d3 | 0.961x | 0.948x | 0.809x |
| polyglot_key | 0.983x | 0.973x | 23.166x |
| polyglot_recompute | 1.001x | 1.000x | 1.135x |
| repetition_key | 1.024x | 1.005x | 14.198x |
| corpus_count | 0.874x | 0.871x | 1.070x |
| corpus_materialize | 0.963x | 0.958x | 1.085x |
| corpus_uci_sorted | 0.998x | 0.997x | 0.571x |
| replay_trusted_native | 0.994x | 0.993x | 0.313x |
| replay_checked_native | 0.995x | 0.995x | 0.659x |

Cold first legal move list: 153.937 ms → 153.922 ms (1.00x).
