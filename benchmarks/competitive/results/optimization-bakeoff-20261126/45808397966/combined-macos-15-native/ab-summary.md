| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.144x | 0.901x | 1.425x |
| materialize | 1.179x | 0.861x | 1.095x |
| perft_bulk_d4 | 0.926x | 0.717x | 1.064x |
| perft_full_d3 | 1.093x | 0.838x | 1.364x |
| fen_parse | 0.948x | 0.758x | 1.099x |
| fen_write_native_ep | 1.165x | 0.869x | 1.831x |
| snapshot | 1.121x | 0.950x | 0.942x |
| san_parse_all | 0.968x | 0.828x | 1.768x |
| san_render_native | 1.136x | 0.809x | 2.436x |
| apply_children_full | 1.052x | 0.690x | 1.018x |
| make_unmake_full | 1.136x | 0.853x | 1.247x |
| startpos | 1.535x | 1.535x | 1.238x |
| chess960_materialize | 1.267x | 1.182x | 1.641x |
| chess960_perft_d3 | 1.036x | 0.997x | 1.426x |
| polyglot_key | 1.021x | 0.922x | 39.397x |
| polyglot_recompute | 0.987x | 0.905x | 0.929x |
| repetition_key | 1.143x | 0.909x | 19.525x |
| corpus_count | 0.732x | 0.717x | 1.123x |
| corpus_materialize | 1.155x | 1.098x | 1.369x |
| corpus_uci_sorted | 0.960x | 0.897x | 1.239x |
| replay_trusted_native | 0.934x | 0.893x | 0.579x |
| replay_checked_native | 0.983x | 0.945x | 1.063x |

Cold first legal move list: 0.012 ms -> 0.007 ms (1.78x).
