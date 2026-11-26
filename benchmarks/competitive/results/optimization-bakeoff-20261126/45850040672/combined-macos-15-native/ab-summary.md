| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.060x | 0.786x | 1.550x |
| materialize | 1.110x | 0.905x | 1.099x |
| perft_bulk_d4 | 1.031x | 0.811x | 1.215x |
| perft_full_d3 | 1.048x | 0.899x | 1.407x |
| fen_parse | 1.000x | 0.735x | 1.215x |
| fen_write_native_ep | 1.128x | 0.866x | 1.849x |
| snapshot | 0.992x | 0.634x | 0.899x |
| san_parse_all | 1.090x | 0.770x | 1.881x |
| san_render_native | 1.188x | 1.004x | 3.025x |
| apply_children_full | 1.100x | 0.931x | 1.091x |
| make_unmake_full | 1.071x | 0.942x | 1.303x |
| startpos | 1.761x | 1.761x | 1.327x |
| chess960_materialize | 1.018x | 0.937x | 1.445x |
| chess960_perft_d3 | 1.020x | 1.006x | 1.270x |
| polyglot_key | 1.034x | 1.000x | 37.797x |
| polyglot_recompute | 1.079x | 0.931x | 1.068x |
| repetition_key | 0.969x | 0.908x | 19.823x |
| corpus_count | 1.135x | 1.077x | 1.279x |
| corpus_materialize | 1.001x | 0.902x | 1.120x |
| corpus_uci_sorted | 0.915x | 0.836x | 1.086x |
| replay_trusted_native | 1.186x | 1.040x | 0.709x |
| replay_checked_native | 1.194x | 0.957x | 1.197x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.83x).
