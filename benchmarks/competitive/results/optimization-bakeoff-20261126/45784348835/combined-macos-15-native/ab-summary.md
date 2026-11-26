| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.917x | 0.560x | 1.295x |
| materialize | 1.098x | 0.779x | 1.111x |
| perft_bulk_d4 | 0.813x | 0.571x | 1.150x |
| perft_full_d3 | 0.911x | 0.750x | 1.321x |
| fen_parse | 0.876x | 0.685x | 1.175x |
| fen_write_native_ep | 0.852x | 0.607x | 1.640x |
| snapshot | 1.006x | 0.631x | 0.893x |
| san_parse_all | 0.931x | 0.801x | 2.095x |
| san_render_native | 0.871x | 0.776x | 2.719x |
| apply_children_full | 0.937x | 0.796x | 0.918x |
| make_unmake_full | 0.927x | 0.725x | 1.179x |
| startpos | 0.562x | 0.562x | 1.016x |
| chess960_materialize | 0.850x | 0.833x | 1.311x |
| chess960_perft_d3 | 0.791x | 0.788x | 1.154x |
| polyglot_key | 1.236x | 1.157x | 46.000x |
| polyglot_recompute | 0.940x | 0.925x | 1.273x |
| repetition_key | 1.056x | 1.049x | 19.756x |
| corpus_count | 1.071x | 0.867x | 1.589x |
| corpus_materialize | 1.207x | 1.206x | 1.124x |
| corpus_uci_sorted | 1.120x | 1.109x | 1.493x |
| replay_trusted_native | 0.902x | 0.684x | 0.569x |
| replay_checked_native | 1.085x | 1.054x | 1.049x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.01x).
