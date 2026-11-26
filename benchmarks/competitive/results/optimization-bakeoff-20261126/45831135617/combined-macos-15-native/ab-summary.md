| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.876x | 0.757x | 1.370x |
| materialize | 0.854x | 0.704x | 1.051x |
| perft_bulk_d4 | 0.874x | 0.713x | 1.058x |
| perft_full_d3 | 0.857x | 0.682x | 1.267x |
| fen_parse | 0.970x | 0.780x | 1.266x |
| fen_write_native_ep | 1.012x | 0.887x | 1.881x |
| snapshot | 0.946x | 0.582x | 0.796x |
| san_parse_all | 0.977x | 0.839x | 1.706x |
| san_render_native | 0.925x | 0.825x | 2.906x |
| apply_children_full | 0.966x | 0.836x | 1.076x |
| make_unmake_full | 0.875x | 0.591x | 1.194x |
| startpos | 2.908x | 2.908x | 2.671x |
| chess960_materialize | 0.774x | 0.766x | 1.440x |
| chess960_perft_d3 | 0.793x | 0.730x | 1.421x |
| polyglot_key | 0.952x | 0.869x | 41.055x |
| polyglot_recompute | 0.849x | 0.771x | 0.982x |
| repetition_key | 0.900x | 0.880x | 20.449x |
| corpus_count | 0.902x | 0.752x | 0.943x |
| corpus_materialize | 1.118x | 0.982x | 1.043x |
| corpus_uci_sorted | 0.891x | 0.891x | 1.325x |
| replay_trusted_native | 1.065x | 0.999x | 0.637x |
| replay_checked_native | 0.946x | 0.848x | 1.059x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.01x).
