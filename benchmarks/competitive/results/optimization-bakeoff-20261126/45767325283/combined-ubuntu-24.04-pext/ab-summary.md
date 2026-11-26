| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.107x | 1.072x | 1.373x |
| materialize | 1.084x | 1.054x | 1.165x |
| perft_bulk_d4 | 1.300x | 1.251x | 1.337x |
| perft_full_d3 | 1.118x | 1.053x | 0.644x |
| fen_parse | 1.056x | 0.969x | 1.192x |
| fen_write_native_ep | 0.959x | 0.949x | 1.149x |
| snapshot | 1.223x | 1.195x | 1.109x |
| san_parse_all | 1.992x | 1.932x | 2.101x |
| san_render_native | 1.946x | 1.499x | 2.245x |
| apply_children_full | 1.288x | 1.254x | 0.921x |
| make_unmake_full | 1.155x | 1.122x | 0.969x |
| startpos | 1.332x | 1.332x | 1.080x |
| chess960_materialize | 1.048x | 0.982x | 1.456x |
| chess960_perft_d3 | 1.382x | 1.280x | 1.434x |
| polyglot_key | 0.891x | 0.794x | 20.510x |
| polyglot_recompute | 0.996x | 0.972x | 1.035x |
| repetition_key | 1.440x | 1.343x | 18.751x |
| corpus_count | 1.053x | 1.042x | 1.406x |
| corpus_materialize | 1.027x | 1.026x | 1.266x |
| corpus_uci_sorted | 1.502x | 1.490x | 0.972x |
| replay_trusted_native | 1.182x | 1.167x | 0.627x |
| replay_checked_native | 1.273x | 1.196x | 0.892x |

Cold first legal move list: 1.034 ms → 0.012 ms (89.31x).
