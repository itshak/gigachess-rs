| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.976x | 0.864x | 1.286x |
| materialize | 0.855x | 0.618x | 0.969x |
| perft_bulk_d4 | 0.863x | 0.655x | 1.278x |
| perft_full_d3 | 0.966x | 0.760x | 1.223x |
| fen_parse | 1.142x | 0.962x | 1.346x |
| fen_write_native_ep | 0.949x | 0.833x | 1.238x |
| snapshot | 0.871x | 0.600x | 0.825x |
| san_parse_all | 0.880x | 0.708x | 1.703x |
| san_render_native | 1.003x | 0.859x | 2.447x |
| apply_children_full | 0.945x | 0.734x | 0.896x |
| make_unmake_full | 0.925x | 0.803x | 1.098x |
| startpos | 1.119x | 1.119x | 1.476x |
| chess960_materialize | 1.063x | 0.976x | 1.379x |
| chess960_perft_d3 | 1.185x | 1.045x | 1.441x |
| polyglot_key | 1.274x | 1.184x | 34.522x |
| polyglot_recompute | 1.297x | 1.275x | 1.047x |
| repetition_key | 1.235x | 1.138x | 19.700x |
| corpus_count | 1.150x | 1.076x | 1.457x |
| corpus_materialize | 1.116x | 1.010x | 1.225x |
| corpus_uci_sorted | 1.144x | 1.070x | 1.415x |
| replay_trusted_native | 0.835x | 0.722x | 0.522x |
| replay_checked_native | 0.948x | 0.875x | 0.970x |

Cold first legal move list: 0.024 ms -> 0.015 ms (1.57x).
