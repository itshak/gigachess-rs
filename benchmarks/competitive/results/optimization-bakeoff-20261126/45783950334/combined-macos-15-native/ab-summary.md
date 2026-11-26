| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.914x | 0.652x | 1.513x |
| materialize | 0.873x | 0.634x | 1.057x |
| perft_bulk_d4 | 0.997x | 0.766x | 1.310x |
| perft_full_d3 | 0.869x | 0.700x | 1.196x |
| fen_parse | 0.856x | 0.769x | 1.124x |
| fen_write_native_ep | 0.898x | 0.724x | 1.680x |
| snapshot | 0.940x | 0.729x | 0.888x |
| san_parse_all | 1.033x | 0.841x | 2.024x |
| san_render_native | 0.959x | 0.815x | 2.807x |
| apply_children_full | 0.978x | 0.722x | 0.943x |
| make_unmake_full | 1.000x | 0.907x | 1.202x |
| startpos | 0.901x | 0.901x | 0.906x |
| chess960_materialize | 0.885x | 0.869x | 1.517x |
| chess960_perft_d3 | 0.923x | 0.901x | 1.737x |
| polyglot_key | 1.002x | 0.765x | 39.720x |
| polyglot_recompute | 1.100x | 0.876x | 1.129x |
| repetition_key | 0.816x | 0.676x | 17.291x |
| corpus_count | 0.789x | 0.602x | 1.188x |
| corpus_materialize | 0.893x | 0.823x | 1.183x |
| corpus_uci_sorted | 0.751x | 0.689x | 1.413x |
| replay_trusted_native | 0.996x | 0.937x | 0.570x |
| replay_checked_native | 0.962x | 0.883x | 1.248x |

Cold first legal move list: 0.013 ms -> 0.022 ms (0.61x).
