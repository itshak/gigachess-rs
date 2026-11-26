| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.944x | 0.644x | 1.299x |
| materialize | 1.009x | 0.808x | 1.161x |
| perft_bulk_d4 | 0.991x | 0.763x | 1.099x |
| perft_full_d3 | 0.943x | 0.857x | 1.256x |
| fen_parse | 0.971x | 0.704x | 1.183x |
| fen_write_native_ep | 1.044x | 0.835x | 1.848x |
| snapshot | 0.933x | 0.658x | 0.910x |
| san_parse_all | 1.001x | 0.847x | 1.732x |
| san_render_native | 1.079x | 0.969x | 2.692x |
| apply_children_full | 0.934x | 0.809x | 1.076x |
| make_unmake_full | 1.069x | 0.881x | 1.293x |
| startpos | 1.393x | 1.393x | 1.774x |
| chess960_materialize | 0.802x | 0.731x | 1.236x |
| chess960_perft_d3 | 0.945x | 0.870x | 1.264x |
| polyglot_key | 0.836x | 0.833x | 32.912x |
| polyglot_recompute | 1.040x | 1.030x | 0.966x |
| repetition_key | 1.116x | 1.018x | 24.572x |
| corpus_count | 0.773x | 0.686x | 0.959x |
| corpus_materialize | 0.885x | 0.747x | 1.069x |
| corpus_uci_sorted | 0.885x | 0.810x | 1.345x |
| replay_trusted_native | 1.140x | 1.116x | 0.699x |
| replay_checked_native | 0.925x | 0.924x | 1.159x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.10x).
