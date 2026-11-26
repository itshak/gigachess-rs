| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.892x | 0.858x | 1.328x |
| materialize | 0.948x | 0.908x | 1.148x |
| perft_bulk_d4 | 0.829x | 0.788x | 1.105x |
| perft_full_d3 | 0.699x | 0.632x | 0.910x |
| fen_parse | 0.915x | 0.882x | 1.337x |
| fen_write_native_ep | 0.945x | 0.890x | 1.433x |
| snapshot | 0.927x | 0.864x | 1.356x |
| san_parse_all | 0.915x | 0.901x | 1.783x |
| san_render_native | 0.873x | 0.835x | 3.565x |
| apply_children_full | 0.813x | 0.758x | 1.262x |
| make_unmake_full | 0.872x | 0.765x | 1.185x |
| startpos | 0.898x | 0.898x | 0.907x |
| chess960_materialize | 0.818x | 0.781x | 1.312x |
| chess960_perft_d3 | 0.608x | 0.449x | 1.028x |
| polyglot_key | 0.937x | 0.851x | 37.330x |
| polyglot_recompute | 0.950x | 0.875x | 0.944x |
| repetition_key | 0.968x | 0.928x | 26.097x |
| corpus_count | 0.936x | 0.935x | 1.137x |
| corpus_materialize | 1.021x | 1.020x | 1.138x |
| corpus_uci_sorted | 1.071x | 1.064x | 1.471x |
| replay_trusted_native | 1.013x | 0.993x | 0.783x |
| replay_checked_native | 0.877x | 0.826x | 0.996x |

Cold first legal move list: 0.015 ms -> 0.018 ms (0.81x).
