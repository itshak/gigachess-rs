| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.061x | 0.763x | 1.638x |
| materialize | 0.988x | 0.781x | 1.024x |
| perft_bulk_d4 | 1.082x | 0.912x | 1.278x |
| perft_full_d3 | 1.015x | 0.839x | 1.416x |
| fen_parse | 0.966x | 0.837x | 1.233x |
| fen_write_native_ep | 0.998x | 0.738x | 1.643x |
| snapshot | 1.060x | 0.655x | 0.860x |
| san_parse_all | 0.955x | 0.871x | 1.798x |
| san_render_native | 0.903x | 0.681x | 2.790x |
| apply_children_full | 1.062x | 0.956x | 1.184x |
| make_unmake_full | 0.944x | 0.883x | 1.264x |
| startpos | 0.723x | 0.723x | 0.932x |
| chess960_materialize | 0.878x | 0.840x | 1.751x |
| chess960_perft_d3 | 0.899x | 0.839x | 1.458x |
| polyglot_key | 0.923x | 0.802x | 35.271x |
| polyglot_recompute | 0.889x | 0.853x | 1.113x |
| repetition_key | 1.005x | 0.897x | 23.335x |
| corpus_count | 1.088x | 0.966x | 1.482x |
| corpus_materialize | 0.922x | 0.880x | 1.290x |
| corpus_uci_sorted | 0.879x | 0.717x | 1.334x |
| replay_trusted_native | 0.896x | 0.856x | 0.645x |
| replay_checked_native | 0.773x | 0.762x | 1.113x |

Cold first legal move list: 0.004 ms -> 0.006 ms (0.62x).
