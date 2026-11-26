| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.963x | 1.248x |
| materialize | 0.918x | 0.850x | 1.493x |
| perft_bulk_d4 | 0.914x | 0.850x | 1.060x |
| perft_full_d3 | 0.965x | 0.911x | 1.494x |
| fen_parse | 1.059x | 1.005x | 1.317x |
| fen_write_native_ep | 0.955x | 0.896x | 1.902x |
| snapshot | 1.009x | 0.938x | 0.987x |
| san_parse_all | 0.924x | 0.879x | 1.671x |
| san_render_native | 1.321x | 1.133x | 2.188x |
| apply_children_full | 1.044x | 0.997x | 1.293x |
| make_unmake_full | 1.020x | 0.862x | 1.215x |
| startpos | 1.059x | 1.059x | 1.355x |
| chess960_materialize | 0.920x | 0.871x | 1.229x |
| chess960_perft_d3 | 1.023x | 0.960x | 1.363x |
| polyglot_key | 0.979x | 0.964x | 41.276x |
| polyglot_recompute | 1.004x | 0.997x | 1.037x |
| repetition_key | 0.779x | 0.654x | 21.297x |
| corpus_count | 0.775x | 0.695x | 0.925x |
| corpus_materialize | 0.921x | 0.907x | 1.070x |
| corpus_uci_sorted | 0.983x | 0.981x | 1.556x |
| replay_trusted_native | 1.066x | 1.054x | 0.859x |
| replay_checked_native | 0.970x | 0.960x | 1.057x |

Cold first legal move list: 0.010 ms -> 0.006 ms (1.59x).
