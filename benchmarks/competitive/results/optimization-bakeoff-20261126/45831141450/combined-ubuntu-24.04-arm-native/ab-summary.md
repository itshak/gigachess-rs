| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.947x | 0.898x | 1.304x |
| materialize | 0.979x | 0.952x | 1.225x |
| perft_bulk_d4 | 0.970x | 0.954x | 1.205x |
| perft_full_d3 | 0.940x | 0.901x | 0.982x |
| fen_parse | 1.008x | 0.990x | 1.226x |
| fen_write_native_ep | 0.948x | 0.907x | 1.491x |
| snapshot | 0.992x | 0.991x | 0.961x |
| san_parse_all | 0.974x | 0.966x | 1.762x |
| san_render_native | 0.965x | 0.947x | 2.462x |
| apply_children_full | 0.998x | 0.974x | 1.156x |
| make_unmake_full | 0.974x | 0.941x | 1.497x |
| startpos | 0.933x | 0.933x | 1.243x |
| chess960_materialize | 0.975x | 0.949x | 1.355x |
| chess960_perft_d3 | 0.978x | 0.975x | 1.318x |
| polyglot_key | 1.013x | 1.005x | 25.591x |
| polyglot_recompute | 0.991x | 0.986x | 1.072x |
| repetition_key | 0.991x | 0.987x | 18.858x |
| corpus_count | 0.971x | 0.964x | 1.177x |
| corpus_materialize | 0.984x | 0.980x | 1.160x |
| corpus_uci_sorted | 1.016x | 1.012x | 1.484x |
| replay_trusted_native | 0.988x | 0.977x | 0.611x |
| replay_checked_native | 0.992x | 0.982x | 1.040x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.97x).
