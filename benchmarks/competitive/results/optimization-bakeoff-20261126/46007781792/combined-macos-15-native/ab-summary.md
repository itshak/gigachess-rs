| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.032x | 0.802x | 1.710x |
| materialize | 0.849x | 0.651x | 1.040x |
| perft_bulk_d4 | 1.033x | 0.810x | 1.309x |
| perft_full_d3 | 0.973x | 0.690x | 1.311x |
| fen_parse | 0.866x | 0.727x | 1.165x |
| fen_write_native_ep | 0.978x | 0.705x | 1.591x |
| snapshot | 0.873x | 0.684x | 0.874x |
| san_parse_all | 0.949x | 0.736x | 1.767x |
| san_render_native | 0.918x | 0.840x | 2.947x |
| apply_children_full | 0.923x | 0.688x | 1.068x |
| make_unmake_full | 0.925x | 0.835x | 1.314x |
| startpos | 1.466x | 1.466x | 1.335x |
| chess960_materialize | 0.826x | 0.729x | 1.261x |
| chess960_perft_d3 | 1.057x | 0.863x | 1.541x |
| polyglot_key | 1.044x | 0.982x | 38.880x |
| polyglot_recompute | 0.946x | 0.913x | 1.045x |
| repetition_key | 0.851x | 0.843x | 18.612x |
| corpus_count | 1.104x | 0.923x | 1.384x |
| corpus_materialize | 1.003x | 0.953x | 1.057x |
| corpus_uci_sorted | 1.053x | 0.971x | 1.324x |
| replay_trusted_native | 1.080x | 1.056x | 0.678x |
| replay_checked_native | 0.989x | 0.936x | 1.284x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.81x).
