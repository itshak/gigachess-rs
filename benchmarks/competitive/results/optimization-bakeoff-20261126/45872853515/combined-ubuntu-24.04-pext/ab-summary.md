| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.969x | 1.549x |
| materialize | 0.995x | 0.986x | 1.595x |
| perft_bulk_d4 | 1.012x | 1.000x | 1.205x |
| perft_full_d3 | 1.231x | 1.059x | 1.665x |
| fen_parse | 0.941x | 0.832x | 1.091x |
| fen_write_native_ep | 1.020x | 0.988x | 2.082x |
| snapshot | 1.000x | 0.998x | 1.330x |
| san_parse_all | 1.017x | 0.991x | 2.004x |
| san_render_native | 1.208x | 1.183x | 2.156x |
| apply_children_full | 1.001x | 0.997x | 1.277x |
| make_unmake_full | 0.997x | 0.989x | 1.197x |
| startpos | 1.116x | 1.116x | 1.245x |
| chess960_materialize | 0.944x | 0.924x | 1.415x |
| chess960_perft_d3 | 0.995x | 0.987x | 1.468x |
| polyglot_key | 0.982x | 0.963x | 38.734x |
| polyglot_recompute | 1.011x | 1.011x | 1.009x |
| repetition_key | 0.931x | 0.865x | 22.322x |
| corpus_count | 0.945x | 0.902x | 1.055x |
| corpus_materialize | 1.010x | 1.003x | 1.092x |
| corpus_uci_sorted | 1.016x | 1.013x | 1.575x |
| replay_trusted_native | 0.965x | 0.904x | 0.854x |
| replay_checked_native | 1.009x | 0.996x | 1.132x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.98x).
