| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.957x | 0.935x | 1.519x |
| materialize | 1.024x | 1.006x | 1.541x |
| perft_bulk_d4 | 1.019x | 0.952x | 1.207x |
| perft_full_d3 | 1.027x | 0.928x | 1.416x |
| fen_parse | 1.003x | 0.944x | 1.060x |
| fen_write_native_ep | 0.918x | 0.856x | 1.938x |
| snapshot | 0.749x | 0.741x | 0.862x |
| san_parse_all | 0.996x | 0.978x | 1.844x |
| san_render_native | 0.994x | 0.956x | 2.164x |
| apply_children_full | 0.951x | 0.896x | 1.361x |
| make_unmake_full | 0.943x | 0.887x | 1.135x |
| startpos | 0.821x | 0.821x | 0.914x |
| chess960_materialize | 0.892x | 0.868x | 1.410x |
| chess960_perft_d3 | 0.970x | 0.938x | 1.431x |
| polyglot_key | 1.041x | 1.003x | 39.410x |
| polyglot_recompute | 0.953x | 0.952x | 0.951x |
| repetition_key | 0.943x | 0.937x | 21.532x |
| corpus_count | 1.137x | 1.073x | 1.277x |
| corpus_materialize | 1.008x | 1.006x | 1.083x |
| corpus_uci_sorted | 1.024x | 1.019x | 1.566x |
| replay_trusted_native | 0.942x | 0.888x | 0.853x |
| replay_checked_native | 0.888x | 0.877x | 1.044x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.98x).
