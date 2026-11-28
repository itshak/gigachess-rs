| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.045x | 1.011x | 1.623x |
| materialize | 1.016x | 0.991x | 1.657x |
| perft_bulk_d4 | 1.045x | 1.018x | 1.252x |
| perft_full_d3 | 1.182x | 1.031x | 1.597x |
| fen_parse | 1.013x | 0.972x | 1.154x |
| fen_write_native_ep | 1.000x | 0.987x | 2.151x |
| snapshot | 0.999x | 0.977x | 1.154x |
| san_parse_all | 1.107x | 1.097x | 1.850x |
| san_render_native | 0.997x | 0.988x | 2.176x |
| apply_children_full | 1.008x | 0.994x | 1.343x |
| make_unmake_full | 0.950x | 0.911x | 1.141x |
| startpos | 0.985x | 0.985x | 1.115x |
| chess960_materialize | 0.947x | 0.935x | 1.452x |
| chess960_perft_d3 | 1.021x | 1.009x | 1.504x |
| polyglot_key | 1.007x | 1.000x | 39.347x |
| polyglot_recompute | 1.675x | 1.356x | 1.612x |
| repetition_key | 1.215x | 1.043x | 23.576x |
| corpus_count | 1.182x | 1.141x | 1.231x |
| corpus_materialize | 1.012x | 1.006x | 1.100x |
| corpus_uci_sorted | 0.997x | 0.991x | 1.568x |
| replay_trusted_native | 0.896x | 0.853x | 0.789x |
| replay_checked_native | 0.922x | 0.901x | 1.072x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.07x).
