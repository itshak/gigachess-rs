| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.993x | 1.380x |
| materialize | 0.998x | 0.989x | 1.485x |
| perft_bulk_d4 | 0.904x | 0.893x | 1.067x |
| perft_full_d3 | 0.853x | 0.829x | 0.749x |
| fen_parse | 0.792x | 0.751x | 0.966x |
| fen_write_native_ep | 1.040x | 1.013x | 1.318x |
| snapshot | 0.603x | 0.599x | 0.782x |
| san_parse_all | 0.892x | 0.876x | 1.586x |
| san_render_native | 0.954x | 0.905x | 2.009x |
| apply_children_full | 0.868x | 0.825x | 0.818x |
| make_unmake_full | 0.812x | 0.761x | 0.757x |
| startpos | 1.090x | 1.090x | 1.687x |
| chess960_materialize | 0.999x | 0.993x | 1.379x |
| chess960_perft_d3 | 0.935x | 0.917x | 1.223x |
| polyglot_key | 0.946x | 0.894x | 37.323x |
| polyglot_recompute | 1.025x | 1.007x | 0.988x |
| repetition_key | 0.878x | 0.801x | 19.266x |
| corpus_count | 0.996x | 0.993x | 1.295x |
| corpus_materialize | 0.995x | 0.988x | 1.104x |
| corpus_uci_sorted | 0.998x | 0.995x | 0.980x |
| replay_trusted_native | 0.889x | 0.866x | 0.543x |
| replay_checked_native | 0.809x | 0.719x | 0.796x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.98x).
