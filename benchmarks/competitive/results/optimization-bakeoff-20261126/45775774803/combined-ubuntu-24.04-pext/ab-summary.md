| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.988x | 1.377x |
| materialize | 1.007x | 0.994x | 1.166x |
| perft_bulk_d4 | 1.002x | 0.973x | 1.327x |
| perft_full_d3 | 0.980x | 0.961x | 0.651x |
| fen_parse | 1.142x | 1.098x | 1.165x |
| fen_write_native_ep | 1.004x | 0.986x | 1.235x |
| snapshot | 1.247x | 1.231x | 1.250x |
| san_parse_all | 1.062x | 1.046x | 2.182x |
| san_render_native | 0.998x | 0.987x | 2.214x |
| apply_children_full | 1.365x | 1.321x | 0.930x |
| make_unmake_full | 0.988x | 0.954x | 0.966x |
| startpos | 0.914x | 0.914x | 0.999x |
| chess960_materialize | 0.985x | 0.981x | 1.453x |
| chess960_perft_d3 | 1.013x | 1.012x | 1.441x |
| polyglot_key | 0.997x | 0.996x | 20.389x |
| polyglot_recompute | 1.004x | 0.999x | 1.026x |
| repetition_key | 1.005x | 1.005x | 19.260x |
| corpus_count | 1.003x | 1.002x | 1.392x |
| corpus_materialize | 0.979x | 0.967x | 1.294x |
| corpus_uci_sorted | 1.008x | 1.005x | 0.970x |
| replay_trusted_native | 0.993x | 0.987x | 0.614x |
| replay_checked_native | 1.015x | 0.986x | 0.963x |

Cold first legal move list: 0.010 ms -> 0.010 ms (0.99x).
