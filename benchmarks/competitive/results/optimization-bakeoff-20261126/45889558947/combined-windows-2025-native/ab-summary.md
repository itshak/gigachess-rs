| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.964x | 1.523x |
| materialize | 1.019x | 0.980x | 1.283x |
| perft_bulk_d4 | 0.934x | 0.869x | 1.355x |
| perft_full_d3 | 1.002x | 0.987x | 1.332x |
| fen_parse | 0.986x | 0.916x | 1.217x |
| fen_write_native_ep | 0.983x | 0.973x | 1.574x |
| snapshot | 0.966x | 0.831x | 0.978x |
| san_parse_all | 0.958x | 0.860x | 1.954x |
| san_render_native | 0.982x | 0.946x | 3.788x |
| apply_children_full | 0.989x | 0.929x | 1.406x |
| make_unmake_full | 1.027x | 0.996x | 1.367x |
| startpos | 1.229x | 1.229x | 1.335x |
| chess960_materialize | 0.988x | 0.972x | 1.284x |
| chess960_perft_d3 | 0.913x | 0.912x | 1.481x |
| polyglot_key | 1.002x | 0.998x | 29.584x |
| polyglot_recompute | 0.896x | 0.784x | 0.973x |
| repetition_key | 0.997x | 0.982x | 21.290x |
| corpus_count | 0.922x | 0.851x | 1.326x |
| corpus_materialize | 1.016x | 1.007x | 1.195x |
| corpus_uci_sorted | 0.910x | 0.907x | 1.286x |
| replay_trusted_native | 0.960x | 0.927x | 0.910x |
| replay_checked_native | 0.942x | 0.924x | 1.052x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.03x).
