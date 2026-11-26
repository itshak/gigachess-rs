| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.017x | 0.993x | 1.433x |
| materialize | 1.005x | 0.996x | 1.456x |
| perft_bulk_d4 | 0.992x | 0.985x | 1.235x |
| perft_full_d3 | 0.994x | 0.987x | 0.882x |
| fen_parse | 1.001x | 0.989x | 1.070x |
| fen_write_native_ep | 0.994x | 0.979x | 1.472x |
| snapshot | 0.700x | 0.687x | 0.994x |
| san_parse_all | 0.975x | 0.960x | 1.691x |
| san_render_native | 1.000x | 0.992x | 2.118x |
| apply_children_full | 1.022x | 0.997x | 0.925x |
| make_unmake_full | 1.003x | 0.996x | 0.941x |
| startpos | 1.203x | 1.203x | 1.990x |
| chess960_materialize | 1.011x | 1.005x | 1.318x |
| chess960_perft_d3 | 0.995x | 0.993x | 1.315x |
| polyglot_key | 1.005x | 1.003x | 40.404x |
| polyglot_recompute | 1.000x | 1.000x | 1.017x |
| repetition_key | 1.030x | 0.865x | 25.859x |
| corpus_count | 0.997x | 0.988x | 1.354x |
| corpus_materialize | 1.023x | 1.019x | 1.134x |
| corpus_uci_sorted | 0.997x | 0.992x | 0.966x |
| replay_trusted_native | 0.992x | 0.988x | 0.598x |
| replay_checked_native | 0.884x | 0.836x | 0.860x |

Cold first legal move list: 0.010 ms -> 0.010 ms (1.00x).
