| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.986x | 1.194x |
| materialize | 1.025x | 1.003x | 1.263x |
| perft_bulk_d4 | 0.989x | 0.964x | 1.155x |
| perft_full_d3 | 1.013x | 0.980x | 1.329x |
| fen_parse | 1.027x | 0.983x | 1.052x |
| fen_write_native_ep | 0.997x | 0.988x | 1.722x |
| snapshot | 1.002x | 0.999x | 0.988x |
| san_parse_all | 0.998x | 0.993x | 1.993x |
| san_render_native | 0.998x | 0.975x | 2.258x |
| apply_children_full | 1.008x | 0.975x | 1.388x |
| make_unmake_full | 0.992x | 0.975x | 1.363x |
| startpos | 1.000x | 1.000x | 1.348x |
| chess960_materialize | 1.075x | 1.036x | 1.352x |
| chess960_perft_d3 | 1.008x | 1.000x | 1.435x |
| polyglot_key | 1.006x | 1.004x | 23.635x |
| polyglot_recompute | 1.024x | 0.979x | 1.028x |
| repetition_key | 1.019x | 0.992x | 18.406x |
| corpus_count | 0.998x | 0.990x | 1.184x |
| corpus_materialize | 0.999x | 0.999x | 1.101x |
| corpus_uci_sorted | 0.995x | 0.985x | 1.701x |
| replay_trusted_native | 0.992x | 0.983x | 0.862x |
| replay_checked_native | 0.999x | 0.995x | 1.035x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.05x).
