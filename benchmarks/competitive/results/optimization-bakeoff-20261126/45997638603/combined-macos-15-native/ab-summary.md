| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.149x | 0.973x | 1.572x |
| materialize | 1.079x | 0.818x | 1.031x |
| perft_bulk_d4 | 1.082x | 0.879x | 1.279x |
| perft_full_d3 | 1.140x | 0.986x | 1.466x |
| fen_parse | 1.137x | 1.055x | 1.165x |
| fen_write_native_ep | 1.067x | 0.832x | 1.729x |
| snapshot | 0.993x | 0.534x | 0.916x |
| san_parse_all | 1.072x | 0.924x | 1.595x |
| san_render_native | 1.032x | 0.845x | 3.084x |
| apply_children_full | 0.948x | 0.775x | 1.082x |
| make_unmake_full | 1.002x | 0.825x | 1.335x |
| startpos | 1.008x | 1.008x | 1.200x |
| chess960_materialize | 1.007x | 0.987x | 1.712x |
| chess960_perft_d3 | 0.931x | 0.774x | 1.373x |
| polyglot_key | 0.772x | 0.761x | 38.511x |
| polyglot_recompute | 0.836x | 0.710x | 0.870x |
| repetition_key | 0.998x | 0.956x | 20.781x |
| corpus_count | 1.009x | 0.920x | 1.418x |
| corpus_materialize | 0.921x | 0.906x | 1.274x |
| corpus_uci_sorted | 0.834x | 0.804x | 1.406x |
| replay_trusted_native | 0.845x | 0.823x | 0.678x |
| replay_checked_native | 0.853x | 0.814x | 1.094x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.96x).
