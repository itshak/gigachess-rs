| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.986x | 1.566x |
| materialize | 1.105x | 1.052x | 1.583x |
| perft_bulk_d4 | 1.001x | 0.987x | 1.195x |
| perft_full_d3 | 0.990x | 0.978x | 1.272x |
| fen_parse | 1.079x | 1.028x | 1.134x |
| fen_write_native_ep | 0.992x | 0.956x | 1.923x |
| snapshot | 0.999x | 0.997x | 1.331x |
| san_parse_all | 0.900x | 0.888x | 1.824x |
| san_render_native | 1.008x | 0.999x | 2.117x |
| apply_children_full | 1.021x | 1.011x | 1.248x |
| make_unmake_full | 0.947x | 0.901x | 1.105x |
| startpos | 1.126x | 1.126x | 1.251x |
| chess960_materialize | 1.061x | 1.036x | 1.475x |
| chess960_perft_d3 | 1.003x | 0.996x | 1.478x |
| polyglot_key | 0.979x | 0.966x | 39.854x |
| polyglot_recompute | 1.110x | 1.086x | 1.101x |
| repetition_key | 1.209x | 1.025x | 23.859x |
| corpus_count | 1.072x | 1.024x | 1.136x |
| corpus_materialize | 1.006x | 1.003x | 1.095x |
| corpus_uci_sorted | 1.003x | 1.002x | 1.576x |
| replay_trusted_native | 0.990x | 0.945x | 0.873x |
| replay_checked_native | 0.973x | 0.972x | 1.018x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.02x).
