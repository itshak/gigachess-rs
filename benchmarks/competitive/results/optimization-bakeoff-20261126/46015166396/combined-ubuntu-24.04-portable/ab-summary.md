| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.056x | 1.002x | 1.358x |
| materialize | 1.000x | 0.984x | 1.750x |
| perft_bulk_d4 | 1.071x | 1.046x | 1.269x |
| perft_full_d3 | 1.161x | 1.035x | 1.555x |
| fen_parse | 0.925x | 0.756x | 1.076x |
| fen_write_native_ep | 0.996x | 0.977x | 1.804x |
| snapshot | 0.999x | 0.989x | 1.002x |
| san_parse_all | 0.996x | 0.983x | 1.892x |
| san_render_native | 0.958x | 0.943x | 2.116x |
| apply_children_full | 0.958x | 0.856x | 1.293x |
| make_unmake_full | 0.990x | 0.929x | 1.295x |
| startpos | 0.998x | 0.998x | 1.282x |
| chess960_materialize | 0.982x | 0.982x | 1.556x |
| chess960_perft_d3 | 1.018x | 1.006x | 1.521x |
| polyglot_key | 0.914x | 0.835x | 39.584x |
| polyglot_recompute | 1.623x | 1.303x | 1.611x |
| repetition_key | 1.004x | 0.988x | 24.973x |
| corpus_count | 1.134x | 1.103x | 1.340x |
| corpus_materialize | 1.007x | 1.007x | 1.152x |
| corpus_uci_sorted | 0.984x | 0.982x | 1.528x |
| replay_trusted_native | 0.957x | 0.921x | 0.856x |
| replay_checked_native | 0.998x | 0.995x | 1.187x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.98x).
