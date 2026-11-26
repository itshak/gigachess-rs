| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.855x | 0.681x | 1.422x |
| materialize | 0.840x | 0.703x | 1.040x |
| perft_bulk_d4 | 0.873x | 0.729x | 1.204x |
| perft_full_d3 | 0.880x | 0.708x | 1.240x |
| fen_parse | 0.903x | 0.735x | 1.238x |
| fen_write_native_ep | 0.864x | 0.780x | 1.768x |
| snapshot | 0.803x | 0.531x | 0.908x |
| san_parse_all | 0.943x | 0.881x | 1.743x |
| san_render_native | 0.964x | 0.789x | 2.882x |
| apply_children_full | 0.909x | 0.842x | 1.086x |
| make_unmake_full | 1.044x | 0.835x | 1.386x |
| startpos | 0.852x | 0.852x | 0.956x |
| chess960_materialize | 1.064x | 1.054x | 1.460x |
| chess960_perft_d3 | 0.833x | 0.816x | 1.413x |
| polyglot_key | 1.197x | 1.158x | 40.317x |
| polyglot_recompute | 1.019x | 0.960x | 1.001x |
| repetition_key | 0.916x | 0.828x | 19.194x |
| corpus_count | 1.166x | 1.108x | 1.438x |
| corpus_materialize | 1.237x | 1.095x | 1.177x |
| corpus_uci_sorted | 1.136x | 1.048x | 1.504x |
| replay_trusted_native | 1.175x | 0.927x | 0.619x |
| replay_checked_native | 1.186x | 1.167x | 1.233x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.20x).
