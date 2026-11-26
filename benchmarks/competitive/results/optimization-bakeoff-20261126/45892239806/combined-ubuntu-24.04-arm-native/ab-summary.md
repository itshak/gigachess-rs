| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.908x | 0.849x | 1.407x |
| materialize | 0.972x | 0.925x | 1.285x |
| perft_bulk_d4 | 0.935x | 0.910x | 1.302x |
| perft_full_d3 | 0.803x | 0.778x | 1.069x |
| fen_parse | 0.975x | 0.962x | 1.194x |
| fen_write_native_ep | 0.947x | 0.913x | 1.507x |
| snapshot | 1.237x | 1.175x | 1.033x |
| san_parse_all | 0.981x | 0.975x | 1.805x |
| san_render_native | 0.954x | 0.938x | 2.543x |
| apply_children_full | 0.963x | 0.934x | 1.111x |
| make_unmake_full | 0.932x | 0.904x | 1.454x |
| startpos | 1.137x | 1.137x | 1.347x |
| chess960_materialize | 0.964x | 0.957x | 1.435x |
| chess960_perft_d3 | 0.938x | 0.931x | 1.406x |
| polyglot_key | 1.042x | 0.950x | 26.658x |
| polyglot_recompute | 1.000x | 0.998x | 1.090x |
| repetition_key | 1.147x | 1.071x | 21.880x |
| corpus_count | 0.936x | 0.925x | 1.272x |
| corpus_materialize | 0.964x | 0.964x | 1.180x |
| corpus_uci_sorted | 1.022x | 1.021x | 1.498x |
| replay_trusted_native | 0.969x | 0.963x | 0.613x |
| replay_checked_native | 0.892x | 0.887x | 1.004x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.10x).
