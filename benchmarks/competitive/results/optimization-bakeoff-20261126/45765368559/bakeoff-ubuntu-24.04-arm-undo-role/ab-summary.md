| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.088x |
| materialize | 1.003x | 0.998x | 1.172x |
| perft_bulk_d4 | 1.022x | 1.013x | 0.922x |
| perft_full_d3 | 1.032x | 1.020x | 0.628x |
| fen_parse | 0.993x | 0.986x | 1.191x |
| fen_write_native_ep | 0.992x | 0.988x | 1.038x |
| snapshot | 0.990x | 0.967x | 0.677x |
| san_parse_all | 1.016x | 1.000x | 0.930x |
| san_render_native | 1.003x | 0.972x | 1.151x |
| apply_children_full | 0.996x | 0.988x | 0.651x |
| make_unmake_full | 1.024x | 1.010x | 0.912x |
| startpos | 1.035x | 1.035x | 0.793x |
| chess960_materialize | 1.005x | 1.001x | 1.106x |
| chess960_perft_d3 | 1.030x | 1.016x | 0.868x |
| polyglot_key | 0.959x | 0.919x | 20.908x |
| polyglot_recompute | 1.004x | 1.002x | 1.136x |
| repetition_key | 1.002x | 1.000x | 14.293x |
| corpus_count | 0.984x | 0.972x | 1.184x |
| corpus_materialize | 0.994x | 0.992x | 1.107x |
| corpus_uci_sorted | 0.995x | 0.993x | 0.576x |
| replay_trusted_native | 0.975x | 0.969x | 0.314x |
| replay_checked_native | 0.985x | 0.984x | 0.649x |

Cold first legal move list: 154.078 ms → 154.000 ms (1.00x).
