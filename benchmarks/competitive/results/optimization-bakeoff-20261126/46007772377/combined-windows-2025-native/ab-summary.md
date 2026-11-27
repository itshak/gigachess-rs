| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.893x | 1.553x |
| materialize | 1.006x | 0.968x | 1.250x |
| perft_bulk_d4 | 1.011x | 0.912x | 1.384x |
| perft_full_d3 | 1.204x | 1.031x | 1.635x |
| fen_parse | 0.962x | 0.889x | 1.339x |
| fen_write_native_ep | 0.906x | 0.826x | 1.479x |
| snapshot | 1.000x | 0.985x | 1.005x |
| san_parse_all | 0.939x | 0.811x | 2.034x |
| san_render_native | 0.961x | 0.851x | 3.805x |
| apply_children_full | 1.014x | 0.924x | 1.460x |
| make_unmake_full | 0.943x | 0.730x | 1.303x |
| startpos | 1.005x | 1.005x | 1.250x |
| chess960_materialize | 0.998x | 0.985x | 1.344x |
| chess960_perft_d3 | 0.902x | 0.806x | 1.427x |
| polyglot_key | 1.005x | 0.997x | 30.199x |
| polyglot_recompute | 0.926x | 0.883x | 1.043x |
| repetition_key | 0.889x | 0.822x | 21.871x |
| corpus_count | 1.032x | 1.030x | 1.473x |
| corpus_materialize | 0.995x | 0.983x | 1.238x |
| corpus_uci_sorted | 0.974x | 0.941x | 1.391x |
| replay_trusted_native | 1.017x | 0.993x | 0.906x |
| replay_checked_native | 0.947x | 0.921x | 1.031x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.09x).
