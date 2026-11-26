| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.150x | 0.837x | 1.242x |
| materialize | 1.065x | 0.815x | 0.949x |
| perft_bulk_d4 | 1.129x | 0.947x | 1.341x |
| perft_full_d3 | 1.122x | 1.032x | 1.280x |
| fen_parse | 0.981x | 0.828x | 1.060x |
| fen_write_native_ep | 1.180x | 1.039x | 1.779x |
| snapshot | 1.187x | 0.680x | 1.106x |
| san_parse_all | 1.115x | 0.896x | 1.731x |
| san_render_native | 1.250x | 1.050x | 2.743x |
| apply_children_full | 1.174x | 1.032x | 1.013x |
| make_unmake_full | 1.245x | 0.916x | 1.278x |
| startpos | 1.006x | 1.006x | 1.110x |
| chess960_materialize | 0.990x | 0.942x | 1.338x |
| chess960_perft_d3 | 1.034x | 0.900x | 1.268x |
| polyglot_key | 1.027x | 0.982x | 42.262x |
| polyglot_recompute | 1.178x | 0.987x | 1.018x |
| repetition_key | 1.168x | 1.141x | 25.049x |
| corpus_count | 0.976x | 0.777x | 1.107x |
| corpus_materialize | 1.251x | 1.247x | 1.132x |
| corpus_uci_sorted | 1.178x | 1.125x | 1.540x |
| replay_trusted_native | 1.096x | 0.992x | 0.551x |
| replay_checked_native | 1.296x | 1.275x | 1.125x |

Cold first legal move list: 0.012 ms -> 0.014 ms (0.83x).
