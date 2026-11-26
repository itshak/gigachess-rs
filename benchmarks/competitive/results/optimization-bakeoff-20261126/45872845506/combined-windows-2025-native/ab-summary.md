| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.999x | 1.491x |
| materialize | 0.996x | 0.989x | 1.292x |
| perft_bulk_d4 | 1.004x | 0.975x | 1.358x |
| perft_full_d3 | 1.001x | 0.994x | 1.321x |
| fen_parse | 1.016x | 1.005x | 1.286x |
| fen_write_native_ep | 0.989x | 0.978x | 1.617x |
| snapshot | 0.971x | 0.832x | 0.969x |
| san_parse_all | 0.999x | 0.987x | 2.085x |
| san_render_native | 0.998x | 0.987x | 3.855x |
| apply_children_full | 1.038x | 0.986x | 1.478x |
| make_unmake_full | 0.991x | 0.930x | 1.387x |
| startpos | 0.999x | 0.999x | 1.252x |
| chess960_materialize | 1.022x | 1.019x | 1.337x |
| chess960_perft_d3 | 0.998x | 0.989x | 1.526x |
| polyglot_key | 1.075x | 0.996x | 29.481x |
| polyglot_recompute | 1.004x | 1.001x | 1.020x |
| repetition_key | 1.045x | 1.028x | 21.866x |
| corpus_count | 0.970x | 0.920x | 1.479x |
| corpus_materialize | 1.012x | 1.008x | 1.209x |
| corpus_uci_sorted | 1.013x | 1.002x | 1.438x |
| replay_trusted_native | 0.919x | 0.875x | 0.780x |
| replay_checked_native | 0.983x | 0.968x | 1.078x |

Cold first legal move list: 0.015 ms -> 0.016 ms (0.91x).
