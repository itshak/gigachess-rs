| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.878x | 0.696x | 1.308x |
| materialize | 0.952x | 0.689x | 0.935x |
| perft_bulk_d4 | 1.029x | 0.941x | 1.078x |
| perft_full_d3 | 1.164x | 0.821x | 1.358x |
| fen_parse | 1.061x | 0.901x | 1.244x |
| fen_write_native_ep | 1.084x | 0.876x | 1.818x |
| snapshot | 1.016x | 0.741x | 0.960x |
| san_parse_all | 1.030x | 0.790x | 1.794x |
| san_render_native | 1.100x | 1.000x | 3.001x |
| apply_children_full | 0.991x | 0.779x | 1.070x |
| make_unmake_full | 0.926x | 0.790x | 1.374x |
| startpos | 1.787x | 1.787x | 1.114x |
| chess960_materialize | 1.006x | 0.956x | 1.432x |
| chess960_perft_d3 | 1.292x | 1.282x | 1.189x |
| polyglot_key | 0.885x | 0.857x | 39.574x |
| polyglot_recompute | 0.955x | 0.947x | 1.088x |
| repetition_key | 0.981x | 0.907x | 19.094x |
| corpus_count | 0.875x | 0.733x | 0.929x |
| corpus_materialize | 0.989x | 0.924x | 1.119x |
| corpus_uci_sorted | 0.793x | 0.757x | 1.347x |
| replay_trusted_native | 0.870x | 0.804x | 0.658x |
| replay_checked_native | 0.872x | 0.840x | 1.274x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.12x).
