| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.990x | 0.968x | 1.465x |
| materialize | 1.037x | 0.996x | 1.533x |
| perft_bulk_d4 | 0.838x | 0.773x | 1.062x |
| perft_full_d3 | 0.675x | 0.573x | 1.034x |
| fen_parse | 0.926x | 0.867x | 0.994x |
| fen_write_native_ep | 0.998x | 0.981x | 1.902x |
| snapshot | 1.583x | 1.580x | 1.324x |
| san_parse_all | 1.146x | 1.062x | 1.922x |
| san_render_native | 0.910x | 0.854x | 1.927x |
| apply_children_full | 0.906x | 0.849x | 0.958x |
| make_unmake_full | 0.663x | 0.596x | 0.757x |
| startpos | 1.200x | 1.200x | 1.264x |
| chess960_materialize | 0.923x | 0.873x | 1.347x |
| chess960_perft_d3 | 0.803x | 0.786x | 1.105x |
| polyglot_key | 1.071x | 0.998x | 41.086x |
| polyglot_recompute | 0.991x | 0.986x | 1.036x |
| repetition_key | 1.095x | 0.991x | 21.019x |
| corpus_count | 1.017x | 1.008x | 1.419x |
| corpus_materialize | 1.004x | 0.997x | 1.123x |
| corpus_uci_sorted | 0.986x | 0.979x | 1.547x |
| replay_trusted_native | 0.942x | 0.826x | 0.662x |
| replay_checked_native | 0.727x | 0.608x | 0.771x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.03x).
