| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.082x | 1.010x | 1.262x |
| materialize | 0.980x | 0.876x | 1.256x |
| perft_bulk_d4 | 1.059x | 1.005x | 1.264x |
| perft_full_d3 | 0.984x | 0.947x | 1.148x |
| fen_parse | 1.008x | 0.963x | 1.239x |
| fen_write_native_ep | 0.996x | 0.956x | 1.698x |
| snapshot | 1.037x | 0.998x | 0.893x |
| san_parse_all | 1.004x | 0.991x | 2.140x |
| san_render_native | 0.987x | 0.970x | 2.304x |
| apply_children_full | 1.001x | 0.981x | 1.203x |
| make_unmake_full | 1.002x | 0.993x | 1.295x |
| startpos | 0.999x | 0.999x | 1.286x |
| chess960_materialize | 1.032x | 1.004x | 1.468x |
| chess960_perft_d3 | 1.068x | 1.050x | 1.510x |
| polyglot_key | 0.990x | 0.984x | 21.747x |
| polyglot_recompute | 0.995x | 0.992x | 0.976x |
| repetition_key | 0.949x | 0.880x | 19.211x |
| corpus_count | 1.023x | 0.971x | 1.260x |
| corpus_materialize | 1.030x | 0.991x | 1.337x |
| corpus_uci_sorted | 0.994x | 0.987x | 1.606x |
| replay_trusted_native | 1.007x | 1.001x | 0.754x |
| replay_checked_native | 1.003x | 1.001x | 0.972x |

Cold first legal move list: 0.010 ms -> 0.012 ms (0.87x).
