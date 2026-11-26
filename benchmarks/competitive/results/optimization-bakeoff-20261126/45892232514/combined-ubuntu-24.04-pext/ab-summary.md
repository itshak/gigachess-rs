| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.954x | 0.919x | 1.407x |
| materialize | 0.970x | 0.935x | 1.235x |
| perft_bulk_d4 | 0.866x | 0.838x | 1.156x |
| perft_full_d3 | 0.875x | 0.844x | 1.152x |
| fen_parse | 0.825x | 0.768x | 1.032x |
| fen_write_native_ep | 0.923x | 0.876x | 1.604x |
| snapshot | 0.998x | 0.997x | 1.249x |
| san_parse_all | 0.878x | 0.855x | 1.885x |
| san_render_native | 0.988x | 0.906x | 2.178x |
| apply_children_full | 0.890x | 0.849x | 1.243x |
| make_unmake_full | 0.861x | 0.779x | 1.100x |
| startpos | 1.503x | 1.503x | 1.881x |
| chess960_materialize | 0.937x | 0.902x | 1.383x |
| chess960_perft_d3 | 0.864x | 0.812x | 1.307x |
| polyglot_key | 1.119x | 1.048x | 31.094x |
| polyglot_recompute | 1.002x | 0.992x | 1.039x |
| repetition_key | 1.090x | 1.059x | 23.357x |
| corpus_count | 0.966x | 0.966x | 1.365x |
| corpus_materialize | 0.984x | 0.983x | 1.188x |
| corpus_uci_sorted | 1.043x | 1.037x | 1.816x |
| replay_trusted_native | 0.907x | 0.876x | 1.056x |
| replay_checked_native | 0.844x | 0.783x | 0.941x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.95x).
