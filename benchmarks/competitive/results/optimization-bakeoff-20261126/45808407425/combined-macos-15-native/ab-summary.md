| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.906x | 0.735x | 1.434x |
| materialize | 0.910x | 0.710x | 1.015x |
| perft_bulk_d4 | 0.825x | 0.693x | 1.261x |
| perft_full_d3 | 0.874x | 0.812x | 1.177x |
| fen_parse | 0.813x | 0.677x | 1.013x |
| fen_write_native_ep | 0.999x | 0.871x | 1.765x |
| snapshot | 1.099x | 0.748x | 1.026x |
| san_parse_all | 0.897x | 0.737x | 1.822x |
| san_render_native | 0.937x | 0.779x | 3.128x |
| apply_children_full | 0.878x | 0.746x | 0.954x |
| make_unmake_full | 0.856x | 0.777x | 1.199x |
| startpos | 1.683x | 1.683x | 2.123x |
| chess960_materialize | 0.859x | 0.723x | 1.278x |
| chess960_perft_d3 | 0.831x | 0.756x | 1.318x |
| polyglot_key | 1.113x | 1.063x | 43.727x |
| polyglot_recompute | 0.947x | 0.936x | 0.900x |
| repetition_key | 0.959x | 0.815x | 17.401x |
| corpus_count | 0.818x | 0.801x | 1.362x |
| corpus_materialize | 0.844x | 0.805x | 1.167x |
| corpus_uci_sorted | 0.750x | 0.665x | 1.316x |
| replay_trusted_native | 0.697x | 0.606x | 0.541x |
| replay_checked_native | 0.871x | 0.828x | 1.037x |

Cold first legal move list: 0.015 ms -> 0.016 ms (0.93x).
