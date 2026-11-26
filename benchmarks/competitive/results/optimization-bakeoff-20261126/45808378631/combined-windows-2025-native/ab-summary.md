| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.035x | 0.956x | 1.318x |
| materialize | 1.032x | 0.974x | 1.172x |
| perft_bulk_d4 | 0.961x | 0.892x | 1.220x |
| perft_full_d3 | 1.041x | 0.927x | 1.269x |
| fen_parse | 1.101x | 1.050x | 1.348x |
| fen_write_native_ep | 1.023x | 0.980x | 1.572x |
| snapshot | 3.641x | 3.040x | 0.968x |
| san_parse_all | 1.055x | 0.990x | 2.101x |
| san_render_native | 1.051x | 0.977x | 3.717x |
| apply_children_full | 1.056x | 1.027x | 1.280x |
| make_unmake_full | 1.015x | 0.984x | 1.299x |
| startpos | 1.176x | 1.176x | 1.249x |
| chess960_materialize | 1.090x | 1.024x | 1.248x |
| chess960_perft_d3 | 1.054x | 0.979x | 1.435x |
| polyglot_key | 0.978x | 0.960x | 25.996x |
| polyglot_recompute | 1.042x | 1.029x | 0.997x |
| repetition_key | 1.086x | 1.034x | 19.308x |
| corpus_count | 1.024x | 0.943x | 1.221x |
| corpus_materialize | 0.967x | 0.967x | 1.091x |
| corpus_uci_sorted | 1.098x | 1.072x | 1.389x |
| replay_trusted_native | 1.165x | 0.999x | 0.721x |
| replay_checked_native | 1.148x | 1.111x | 0.988x |

Cold first legal move list: 0.053 ms -> 0.020 ms (2.60x).
