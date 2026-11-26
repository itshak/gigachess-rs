| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.971x | 0.781x | 1.397x |
| materialize | 0.947x | 0.834x | 1.014x |
| perft_bulk_d4 | 0.911x | 0.680x | 1.274x |
| perft_full_d3 | 0.737x | 0.663x | 1.045x |
| fen_parse | 0.988x | 0.875x | 1.298x |
| fen_write_native_ep | 0.871x | 0.712x | 1.204x |
| snapshot | 1.293x | 0.935x | 1.050x |
| san_parse_all | 1.019x | 0.781x | 1.775x |
| san_render_native | 0.875x | 0.721x | 2.913x |
| apply_children_full | 0.809x | 0.624x | 0.897x |
| make_unmake_full | 0.837x | 0.687x | 1.268x |
| startpos | 1.018x | 1.018x | 1.599x |
| chess960_materialize | 0.946x | 0.943x | 1.410x |
| chess960_perft_d3 | 0.887x | 0.770x | 1.713x |
| polyglot_key | 0.902x | 0.801x | 43.501x |
| polyglot_recompute | 0.792x | 0.708x | 0.834x |
| repetition_key | 0.936x | 0.899x | 18.280x |
| corpus_count | 0.938x | 0.884x | 1.290x |
| corpus_materialize | 0.961x | 0.905x | 1.132x |
| corpus_uci_sorted | 0.865x | 0.853x | 1.319x |
| replay_trusted_native | 0.834x | 0.786x | 0.560x |
| replay_checked_native | 0.852x | 0.745x | 1.089x |

Cold first legal move list: 0.014 ms -> 0.018 ms (0.77x).
