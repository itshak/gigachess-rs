| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.970x | 0.914x | 1.369x |
| materialize | 1.001x | 0.944x | 1.502x |
| perft_bulk_d4 | 0.837x | 0.750x | 1.015x |
| perft_full_d3 | 0.986x | 0.901x | 1.436x |
| fen_parse | 1.008x | 0.995x | 1.076x |
| fen_write_native_ep | 0.967x | 0.914x | 2.058x |
| snapshot | 1.674x | 1.668x | 1.158x |
| san_parse_all | 0.887x | 0.878x | 1.811x |
| san_render_native | 1.024x | 1.017x | 2.126x |
| apply_children_full | 1.054x | 1.026x | 1.212x |
| make_unmake_full | 0.971x | 0.912x | 1.081x |
| startpos | 1.337x | 1.337x | 1.109x |
| chess960_materialize | 1.018x | 0.982x | 1.473x |
| chess960_perft_d3 | 1.019x | 0.973x | 1.478x |
| polyglot_key | 0.923x | 0.858x | 37.168x |
| polyglot_recompute | 1.036x | 1.022x | 1.041x |
| repetition_key | 0.825x | 0.705x | 16.207x |
| corpus_count | 0.659x | 0.561x | 0.879x |
| corpus_materialize | 0.953x | 0.934x | 1.040x |
| corpus_uci_sorted | 0.988x | 0.981x | 1.558x |
| replay_trusted_native | 1.010x | 1.002x | 0.830x |
| replay_checked_native | 1.018x | 1.005x | 1.174x |

Cold first legal move list: 0.010 ms -> 0.005 ms (1.78x).
