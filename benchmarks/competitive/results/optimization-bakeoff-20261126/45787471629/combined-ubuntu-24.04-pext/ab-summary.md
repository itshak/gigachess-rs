| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.991x | 1.434x |
| materialize | 0.964x | 0.946x | 1.188x |
| perft_bulk_d4 | 0.994x | 0.970x | 1.327x |
| perft_full_d3 | 0.743x | 0.700x | 0.735x |
| fen_parse | 0.955x | 0.930x | 1.104x |
| fen_write_native_ep | 0.994x | 0.982x | 1.809x |
| snapshot | 1.001x | 0.987x | 1.106x |
| san_parse_all | 1.002x | 1.000x | 2.186x |
| san_render_native | 1.003x | 0.995x | 2.146x |
| apply_children_full | 1.001x | 0.990x | 1.054x |
| make_unmake_full | 0.992x | 0.970x | 1.181x |
| startpos | 0.999x | 0.999x | 1.183x |
| chess960_materialize | 0.934x | 0.928x | 1.383x |
| chess960_perft_d3 | 1.012x | 1.004x | 1.528x |
| polyglot_key | 1.003x | 0.999x | 20.557x |
| polyglot_recompute | 1.003x | 1.001x | 0.986x |
| repetition_key | 1.007x | 1.002x | 19.308x |
| corpus_count | 1.002x | 1.000x | 1.464x |
| corpus_materialize | 0.999x | 0.999x | 1.273x |
| corpus_uci_sorted | 1.017x | 1.013x | 1.670x |
| replay_trusted_native | 1.001x | 0.987x | 0.755x |
| replay_checked_native | 0.966x | 0.898x | 0.906x |

Cold first legal move list: 0.011 ms -> 0.011 ms (0.97x).
