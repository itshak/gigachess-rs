| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.819x | 1.224x |
| materialize | 0.996x | 0.786x | 0.994x |
| perft_bulk_d4 | 1.103x | 0.919x | 1.325x |
| perft_full_d3 | 1.097x | 0.949x | 1.334x |
| fen_parse | 0.955x | 0.834x | 1.219x |
| fen_write_native_ep | 0.954x | 0.862x | 1.630x |
| snapshot | 0.926x | 0.803x | 1.000x |
| san_parse_all | 0.924x | 0.663x | 1.925x |
| san_render_native | 0.881x | 0.728x | 2.534x |
| apply_children_full | 0.969x | 0.694x | 0.936x |
| make_unmake_full | 1.076x | 0.897x | 1.137x |
| startpos | 0.870x | 0.870x | 1.228x |
| chess960_materialize | 1.067x | 1.014x | 1.462x |
| chess960_perft_d3 | 0.991x | 0.963x | 1.562x |
| polyglot_key | 1.007x | 0.901x | 38.810x |
| polyglot_recompute | 0.719x | 0.614x | 0.892x |
| repetition_key | 1.051x | 0.949x | 19.241x |
| corpus_count | 0.908x | 0.724x | 1.295x |
| corpus_materialize | 0.853x | 0.829x | 1.114x |
| corpus_uci_sorted | 1.081x | 1.032x | 1.752x |
| replay_trusted_native | 0.752x | 0.748x | 0.534x |
| replay_checked_native | 0.926x | 0.883x | 0.936x |

Cold first legal move list: 0.017 ms -> 0.012 ms (1.34x).
