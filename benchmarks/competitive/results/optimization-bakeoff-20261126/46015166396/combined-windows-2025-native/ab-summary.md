| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.967x | 1.570x |
| materialize | 1.009x | 0.989x | 1.159x |
| perft_bulk_d4 | 1.020x | 0.959x | 1.456x |
| perft_full_d3 | 1.300x | 1.050x | 1.748x |
| fen_parse | 0.993x | 0.871x | 1.335x |
| fen_write_native_ep | 0.910x | 0.834x | 1.479x |
| snapshot | 1.031x | 0.994x | 1.174x |
| san_parse_all | 0.986x | 0.945x | 1.968x |
| san_render_native | 1.025x | 0.997x | 3.388x |
| apply_children_full | 0.994x | 0.980x | 1.369x |
| make_unmake_full | 1.002x | 0.942x | 1.309x |
| startpos | 0.836x | 0.836x | 0.988x |
| chess960_materialize | 0.987x | 0.963x | 1.380x |
| chess960_perft_d3 | 0.999x | 0.998x | 1.698x |
| polyglot_key | 1.007x | 0.992x | 22.696x |
| polyglot_recompute | 1.229x | 1.067x | 1.275x |
| repetition_key | 0.984x | 0.972x | 19.528x |
| corpus_count | 1.057x | 1.028x | 1.432x |
| corpus_materialize | 0.997x | 0.994x | 1.201x |
| corpus_uci_sorted | 0.957x | 0.923x | 1.559x |
| replay_trusted_native | 1.177x | 1.006x | 1.197x |
| replay_checked_native | 0.990x | 0.984x | 1.176x |

Cold first legal move list: 0.017 ms -> 0.016 ms (1.08x).
