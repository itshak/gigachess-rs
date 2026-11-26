| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.995x | 1.466x |
| materialize | 0.981x | 0.959x | 1.529x |
| perft_bulk_d4 | 1.006x | 0.998x | 1.267x |
| perft_full_d3 | 0.992x | 0.973x | 1.479x |
| fen_parse | 1.066x | 1.028x | 1.117x |
| fen_write_native_ep | 1.590x | 1.522x | 1.908x |
| snapshot | 0.773x | 0.764x | 0.998x |
| san_parse_all | 1.044x | 1.028x | 1.955x |
| san_render_native | 0.998x | 0.983x | 2.094x |
| apply_children_full | 0.773x | 0.751x | 0.866x |
| make_unmake_full | 1.004x | 0.966x | 1.140x |
| startpos | 0.726x | 0.726x | 1.081x |
| chess960_materialize | 1.072x | 1.070x | 1.446x |
| chess960_perft_d3 | 0.996x | 0.994x | 1.572x |
| polyglot_key | 0.987x | 0.974x | 39.392x |
| polyglot_recompute | 0.956x | 0.941x | 0.969x |
| repetition_key | 0.926x | 0.862x | 20.902x |
| corpus_count | 1.000x | 0.998x | 1.406x |
| corpus_materialize | 1.007x | 1.007x | 1.111x |
| corpus_uci_sorted | 1.012x | 1.009x | 1.571x |
| replay_trusted_native | 0.990x | 0.985x | 0.717x |
| replay_checked_native | 1.003x | 0.999x | 1.065x |

Cold first legal move list: 0.008 ms -> 0.010 ms (0.81x).
