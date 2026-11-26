| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.998x | 1.542x |
| materialize | 1.005x | 0.993x | 1.631x |
| perft_bulk_d4 | 1.020x | 1.004x | 1.203x |
| perft_full_d3 | 0.972x | 0.940x | 1.309x |
| fen_parse | 0.937x | 0.853x | 1.051x |
| fen_write_native_ep | 0.961x | 0.918x | 2.033x |
| snapshot | 0.709x | 0.707x | 1.154x |
| san_parse_all | 1.120x | 1.092x | 1.917x |
| san_render_native | 1.009x | 0.998x | 2.126x |
| apply_children_full | 1.012x | 0.989x | 1.382x |
| make_unmake_full | 1.002x | 0.978x | 1.165x |
| startpos | 0.912x | 0.912x | 1.298x |
| chess960_materialize | 0.936x | 0.931x | 1.467x |
| chess960_perft_d3 | 0.998x | 0.984x | 1.466x |
| polyglot_key | 0.975x | 0.950x | 38.559x |
| polyglot_recompute | 1.057x | 1.054x | 1.031x |
| repetition_key | 0.750x | 0.725x | 17.248x |
| corpus_count | 0.996x | 0.992x | 1.063x |
| corpus_materialize | 0.994x | 0.988x | 1.080x |
| corpus_uci_sorted | 1.009x | 1.007x | 1.573x |
| replay_trusted_native | 1.019x | 1.011x | 0.925x |
| replay_checked_native | 1.008x | 1.004x | 1.058x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.14x).
