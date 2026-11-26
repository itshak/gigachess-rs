| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.956x | 1.432x |
| materialize | 1.013x | 0.934x | 1.523x |
| perft_bulk_d4 | 0.833x | 0.788x | 0.983x |
| perft_full_d3 | 0.716x | 0.577x | 1.035x |
| fen_parse | 0.912x | 0.809x | 0.957x |
| fen_write_native_ep | 0.951x | 0.894x | 2.006x |
| snapshot | 1.221x | 1.132x | 1.320x |
| san_parse_all | 0.954x | 0.915x | 1.859x |
| san_render_native | 0.885x | 0.804x | 1.977x |
| apply_children_full | 0.896x | 0.833x | 0.948x |
| make_unmake_full | 0.694x | 0.619x | 0.758x |
| startpos | 0.843x | 0.843x | 1.148x |
| chess960_materialize | 0.920x | 0.850x | 1.267x |
| chess960_perft_d3 | 0.768x | 0.763x | 1.080x |
| polyglot_key | 0.947x | 0.886x | 39.458x |
| polyglot_recompute | 0.969x | 0.941x | 1.026x |
| repetition_key | 0.944x | 0.797x | 18.702x |
| corpus_count | 0.996x | 0.986x | 1.349x |
| corpus_materialize | 1.025x | 1.024x | 1.100x |
| corpus_uci_sorted | 0.996x | 0.956x | 1.543x |
| replay_trusted_native | 0.952x | 0.834x | 0.685x |
| replay_checked_native | 0.708x | 0.625x | 0.792x |

Cold first legal move list: 0.010 ms -> 0.010 ms (0.91x).
