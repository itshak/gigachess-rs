| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.987x | 1.426x |
| materialize | 1.015x | 1.002x | 1.324x |
| perft_bulk_d4 | 0.998x | 0.978x | 1.223x |
| perft_full_d3 | 0.990x | 0.970x | 1.591x |
| fen_parse | 1.070x | 1.028x | 1.218x |
| fen_write_native_ep | 1.011x | 0.972x | 1.248x |
| snapshot | 2.275x | 2.003x | 1.113x |
| san_parse_all | 1.027x | 0.966x | 1.767x |
| san_render_native | 0.995x | 0.987x | 4.157x |
| apply_children_full | 1.037x | 0.997x | 1.063x |
| make_unmake_full | 0.984x | 0.967x | 1.091x |
| startpos | 1.397x | 1.397x | 1.120x |
| chess960_materialize | 1.003x | 0.987x | 1.305x |
| chess960_perft_d3 | 1.018x | 1.009x | 1.382x |
| polyglot_key | 1.024x | 1.006x | 42.423x |
| polyglot_recompute | 1.023x | 1.011x | 1.028x |
| repetition_key | 1.235x | 1.050x | 29.933x |
| corpus_count | 1.119x | 1.047x | 1.332x |
| corpus_materialize | 1.015x | 0.987x | 1.094x |
| corpus_uci_sorted | 1.040x | 1.008x | 1.574x |
| replay_trusted_native | 1.003x | 0.981x | 0.697x |
| replay_checked_native | 1.042x | 1.015x | 1.097x |

Cold first legal move list: 0.033 ms -> 0.044 ms (0.74x).
