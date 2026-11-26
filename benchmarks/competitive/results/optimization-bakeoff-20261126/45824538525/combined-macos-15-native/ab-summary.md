| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.950x | 0.749x | 1.276x |
| materialize | 0.989x | 0.885x | 1.074x |
| perft_bulk_d4 | 0.901x | 0.838x | 1.072x |
| perft_full_d3 | 0.925x | 0.765x | 1.293x |
| fen_parse | 0.977x | 0.851x | 1.239x |
| fen_write_native_ep | 0.937x | 0.840x | 1.847x |
| snapshot | 1.035x | 0.701x | 0.980x |
| san_parse_all | 0.897x | 0.741x | 1.700x |
| san_render_native | 1.011x | 0.796x | 2.768x |
| apply_children_full | 0.974x | 0.825x | 1.123x |
| make_unmake_full | 0.943x | 0.826x | 1.321x |
| startpos | 0.813x | 0.813x | 1.008x |
| chess960_materialize | 0.789x | 0.758x | 1.377x |
| chess960_perft_d3 | 0.827x | 0.733x | 1.117x |
| polyglot_key | 0.880x | 0.854x | 42.736x |
| polyglot_recompute | 0.871x | 0.867x | 1.057x |
| repetition_key | 0.830x | 0.810x | 17.971x |
| corpus_count | 0.803x | 0.763x | 0.991x |
| corpus_materialize | 0.858x | 0.798x | 1.179x |
| corpus_uci_sorted | 0.872x | 0.737x | 1.355x |
| replay_trusted_native | 0.841x | 0.803x | 0.650x |
| replay_checked_native | 0.732x | 0.719x | 1.104x |

Cold first legal move list: 0.003 ms -> 0.003 ms (1.35x).
