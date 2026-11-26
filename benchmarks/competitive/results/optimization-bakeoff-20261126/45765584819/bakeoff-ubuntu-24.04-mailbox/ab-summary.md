| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.990x | 0.982x | 1.190x |
| materialize | 0.995x | 0.977x | 1.367x |
| perft_bulk_d4 | 1.044x | 1.023x | 0.925x |
| perft_full_d3 | 1.010x | 0.924x | 0.628x |
| fen_parse | 1.007x | 0.952x | 1.029x |
| fen_write_native_ep | 1.009x | 0.993x | 1.202x |
| snapshot | 0.708x | 0.705x | 0.692x |
| san_parse_all | 0.991x | 0.978x | 0.913x |
| san_render_native | 0.988x | 0.969x | 1.049x |
| apply_children_full | 0.904x | 0.887x | 0.588x |
| make_unmake_full | 0.970x | 0.921x | 0.732x |
| startpos | 0.834x | 0.834x | 0.527x |
| chess960_materialize | 0.996x | 0.995x | 1.210x |
| chess960_perft_d3 | 1.032x | 1.017x | 0.985x |
| polyglot_key | 0.990x | 0.972x | 33.751x |
| polyglot_recompute | 0.969x | 0.968x | 0.957x |
| repetition_key | 0.783x | 0.769x | 12.532x |
| corpus_count | 0.985x | 0.975x | 1.137x |
| corpus_materialize | 0.993x | 0.987x | 1.061x |
| corpus_uci_sorted | 1.015x | 1.013x | 0.633x |
| replay_trusted_native | 0.983x | 0.828x | 0.508x |
| replay_checked_native | 0.953x | 0.878x | 0.726x |

Cold first legal move list: 138.541 ms → 138.099 ms (1.00x).
