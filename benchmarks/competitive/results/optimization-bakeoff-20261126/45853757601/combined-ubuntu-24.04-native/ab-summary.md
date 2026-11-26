| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.983x | 1.508x |
| materialize | 0.995x | 0.976x | 1.385x |
| perft_bulk_d4 | 1.010x | 1.007x | 1.211x |
| perft_full_d3 | 1.001x | 0.998x | 1.349x |
| fen_parse | 0.919x | 0.890x | 1.053x |
| fen_write_native_ep | 0.960x | 0.920x | 1.826x |
| snapshot | 0.998x | 0.991x | 1.228x |
| san_parse_all | 1.021x | 1.003x | 1.915x |
| san_render_native | 1.004x | 0.997x | 2.403x |
| apply_children_full | 0.965x | 0.941x | 1.214x |
| make_unmake_full | 1.012x | 0.987x | 1.348x |
| startpos | 1.002x | 1.002x | 1.119x |
| chess960_materialize | 1.001x | 0.998x | 1.486x |
| chess960_perft_d3 | 1.018x | 1.012x | 1.470x |
| polyglot_key | 1.025x | 1.000x | 31.398x |
| polyglot_recompute | 0.993x | 0.989x | 1.086x |
| repetition_key | 0.974x | 0.947x | 19.502x |
| corpus_count | 1.000x | 1.000x | 0.961x |
| corpus_materialize | 1.017x | 1.013x | 1.123x |
| corpus_uci_sorted | 1.005x | 1.005x | 1.519x |
| replay_trusted_native | 0.988x | 0.979x | 0.828x |
| replay_checked_native | 0.934x | 0.920x | 1.034x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.04x).
