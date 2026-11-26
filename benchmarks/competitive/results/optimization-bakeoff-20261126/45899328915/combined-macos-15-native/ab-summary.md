| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.900x | 0.643x | 1.360x |
| materialize | 1.000x | 0.911x | 1.070x |
| perft_bulk_d4 | 1.011x | 0.913x | 1.203x |
| perft_full_d3 | 0.864x | 0.743x | 1.214x |
| fen_parse | 0.982x | 0.951x | 1.212x |
| fen_write_native_ep | 0.921x | 0.758x | 1.643x |
| snapshot | 1.021x | 0.931x | 1.048x |
| san_parse_all | 1.003x | 0.787x | 1.803x |
| san_render_native | 0.984x | 0.892x | 2.854x |
| apply_children_full | 0.458x | 0.366x | 0.517x |
| make_unmake_full | 0.835x | 0.719x | 1.165x |
| startpos | 0.649x | 0.649x | 0.768x |
| chess960_materialize | 0.817x | 0.732x | 1.432x |
| chess960_perft_d3 | 0.761x | 0.745x | 1.328x |
| polyglot_key | 1.002x | 0.957x | 37.471x |
| polyglot_recompute | 0.962x | 0.956x | 1.093x |
| repetition_key | 1.029x | 1.021x | 19.671x |
| corpus_count | 0.975x | 0.900x | 1.258x |
| corpus_materialize | 0.908x | 0.887x | 1.112x |
| corpus_uci_sorted | 0.962x | 0.939x | 1.414x |
| replay_trusted_native | 0.965x | 0.887x | 0.612x |
| replay_checked_native | 0.980x | 0.969x | 1.243x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.17x).
