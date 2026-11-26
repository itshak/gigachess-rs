| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.992x | 1.375x |
| materialize | 0.939x | 0.909x | 1.190x |
| perft_bulk_d4 | 0.996x | 0.977x | 1.324x |
| perft_full_d3 | 1.051x | 1.030x | 0.672x |
| fen_parse | 1.031x | 0.963x | 1.089x |
| fen_write_native_ep | 1.536x | 1.423x | 1.863x |
| snapshot | 0.642x | 0.620x | 0.963x |
| san_parse_all | 0.999x | 0.989x | 2.085x |
| san_render_native | 0.991x | 0.965x | 2.230x |
| apply_children_full | 1.007x | 0.978x | 0.876x |
| make_unmake_full | 0.995x | 0.970x | 0.972x |
| startpos | 1.199x | 1.199x | 1.398x |
| chess960_materialize | 0.988x | 0.981x | 1.372x |
| chess960_perft_d3 | 0.992x | 0.991x | 1.394x |
| polyglot_key | 0.995x | 0.990x | 20.331x |
| polyglot_recompute | 1.006x | 1.002x | 1.022x |
| repetition_key | 1.003x | 0.996x | 19.051x |
| corpus_count | 1.001x | 0.998x | 1.402x |
| corpus_materialize | 0.998x | 0.998x | 1.276x |
| corpus_uci_sorted | 1.000x | 0.993x | 0.967x |
| replay_trusted_native | 0.990x | 0.989x | 0.648x |
| replay_checked_native | 1.012x | 1.004x | 0.939x |

Cold first legal move list: 0.010 ms -> 0.009 ms (1.12x).
